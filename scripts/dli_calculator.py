#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""
dli_calculator.py — Convert between DLI, PPFD, and photoperiod for OpenCanopy V1.

DLI (Daily Light Integral) and PPFD are the engineering quantities for grow lighting — never lumens
(spec §2.1, source R1). This calculator is the Plant Science source of truth for the conversions the
light fixture sizing (WI-EE-01), the LED dim-map calibration (WI-EE-08), and the photometric model
(WI-PL-06) all rely on. See docs/dli-targets.md for the per-stage targets these conversions feed.

Core relationship (spec §5.2):

    DLI [mol·m⁻²·day⁻¹] = PPFD [µmol·m⁻²·s⁻¹] × photoperiod_hours × 0.0036
    PPFD               = DLI / (photoperiod_hours × 0.0036)

    The constant 0.0036 = 3600 s/h × 1e-6 mol/µmol (µmol·s⁻¹ integrated over an hour → mol).

Sub-commands:

    dli_calculator.py dli      --ppfd P --hours H        PPFD + photoperiod  -> DLI
    dli_calculator.py ppfd     --dli D  --hours H        DLI  + photoperiod  -> required avg PPFD
    dli_calculator.py fixture  --ppf F  --area A         fixture PPF + canopy area -> delivered PPFD
                               [--capture C] [--hours H]   (and the DLI it yields at H hours)
    dli_calculator.py table                              print the per-stage DLI/PPFD target table
    dli_calculator.py --selftest                         run built-in checks (used by CI)

