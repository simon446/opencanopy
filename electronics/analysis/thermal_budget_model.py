#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
thermal_budget_model.py — Pre-order steady-state thermal budget for the OpenCanopy grow light.

Implements the thermal half of the §23 DR-01 pre-order modeling gate (work item WI-EE-10).
It is a first-order *lumped* model whose purpose is to catch an unbuildable thermal design BEFORE
the LED/driver is ordered and CAD is frozen — not to replace the post-build measurement
(validation/thermal/, WI-QA-03).

PASSIVE (no-fan) revision — ECO-001, 2026-06-14:
    V1 has **no fan** (the fan was a plant-canopy circulation actuator, deleted in the mechanical
    redesign; it is not added back). The earlier model leaned on that fan for *forced-air* heatsink
    cooling (Rth(hs-a)=0.55 C/W). With no fan the LED heatsink runs by **natural convection**, so its
    thermal resistance is several times higher and is the binding constraint. This model now sizes the
    LED against a **passive** heatsink and reports the LED-power ceiling that stays in spec fan-less.

    The two airflow duties the deleted fan also nominally served are small and pass without it:
      * Upper dry bay (driver loss + MCU ~8-13 W): needs <1 CFM-equivalent -> carried by open-frame
        buoyancy; the bay is open on the dry side. Confirmed below.
      * Canopy-air mixing: this was the fan's *actual* purpose (VPD/boundary-layer for the plant, not
        electronics). Its removal is a Plant-Science / firmware-climate question (VPD homogeneity),
        NOT an LED-thermal one — flagged, out of the electronics thermal scope.

Model chain (per LED power point P_elec):

    P_heat   = (1 - eta_rad) * P_elec          # heat that must leave the LED junction
    T_hs     = T_amb + P_heat * Rth_hs_amb      # heatsink base temperature (natural convection)
    T_j      = T_hs  + P_heat * Rth_j_hs         # LED junction temperature
    T_bay    = T_amb + P_bay / (rho*cp*Vdot_bay) # upper dry-bay air (driver loss + MCU)

It also inverts the chain to report, for each power point, the *required* heatsink-to-ambient
thermal resistance to hit a target heatsink temperature.

