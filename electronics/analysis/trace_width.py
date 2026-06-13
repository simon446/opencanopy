#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
trace_width.py — IPC-2221 trace-width and voltage-drop calculator for the OpenCanopy controller.

Deliverable for WI-EE-06 (§11.3): minimum trace width per power path and a predicted voltage drop.
This is the design calculation; the *measured* voltage drop and component temperatures are filled in
at board bring-up (WI-EE-08) into electronics/test/pcb-verification.md.

IPC-2221 cross-section for a given current and allowed temperature rise:

    A_mils2 = (I / (k * dT**0.44)) ** (1 / 0.725)      # cross-sectional area, mil^2
    width_mil = A_mils2 / (thickness_oz * 1.378)        # 1 oz Cu ~= 1.378 mil thick

k = 0.048 for external layers, 0.024 for internal layers.

Run:  python3 electronics/analysis/trace_width.py
"""
from __future__ import annotations

from dataclasses import dataclass

RHO_CU = 1.724e-8        # copper resistivity, ohm.m (at ~20 C)
OZ_TO_M = 34.79e-6       # 1 oz/ft^2 copper thickness in metres (~34.8 um)
MIL_TO_MM = 0.0254


@dataclass
class Path:
    name: str
    current_a: float
    length_mm: float       # representative routed length for voltage-drop prediction
    layer: str = "external"


# Worst-case currents from WI-EE-02 power budget; lengths are representative routed estimates.
PATHS = [
    Path("24V input -> regulators/LED", 4.20, 60.0),
    Path("LED driver feed (24V)", 4.20, 40.0),
    Path("Pump MOSFET drain/source (24V)", 0.63, 30.0),
    Path("12V rail (fan + pump option)", 1.70, 50.0),
    Path("5V rail (sensors/status/3V3)", 0.60, 50.0),
    Path("3V3 rail (MCU)", 0.60, 40.0),
]

DT_RISE = 10.0           # allowed conductor temperature rise, deg C (conservative)
THICKNESS_OZ = 1.0       # finished outer-layer copper


def min_width_mm(current_a: float, dt: float, oz: float, layer: str) -> float:
    k = 0.048 if layer == "external" else 0.024
    area_mils2 = (current_a / (k * dt ** 0.44)) ** (1.0 / 0.725)
    width_mil = area_mils2 / (oz * 1.378)
    return width_mil * MIL_TO_MM


def vdrop_mv(current_a: float, length_mm: float, width_mm: float, oz: float) -> float:
    t_m = oz * OZ_TO_M
    area_m2 = (width_mm * 1e-3) * t_m
    r = RHO_CU * (length_mm * 1e-3) / area_m2
    return current_a * r * 1000.0  # mV (one-way; double for round trip)


def main() -> int:
    print("OpenCanopy trace-width & voltage-drop calc (WI-EE-06, IPC-2221)")
    print(f"  dT_rise={DT_RISE} C   copper={THICKNESS_OZ} oz   external layers\n")
    hdr = "path                                  I(A)   min_w(mm)  used_w(mm)  Vdrop_1way(mV)"
    print(hdr); print("-" * len(hdr))
    # Use a sensible manufacturable width = max(min_width rounded up, a floor), with margin.
    for p in PATHS:
        w_min = min_width_mm(p.current_a, DT_RISE, THICKNESS_OZ, p.layer)
        # designer-chosen width: >=1.5x min, floored to easy fab values
        used = max(round(w_min * 1.5, 1), 0.3)
        vd = vdrop_mv(p.current_a, p.length_mm, used, THICKNESS_OZ)
        print(f"{p.name:36s} {p.current_a:5.2f}  {w_min:8.2f}   {used:8.1f}    {vd:10.1f}")
    print("\nNotes: used_w carries >=1.5x margin over IPC minimum; high-current 24V/LED paths are")
    print("filled copper pours in practice (>= used_w). Measured Vdrop is recorded at bring-up.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
