#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""
photometric_model.py — Pre-order PPFD-delivery model & DLI gate for OpenCanopy V1.

This is the photometric half of the spec §23 DR-01 pre-order modeling gate (WI-PL-06). Before any grow
light is PURCHASED, it predicts the *delivered* PPFD map, min/avg uniformity, and DLI for candidate
fixtures at the actual mounting geometry (clearance, canopy area, frame reflectance) — so the §7.2
targets are confirmed physically achievable in the locked envelope, not assumed from a fixture's
headline PPF spec sheet. The later physical PPFD survey (WI-EE-01, ppfd-measurements/) validates this
model against reality.

Model (first-order, deliberately conservative):

  * Each fixture is a set of point emitters (a bar = a line; a panel = a grid) sharing the fixture's
    total PPF. Each emitter is treated as a downward Lambertian source.
  * Direct PPFD at a canopy point from one emitter of flux phi at height h, horizontal offset r:
        I0 = phi / pi                      (Lambertian: total flux = pi * axial intensity)
        E  = I0 * cos^2(theta) / d^2
           = (phi/pi) * h^2 / (h^2 + r^2)^2     with d^2 = h^2 + r^2, cos(theta) = h/d
  * Frame/wall reflectance adds a diffuse fill term, modeled as a uniform additive contribution
    proportional to the mean direct PPFD: E_refl = wall_reflectance * mean(E_direct). Open-frame walls
    are sparse, so this term is small and is applied uniformly (reflections fill edges, so this is also
    slightly conservative on uniformity). Set wall_reflectance=0 for a pure direct-only bound.

Targets (spec §7.2 / WI-PL-06 acceptance): >=350 µmol/m²/s average at >=0.6 min/avg uniformity across
the canopy at >=150 mm clearance, hitting the WI-PL-02 fruiting DLI (20-25 mol/m²/day at 16 h).

Usage:
    photometric_model.py                 run the gate over all candidates; print report
    photometric_model.py --csv DIR       also write per-candidate PPFD-map CSVs into DIR
    photometric_model.py --selftest      run built-in physics/sanity checks (used by CI)