Run:  python3 electronics/analysis/thermal_budget_model.py
Exit code 0 = the committed V1 light passes fan-less at its target passive heatsink; 1 otherwise.
"""
from __future__ import annotations

import sys
from dataclasses import dataclass

# ----------------------------------------------------------------------------- model parameters
# All assumptions are explicit and sourced in WI-EE-10-thermal-budget-model.md.

T_AMB = 25.0          # worst-case room ambient, deg C (spec §17.2 "room 22-25 C" -> take 25)
ETA_RAD = 0.42        # LED radiant (PAR) efficiency; PPE ~2.5 umol/J white hort LED (spec §7.2)
RTH_J_HS = 0.5        # aggregate junction->heatsink-base incl. TIM, deg C/W (distributed bar)

# Natural-convection (no-fan) heatsink classes. Forced-air's 0.55 C/W is gone with the fan.
# These bracket what a passive finned heatsink realistically achieves at this dissipation, vertical
# fins, open frame. The large class is the WI-ME-05 light-mount design target for V1.
RTH_HS_AMB_PASSIVE = {
    "moderate passive (~1.2 C/W, large finned extrusion)": 1.2,
    "large passive (~0.8 C/W, dedicated LED cooler)": 0.8,
}

# The committed V1 grow light (BOM LED_PANEL) is a 60 W panel; the large passive heatsink is the
# mechanical design target. The build is GO fan-less iff this design point stays in spec.
V1_DESIGN_W = 60.0
V1_DESIGN_RTH = 0.8

T_HS_TARGET = 60.0    # design heatsink target; <60 = "normal" band in spec §9.5 LED-temp table
T_J_LIMIT = 105.0     # LED junction max (typ. mid/high-power white LED); design margin to 85 C
T_J_DESIGN = 85.0     # preferred junction ceiling for lifetime (Arrhenius derate headroom)

# Upper dry-bay air check (no fan -> open-frame natural convection).
RHO_AIR = 1.16        # kg/m^3 at ~25 C
CP_AIR = 1005.0       # J/(kg.K)
BAY_DT_BUDGET = 25.0    # deg C; keeps bay <=50 C for RTC/MCU/caps (spec §16.1, DR-05)
DRIVER_EFF = 0.90     # constant-current LED driver efficiency -> loss into the upper dry bay
P_MCU_SENSORS = 3.0   # W, typical (spec §7.8 MCU/sensors <2 W typ, 5 W peak)

CFM_TO_M3S = 0.000471947  # 1 CFM = 4.71947e-4 m^3/s

POWER_POINTS = [50.0, 60.0, 80.0, 100.0]


@dataclass
class Point:
    p_elec: float
    rth_hs_amb: float
    p_heat: float
    t_hs: float
    t_j: float
    rth_hs_amb_req: float   # required heatsink->ambient Rth to hit T_HS_TARGET
    hs_ok: bool             # heatsink base within the §9.5 "normal" band
    j_ok: bool              # junction within the preferred lifetime ceiling
    j_safe: bool            # junction below the hard max (survivable, lifetime aside)


def evaluate(p_elec: float, rth_hs_amb: float) -> Point:
    p_heat = (1.0 - ETA_RAD) * p_elec
    t_hs = T_AMB + p_heat * rth_hs_amb
    t_j = t_hs + p_heat * RTH_J_HS
    rth_req = (T_HS_TARGET - T_AMB) / p_heat
    return Point(
        p_elec, rth_hs_amb, p_heat, t_hs, t_j, rth_req,
        hs_ok=t_hs <= T_HS_TARGET + 0.5,
        j_ok=t_j <= T_J_DESIGN,
        j_safe=t_j <= T_J_LIMIT,
    )


def min_cfm_for_dt(power_w: float, dt_budget: float) -> float:
    """Equivalent airflow (CFM) so `power_w` raises a stream by no more than `dt_budget`.
    With no fan this is the through-flow the open-frame buoyancy must supply, not a fan spec."""
    vdot = power_w / (RHO_AIR * CP_AIR * dt_budget)
    return vdot / CFM_TO_M3S


def main() -> int:
    print("OpenCanopy pre-order thermal budget (WI-EE-10) — PASSIVE / no-fan revision (ECO-001)")
    print(f"  ambient={T_AMB}C  eta_rad={ETA_RAD}  Rth(j-hs)={RTH_J_HS} C/W")
    print(f"  heatsink target={T_HS_TARGET}C  junction design ceiling={T_J_DESIGN}C hard limit={T_J_LIMIT}C")
    print(f"  V1 design point: {V1_DESIGN_W:.0f} W LED on a {V1_DESIGN_RTH} C/W passive heatsink")
    print()

    for label, rth in RTH_HS_AMB_PASSIVE.items():
        print(f"Heatsink: {label}")
        header = "  P_elec  P_heat  T_hs    T_j    Rth(hs-a) needed   verdict"
        print(header)
        print("  " + "-" * (len(header) - 2))
        for p in POWER_POINTS:
            r = evaluate(p, rth)
            if not r.j_safe:
                verdict = "NO-GO (junction over hard max)"
            elif not r.j_ok or not r.hs_ok:
                verdict = "MARGINAL (over lifetime/normal band)"
            else:
                verdict = "GO"
            print(f"  {r.p_elec:5.0f}W {r.p_heat:6.1f}W {r.t_hs:5.0f}C {r.t_j:5.0f}C "
                  f"     <= {r.rth_hs_amb_req:.2f}        {verdict}")
        print()

    # Bay check (no fan): the dry bay carries driver loss + MCU; tiny, met by open-frame convection.
    print("Upper dry-bay (no fan -> open-frame natural convection):")
    for p in POWER_POINTS:
        p_bay = p * (1.0 - DRIVER_EFF) + P_MCU_SENSORS
        cfm = min_cfm_for_dt(p_bay, BAY_DT_BUDGET)
        print(f"  LED {p:5.0f}W -> bay heat {p_bay:4.1f}W needs only {cfm:.2f} CFM-equiv "
              f"to hold bay <= {T_AMB + BAY_DT_BUDGET:.0f}C")
    print()

    # Verdict: the committed V1 light must pass fan-less at the design heatsink.
    d = evaluate(V1_DESIGN_W, V1_DESIGN_RTH)
    v1_ok = d.hs_ok and d.j_ok
    print(f"V1 DESIGN POINT ({V1_DESIGN_W:.0f} W @ {V1_DESIGN_RTH} C/W passive): "
          f"T_hs={d.t_hs:.0f}C T_j={d.t_j:.0f}C -> "
          f"{'PASS (fan-less GO)' if v1_ok else 'FAIL'}")
    # 100 W is the full-yield variant: confirm it is NOT viable fan-less (documents the boundary).
    hot = evaluate(100.0, V1_DESIGN_RTH)
    print(f"100 W full-yield variant @ {V1_DESIGN_RTH} C/W passive: "
          f"T_hs={hot.t_hs:.0f}C T_j={hot.t_j:.0f}C -> "
          f"{'unexpectedly OK' if (hot.hs_ok and hot.j_ok) else 'NOT viable fan-less (needs active cooling / lower drive)'}")
    print()
    print("RESULT:", "PASS (committed V1 light is passively cooled, no fan required)" if v1_ok
          else "REVIEW (V1 light fails fan-less — bigger heatsink or lower drive needed)")
    return 0 if v1_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
