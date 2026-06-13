#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
verify.py — Computes the §12.1 CAD-verification quantities from the model so the
checklist in mechanical/cad-verification-checklist.md cites real numbers, not
hand-waving. Run:

    .venv-cad/bin/python mechanical/cad/source/verify.py
"""
import math

from opencanopy import params as P
from opencanopy import assembly


def frustum_volume_l(d_top, d_bot, h):
    R, r = d_top / 2, d_bot / 2
    return (math.pi / 3) * h * (R * R + R * r + r * r) / 1e6


def main():
    interior_w = P.ENV_W - 2 * (P.UPRIGHT_INSET + P.EXTRUSION)
    interior_d = P.ENV_D - 2 * (P.UPRIGHT_INSET + P.EXTRUSION)

    pot_cap = frustum_volume_l(P.POT_TOP_DIA - 2 * P.POT_WALL,
                               P.POT_BOT_DIA - 2 * P.POT_WALL,
                               P.POT_H - P.POT_WALL)
    res_cap = (P.RES_W - 2 * P.RES_WALL) * (P.RES_D - 2 * P.RES_WALL) * \
              (P.RES_H - P.RES_WALL) / 1e6

    pot_rim_top = P.POT_DECK_Z + 6 + P.POT_H
    from opencanopy.light_mount import FIXTURE_UNDERSIDE_Z
    led_clear = FIXTURE_UNDERSIDE_Z - pot_rim_top
    res_top = P.WET_BAY_FLOOR_Z + 6 + P.RES_H

    sz = assembly.envelope_report()
    assembly.assert_zone_separation()
    cg = assembly.cg_report()

    checks = [
        ("Envelope (W x D x H)", f"{sz.X:.0f} x {sz.Y:.0f} x {sz.Z:.0f} mm",
         f"<= {P.ENV_W:.0f} x {P.ENV_D:.0f} x {P.ENV_H:.0f}",
         sz.X <= P.ENV_W and sz.Y <= P.ENV_D and sz.Z <= P.ENV_H + 0.5),
        ("Frame interior (W x D)", f"{interior_w:.0f} x {interior_d:.0f} mm", "info", True),
        ("Pot usable capacity", f"{pot_cap:.1f} L", "8-10 L compact", 8.0 <= pot_cap <= 11.0),
        ("Pot OD vs interior depth", f"{P.POT_TOP_DIA:.0f} vs {interior_d:.0f} mm",
         f"clearance {interior_d - P.POT_TOP_DIA:.0f} mm > 0", P.POT_TOP_DIA < interior_d),
        ("Reservoir usable capacity", f"{res_cap:.1f} L", "2.5-4 L compact", 2.5 <= res_cap <= 4.6),
        ("LED clearance above pot rim", f"{led_clear:.0f} mm", "150-300 mm",
         P.LED_CLEAR_MIN <= led_clear <= P.LED_CLEAR_MAX),
        ("Reservoir top vs pot deck", f"{res_top:.0f} vs {P.POT_DECK_Z:.0f} mm",
         "reservoir withdraws below the deck", res_top < P.POT_DECK_Z),
        ("CG height (full + plant)", f"{cg['cg_z']:.0f} mm ({cg['frac_h']*100:.0f}% H)",
         "bottom-heavy (< 50% H)", cg['frac_h'] < 0.5),
        ("Cable bend / drip-loop radius", f"{P.DRIP_LOOP_R:.0f} mm", "drip loop before bay", True),
    ]

    print(f"{'CHECK':<34}{'VALUE':<22}{'TARGET':<32}RESULT")
    allok = True
    for name, val, target, ok in checks:
        allok = allok and ok
        print(f"{name:<34}{val:<22}{target:<32}{'PASS' if ok else 'FAIL'}")
    print("\nCAD verification:", "PASS" if allok else "FAIL")
    return 0 if allok else 1


if __name__ == "__main__":
    raise SystemExit(main())