Exit code 0 = success / all self-tests pass, 1 = failure.
"""
from __future__ import annotations

import argparse
import sys

# spec §5.2: DLI = PPFD × hours × 3600 s × 1e-6 mol/µmol
SECONDS_PER_HOUR_IN_MOL = 0.0036

# Per-stage targets, transcribed from docs/plant-profile-hot-pepper.md / spec §5.2.
# (stage, dli_lo, dli_hi, photoperiod_h, ppfd_lo, ppfd_hi). Germination omitted: light is optional/low
# until emergence, after which the seedling regime applies.
STAGE_TARGETS = [
    ("Seedling", 8, 12, 16, 140, 210),
    ("Vegetative", 14, 20, 16, 245, 350),
    ("Flowering", 18, 24, 16, 315, 420),
    ("Fruiting", 20, 25, 16, 350, 435),
]


def dli_from_ppfd(ppfd: float, hours: float) -> float:
    """DLI [mol·m⁻²·day⁻¹] from PPFD [µmol·m⁻²·s⁻¹] over a photoperiod of `hours`."""
    if ppfd < 0 or hours < 0:
        raise ValueError("ppfd and hours must be non-negative")
    return ppfd * hours * SECONDS_PER_HOUR_IN_MOL


def ppfd_from_dli(dli: float, hours: float) -> float:
    """Average PPFD [µmol·m⁻²·s⁻¹] required to hit `dli` over a photoperiod of `hours`."""
    if dli < 0:
        raise ValueError("dli must be non-negative")
    if hours <= 0:
        raise ValueError("hours must be positive to back out a PPFD")
    return dli / (hours * SECONDS_PER_HOUR_IN_MOL)


def delivered_ppfd(ppf: float, area_m2: float, capture: float = 1.0) -> float:
    """
    Estimate average canopy PPFD [µmol·m⁻²·s⁻¹] from a fixture's PPF [µmol/s] over a canopy area.

    `capture` is the fraction of fixture photons that actually land on the useful canopy (optics,
    mounting height, side losses, non-uniformity). capture=1.0 is the ideal upper bound used in the
    spec §7.2 worked example; real fixtures need capture < 1 and a proper photometric model
    (WI-PL-06) — this first-order estimate is only for sizing/sanity-checking.
    """
    if ppf < 0 or capture < 0:
        raise ValueError("ppf and capture must be non-negative")
    if area_m2 <= 0:
        raise ValueError("area_m2 must be positive")
    return ppf * capture / area_m2


def _fmt(x: float) -> str:
    return f"{x:.6g}"


def _print_table() -> None:
    print(f"{'Stage':<12}{'DLI band':>12}{'h':>5}{'PPFD band (table)':>22}{'PPFD from DLI':>20}")
    for stage, dlo, dhi, h, plo, phi in STAGE_TARGETS:
        calc_lo = ppfd_from_dli(dlo, h)
        calc_hi = ppfd_from_dli(dhi, h)
        print(
            f"{stage:<12}{f'{dlo}-{dhi}':>12}{h:>5}"
            f"{f'{plo}-{phi}':>22}{f'{calc_lo:.0f}-{calc_hi:.0f}':>20}"
        )


def _selftest() -> int:
    failures = []

    def check(name, got, want, tol):
        if abs(got - want) > tol:
            failures.append(f"{name}: got {got:.4f}, want {want:.4f} (±{tol})")

    # Spec §5.2 worked example: DLI 23, 16 h -> ~399 µmol·m⁻²·s⁻¹.
    check("worked-example ppfd", ppfd_from_dli(23, 16), 399.0, 0.5)
    # ...and its inverse round-trips.
    check("worked-example dli", dli_from_ppfd(ppfd_from_dli(23, 16), 16), 23.0, 1e-9)
    # Round-trip a few PPFD values.
    for ppfd in (140, 350, 435):
        check(f"roundtrip ppfd={ppfd}", ppfd_from_dli(dli_from_ppfd(ppfd, 16), 16), ppfd, 1e-9)

    # Per-stage table self-consistency: each PPFD band == DLI band run through the formula. The spec
    # §5.2 table rounds PPFD to tidy multiples of 5 µmol (e.g. exact 416.7 -> listed 420), so allow
    # ±5 µmol — enough to confirm consistency, tight enough to catch a transcription error.
    for stage, dlo, dhi, h, plo, phi in STAGE_TARGETS:
        check(f"{stage} ppfd-lo", ppfd_from_dli(dlo, h), plo, 5.0)
        check(f"{stage} ppfd-hi", ppfd_from_dli(dhi, h), phi, 5.0)

    # Fixture estimate: spec §7.2 — 450 µmol/m²/s across 0.10 m² needs 45 µmol/s delivered (ideal).
    check("fixture ideal ppfd", delivered_ppfd(45, 0.10, capture=1.0), 450.0, 1e-9)
    # A 150 µmol/s fixture at 70% capture over 0.10 m² -> 1050 µmol/m²/s raw (well above target,
    # i.e. it is dimmed in operation) — sanity bound only.
    check("fixture realistic", delivered_ppfd(150, 0.10, capture=0.70), 1050.0, 1e-9)

    if failures:
        print("SELFTEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print(f"SELFTEST PASSED ({len(STAGE_TARGETS)} stages + worked example + fixture checks)")
    return 0


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)

    parser = argparse.ArgumentParser(description="DLI / PPFD / photoperiod calculator (spec §5.2)")
    parser.add_argument("--selftest", action="store_true", help="run built-in checks and exit")
    sub = parser.add_subparsers(dest="cmd")

    p_dli = sub.add_parser("dli", help="PPFD + photoperiod -> DLI")
    p_dli.add_argument("--ppfd", type=float, required=True, help="µmol·m⁻²·s⁻¹")
    p_dli.add_argument("--hours", type=float, required=True, help="photoperiod hours")

    p_ppfd = sub.add_parser("ppfd", help="DLI + photoperiod -> required average PPFD")
    p_ppfd.add_argument("--dli", type=float, required=True, help="mol·m⁻²·day⁻¹")
    p_ppfd.add_argument("--hours", type=float, required=True, help="photoperiod hours")

    p_fix = sub.add_parser("fixture", help="fixture PPF + canopy area -> delivered PPFD")
    p_fix.add_argument("--ppf", type=float, required=True, help="fixture PPF, µmol/s")
    p_fix.add_argument("--area", type=float, required=True, help="canopy area, m²")
    p_fix.add_argument("--capture", type=float, default=1.0, help="fraction of PPF on canopy (default 1.0)")
    p_fix.add_argument("--hours", type=float, default=16.0, help="photoperiod hours (default 16)")

    sub.add_parser("table", help="print the per-stage DLI/PPFD target table")

    args = parser.parse_args(argv)

    if args.selftest:
        return _selftest()

    if args.cmd == "dli":
        print(f"DLI = {_fmt(dli_from_ppfd(args.ppfd, args.hours))} mol·m⁻²·day⁻¹")
        return 0
    if args.cmd == "ppfd":
        print(f"Required average PPFD = {_fmt(ppfd_from_dli(args.dli, args.hours))} µmol·m⁻²·s⁻¹")
        return 0
    if args.cmd == "fixture":
        ppfd = delivered_ppfd(args.ppf, args.area, args.capture)
        dli = dli_from_ppfd(ppfd, args.hours)
        print(f"Delivered avg PPFD = {_fmt(ppfd)} µmol·m⁻²·s⁻¹ (capture={args.capture})")
        print(f"  -> DLI at {args.hours} h = {_fmt(dli)} mol·m⁻²·day⁻¹")
        return 0
    if args.cmd == "table":
        _print_table()
        return 0

    parser.print_help()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
