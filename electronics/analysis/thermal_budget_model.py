#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
thermal_budget_model.py — Pre-order steady-state thermal budget for the OpenCanopy grow light.

Implements the thermal half of the §23 DR-01 pre-order modeling gate (work item WI-EE-10).
It is a first-order *lumped* model whose purpose is to catch an unbuildable thermal design BEFORE
the LED/driver is ordered and CAD is frozen — not to replace the post-build measurement
(validation/thermal/, WI-QA-03).

Model chain (per LED power point P_elec):

    P_heat   = (1 - eta_rad) * P_elec          # heat that must leave the LED junction
    T_hs     = T_amb + P_heat * Rth_hs_amb      # heatsink base temperature (forced air)
    T_j      = T_hs  + P_heat * Rth_j_hs         # LED junction temperature
    dT_canopy= f_canopy * P_heat / (rho*cp*Vdot) # canopy-air rise from the fraction reaching it
    T_bay    = T_amb + P_bay / (rho*cp*Vdot_bay)  # upper dry-bay air (driver loss + MCU)

It also inverts the chain to report, for each power point, the *required* heatsink-to-ambient
thermal resistance to hit a target heatsink temperature, and the *minimum* fan airflow to hold a
canopy-air and bay-air rise budget.

Run:  python3 electronics/analysis/thermal_budget_model.py
Exit code 0 = all power points within spec at their target fan point; 1 = a point is out of spec.
"""
from __future__ import annotations

import sys
from dataclasses import dataclass

# ----------------------------------------------------------------------------- model parameters
# All assumptions are explicit and sourced in WI-EE-10-thermal-budget-model.md.

T_AMB = 25.0          # worst-case room ambient, deg C (spec §17.2 "room 22-25 C" -> take 25)
ETA_RAD = 0.42        # LED radiant (PAR) efficiency; PPE ~2.5 umol/J white hort LED (spec §7.2)
RTH_J_HS = 0.5        # aggregate junction->heatsink-base incl. TIM, deg C/W (distributed bar)
RTH_HS_AMB = 0.55     # candidate forced-air heatsink+fan at design airflow, deg C/W

T_HS_TARGET = 60.0    # design heatsink target; <60 = "normal" band in spec §9.5 LED-temp table
T_J_LIMIT = 105.0     # LED junction max (typ. mid/high-power white LED); design margin to 85 C
T_J_DESIGN = 85.0     # preferred junction ceiling for lifetime (Arrhenius derate headroom)

# Canopy / bay air model
RHO_AIR = 1.16        # kg/m^3 at ~25 C
CP_AIR = 1005.0       # J/(kg.K)
F_CANOPY = 0.25       # fraction of LED heat that ends up warming canopy-zone air (open frame)
CANOPY_DT_BUDGET = 5.0  # deg C; keeps canopy <=30 C -> at/under spec §9.5 fan-ramp threshold
BAY_DT_BUDGET = 25.0    # deg C; keeps bay <=50 C for RTC/MCU/caps (spec §16.1, DR-05)

# Upper dry-bay heat: LED driver loss + MCU/sensors (spec §7.8). Driver eff ~0.90.
DRIVER_EFF = 0.90
P_MCU_SENSORS = 3.0   # W, typical (spec §7.8 MCU/sensors <2 W typ, 5 W peak)

CFM_TO_M3S = 0.000471947  # 1 CFM = 4.71947e-4 m^3/s


@dataclass
class Result:
    p_elec: float
    p_heat: float
    t_hs: float
    t_j: float
    rth_hs_amb_req: float   # required heatsink->ambient Rth to hit T_HS_TARGET
    p_driver_loss: float
    min_cfm_canopy: float   # min airflow to hold canopy dT budget
    min_cfm_bay: float      # min airflow to hold bay dT budget
    ok: bool
    notes: list[str]


def air_dt(power_w: float, cfm: float) -> float:
    """Temperature rise (K) of an air stream of `cfm` carrying `power_w` of heat."""
    vdot = cfm * CFM_TO_M3S
    return power_w / (RHO_AIR * CP_AIR * vdot)


def min_cfm_for_dt(power_w: float, dt_budget: float) -> float:
    """Minimum airflow (CFM) so that `power_w` raises the stream by no more than `dt_budget`."""
    vdot = power_w / (RHO_AIR * CP_AIR * dt_budget)
    return vdot / CFM_TO_M3S


def evaluate(p_elec: float) -> Result:
    p_heat = (1.0 - ETA_RAD) * p_elec
    t_hs = T_AMB + p_heat * RTH_HS_AMB
    t_j = t_hs + p_heat * RTH_J_HS
    rth_req = (T_HS_TARGET - T_AMB) / p_heat
    p_driver_loss = p_elec * (1.0 - DRIVER_EFF)
    p_bay = p_driver_loss + P_MCU_SENSORS
    min_cfm_canopy = min_cfm_for_dt(F_CANOPY * p_heat, CANOPY_DT_BUDGET)
    min_cfm_bay = min_cfm_for_dt(p_bay, BAY_DT_BUDGET)

    notes: list[str] = []
    ok = True
    if t_j > T_J_LIMIT:
        ok = False
        notes.append(f"junction {t_j:.0f}C EXCEEDS hard limit {T_J_LIMIT:.0f}C")
    elif t_j > T_J_DESIGN:
        notes.append(f"junction {t_j:.0f}C over preferred {T_J_DESIGN:.0f}C ceiling (lifetime margin tight)")
    if t_hs > T_HS_TARGET + 0.5:
        notes.append(f"heatsink {t_hs:.0f}C over target {T_HS_TARGET:.0f}C at Rth={RTH_HS_AMB} C/W "
                     f"-> need <={rth_req:.2f} C/W")
    return Result(p_elec, p_heat, t_hs, t_j, rth_req, p_driver_loss,
                  min_cfm_canopy, min_cfm_bay, ok, notes)


def main() -> int:
    points = [50.0, 80.0, 100.0]
    print("OpenCanopy pre-order thermal budget (WI-EE-10) — lumped first-order model")
    print(f"  ambient={T_AMB}C  eta_rad={ETA_RAD}  Rth(j-hs)={RTH_J_HS}  Rth(hs-a)={RTH_HS_AMB} C/W")
    print(f"  heatsink target={T_HS_TARGET}C  junction design ceiling={T_J_DESIGN}C limit={T_J_LIMIT}C")
    print()
    header = ("P_elec  P_heat  T_hs   T_j    Rth(hs-a)req  drv_loss  minCFM(canopy)  minCFM(bay)")
    print(header)
    print("-" * len(header))
    all_ok = True
    results = []
    for p in points:
        r = evaluate(p)
        results.append(r)
        all_ok = all_ok and r.ok
        print(f"{r.p_elec:5.0f}W {r.p_heat:6.1f}W {r.t_hs:5.0f}C {r.t_j:5.0f}C "
              f"{r.rth_hs_amb_req:11.2f}   {r.p_driver_loss:6.1f}W  {r.min_cfm_canopy:13.1f}  {r.min_cfm_bay:11.1f}")
    print()
    for r in results:
        for n in r.notes:
            print(f"  [{r.p_elec:.0f}W] {n}")
    print()
    print("RESULT:", "PASS (all points within spec at design airflow)" if all_ok
          else "REVIEW (a point needs mitigation — see notes / WI-EE-10 doc)")
    return 0 if all_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