Exit code: 0 = report produced / self-tests pass; 1 = self-test failure.
"""
from __future__ import annotations

import argparse
import math
import sys

# --- Targets (spec §7.2, WI-PL-02) -------------------------------------------------------------
PPFD_AVG_TARGET = 350.0      # µmol/m²/s minimum average across canopy
UNIFORMITY_TARGET = 0.60     # min/avg
PHOTOPERIOD_H = 16.0
DLI_FRUIT_LO, DLI_FRUIT_HI = 20.0, 25.0   # mol/m²/day fruiting band
SEC_PER_HOUR_IN_MOL = 0.0036

# --- Locked envelope (spec §3.3 compact, §7.2) -------------------------------------------------
# Compact useful canopy: 300-400 mm W × 220-300 mm D. Use a mid-compact canopy for the gate.
CANOPY_W_M = 0.350
CANOPY_D_M = 0.260
GRID_NX = 15
GRID_NY = 11
CLEARANCE_MIN_M = 0.150      # spec §7.2 minimum light-to-canopy clearance


class Fixture:
    """A candidate grow-light fixture as a set of co-planar downward point emitters."""

    def __init__(self, name, ppf_umol_s, kind, span_w_m, span_d_m, n_w, n_d, ppe=None, note=""):
        self.name = name
        self.ppf = ppf_umol_s
        self.kind = kind          # "bar" or "panel"
        self.span_w = span_w_m
        self.span_d = span_d_m
        self.n_w = n_w
        self.n_d = n_d
        self.ppe = ppe            # µmol/J, for the §7.8 power/thermal cross-check
        self.note = note

    def emitters(self):
        """Yield (x, y) emitter positions centered on the canopy, sharing total PPF equally."""
        n = self.n_w * self.n_d
        phi = self.ppf / n
        xs = _linspace(-self.span_w / 2, self.span_w / 2, self.n_w)
        ys = _linspace(-self.span_d / 2, self.span_d / 2, self.n_d)
        for x in xs:
            for y in ys:
                yield x, y, phi

    @property
    def power_w(self):
        return None if not self.ppe else self.ppf / self.ppe


def _linspace(a, b, n):
    if n == 1:
        return [(a + b) / 2]
    step = (b - a) / (n - 1)
    return [a + i * step for i in range(n)]


def direct_ppfd_at(fx: Fixture, px, py, h):
    """Sum direct Lambertian inverse-square PPFD at canopy point (px,py) from all emitters."""
    total = 0.0
    for ex, ey, phi in fx.emitters():
        r2 = (px - ex) ** 2 + (py - ey) ** 2
        d2 = h * h + r2
        # E = (phi/pi) * h^2 / d^4
        total += (phi / math.pi) * (h * h) / (d2 * d2)
    return total


def ppfd_map(fx: Fixture, h, wall_reflectance=0.0):
    """Return (grid, avg, mn, mx, uniformity) for the canopy at clearance h."""
    xs = _linspace(-CANOPY_W_M / 2, CANOPY_W_M / 2, GRID_NX)
    ys = _linspace(-CANOPY_D_M / 2, CANOPY_D_M / 2, GRID_NY)
    grid = [[direct_ppfd_at(fx, x, y, h) for x in xs] for y in ys]
    flat = [v for row in grid for v in row]
    mean_direct = sum(flat) / len(flat)
    if wall_reflectance:
        fill = wall_reflectance * mean_direct
        grid = [[v + fill for v in row] for row in grid]
        flat = [v + fill for v in flat]
    avg = sum(flat) / len(flat)
    mn, mx = min(flat), max(flat)
    uniformity = mn / avg if avg else 0.0
    return grid, avg, mn, mx, uniformity


def dli_from_ppfd(ppfd, hours=PHOTOPERIOD_H):
    return ppfd * hours * SEC_PER_HOUR_IN_MOL


# --- Candidate fixtures (framed by spec §7.2 ranges) -------------------------------------------
# Bars span slightly wider than the canopy to lift edge uniformity. Panels cover the canopy footprint.
CANDIDATES = [
    Fixture("A: compact-min bar (100 µmol/s)", 100, "bar", 0.400, 0.0, 24, 1,
            ppe=2.2, note="spec §7.2 absolute minimum; expected to fail uniformity/avg"),
    Fixture("B: compact-preferred bar (150 µmol/s)", 150, "bar", 0.400, 0.0, 24, 1,
            ppe=2.5, note="mid of §7.2 preferred 140-220 µmol/s compact range"),
    Fixture("C: compact-preferred panel (150 µmol/s)", 150, "panel", 0.360, 0.240, 8, 6,
            ppe=2.5, note="same PPF as B but 2D panel — better uniformity"),
    Fixture("D: full-yield bar (220 µmol/s)", 220, "bar", 0.420, 0.0, 28, 1,
            ppe=2.5, note="top of compact range / full-yield headroom"),
]

# Open-frame white-ish frame: modest reflectance fill. 0.0 would be the pure direct lower bound.
WALL_REFLECTANCE = 0.10


def evaluate(fx: Fixture, h, wall_reflectance=WALL_REFLECTANCE):
    grid, avg, mn, mx, uni = ppfd_map(fx, h, wall_reflectance)
    dli = dli_from_ppfd(avg)
    passes = (avg >= PPFD_AVG_TARGET and uni >= UNIFORMITY_TARGET
              and DLI_FRUIT_LO <= dli)  # DLI must reach fruiting floor; upper is a dim-down ceiling
    return dict(grid=grid, avg=avg, mn=mn, mx=mx, uni=uni, dli=dli, passes=passes)


def _report(write_csv_dir=None):
    print("OpenCanopy V1 — Pre-order photometric model & DLI gate (WI-PL-06 / §23 DR-01)")
    print(f"Canopy {CANOPY_W_M*1000:.0f}×{CANOPY_D_M*1000:.0f} mm ({CANOPY_W_M*CANOPY_D_M:.3f} m²), "
          f"grid {GRID_NX}×{GRID_NY}, wall reflectance {WALL_REFLECTANCE:.0%}")
    print(f"Targets: avg >= {PPFD_AVG_TARGET:.0f} µmol/m²/s, uniformity >= {UNIFORMITY_TARGET:.2f}, "
          f"DLI >= {DLI_FRUIT_LO:.0f} mol/m²/day @ {PHOTOPERIOD_H:.0f} h\n")

    header = f"{'Candidate':<40}{'clr(mm)':>8}{'avg':>8}{'min':>8}{'unif':>7}{'DLI':>7}{'pass':>6}"
    any_pass_at_min = False
    for fx in CANDIDATES:
        print(f"# {fx.name}   PPF={fx.ppf:.0f} µmol/s"
              + (f", ~{fx.power_w:.0f} W @ {fx.ppe} µmol/J" if fx.power_w else ""))
        print(header)
        # Sensitivity sweep over clearance 150-250 mm (canopy rises toward the light over the cycle).
        for clr_mm in (150, 175, 200, 225, 250):
            h = clr_mm / 1000.0
            r = evaluate(fx, h)
            mark = "PASS" if r["passes"] else "fail"
            print(f"{'':<40}{clr_mm:>8}{r['avg']:>8.0f}{r['mn']:>8.0f}"
                  f"{r['uni']:>7.2f}{r['dli']:>7.1f}{mark:>6}")
            if clr_mm == 150 and r["passes"]:
                any_pass_at_min = True
            if write_csv_dir and clr_mm == 150:
                _write_csv(write_csv_dir, fx, h, r["grid"])
        print(f"  note: {fx.note}\n")

    print("GATE RESULT:",
          "PASS — >=1 candidate meets all targets at 150 mm clearance."
          if any_pass_at_min else
          "NO-GO — no candidate meets targets at 150 mm; do not order. See README.")
    return 0


def _write_csv(dirpath, fx, h, grid):
    import os
    safe = fx.name.split(":")[0].strip().lower()
    path = os.path.join(dirpath, f"ppfd-map-{safe}-{int(h*1000)}mm.csv")
    xs = _linspace(-CANOPY_W_M / 2, CANOPY_W_M / 2, GRID_NX)
    with open(path, "w") as fh:
        fh.write("y_mm\\x_mm," + ",".join(f"{x*1000:.0f}" for x in xs) + "\n")
        ys = _linspace(-CANOPY_D_M / 2, CANOPY_D_M / 2, GRID_NY)
        for y, row in zip(ys, grid):
            fh.write(f"{y*1000:.0f}," + ",".join(f"{v:.0f}" for v in row) + "\n")
    print(f"  wrote {path}")


def _selftest():
    failures = []

    def check(name, got, want, tol):
        if abs(got - want) > tol:
            failures.append(f"{name}: got {got:.4f}, want {want:.4f} (±{tol})")

    # Single emitter directly overhead: E = (phi/pi)/h^2.
    one = Fixture("one", 100.0, "bar", 0.0, 0.0, 1, 1)
    h = 0.150
    got = direct_ppfd_at(one, 0.0, 0.0, h)
    want = (100.0 / math.pi) / (h * h)
    check("single-emitter nadir", got, want, 1e-6)

    # Inverse-square: doubling height -> ~1/4 PPFD for a single nadir emitter.
    half = direct_ppfd_at(one, 0.0, 0.0, 2 * h)
    check("inverse-square scaling", half * 4.0, got, 1e-6)

    # Off-axis point must be dimmer than nadir.
    if not direct_ppfd_at(one, 0.20, 0.0, h) < got:
        failures.append("off-axis not dimmer than nadir")

    # DLI conversion matches the WI-PL-02 calculator (399 at PPFD from DLI 23).
    check("dli@399", dli_from_ppfd(399.306), 23.0, 0.01)

    # Energy sanity: total direct flux landing on an infinite plane from one Lambertian emitter
    # approaches phi as the plane grows (here just check a wide coarse integral is < phi and > 0).
    wide = Fixture("w", 100.0, "bar", 0.0, 0.0, 1, 1)
    s = 0.0
    span, npts = 4.0, 81           # 4 m square, coarse
    cell = (span / (npts - 1)) ** 2
    grid_pts = _linspace(-span / 2, span / 2, npts)
    for x in grid_pts:
        for y in grid_pts:
            s += direct_ppfd_at(wide, x, y, 0.15) * cell
    if not (0.5 * 100.0 < s < 1.01 * 100.0):
        failures.append(f"flux integral implausible: {s:.1f} (expected ~ up to 100)")

    if failures:
        print("SELFTEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("SELFTEST PASSED (physics + DLI + flux-conservation checks)")
    return 0


def main(argv=None):
    p = argparse.ArgumentParser(description="Pre-order PPFD/DLI photometric gate (WI-PL-06)")
    p.add_argument("--selftest", action="store_true", help="run built-in checks and exit")
    p.add_argument("--csv", metavar="DIR", help="write per-candidate PPFD-map CSVs at 150 mm into DIR")
    args = p.parse_args(argv)
    if args.selftest:
        return _selftest()
    return _report(write_csv_dir=args.csv)


if __name__ == "__main__":
    raise SystemExit(main())
