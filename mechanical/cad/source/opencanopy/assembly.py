# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
assembly.py — Full open-frame vertical-stack assembly (spec §8.1, §6.2; WI-ME-01).

Composes every module into one assembly in the locked 480 x 320 x 700 mm envelope
and provides the checks behind the §12.1 CAD verification:
  * bounding box vs the locked envelope (and hard maxima)
  * a coarse center-of-gravity model with a FULL reservoir + a mature plant, to
    confirm the unit is bottom-heavy (stable) — the heavy water + media + pot all
    sit low, electronics sit high but are light.

The "water fails downward / electronics live upward" rule (§6.2) is structural:
the only water-carrying parts (reservoir, pump, leak tray, pot tray) are at the
bottom; the electronics bay is the topmost module. assert_zone_separation() proves
no electronics part overlaps the wet-zone Z band.
"""

from build123d import Compound, CenterOf

from . import params as P
from . import frame, pot_reservoir as pr, electronics_bay as eb, wet_bay as wb
from . import light_mount as lm, fan_mount as fan, routing as rt


def _placed():
    """Return list of (label, placed_solid, zone) for the whole unit."""
    items = []
    # structure
    items.append(("frame", frame.build_frame(), "structure"))
    # wet zone (bottom)
    items.append(("leak-tray", wb.place_leak_tray(wb.build_leak_tray()), "wet"))
    items.append(("reservoir", pr.place_reservoir(pr.build_reservoir()), "wet"))
    items.append(("pump-clip", wb.place_pump_clip(wb.build_pump_clip()), "wet"))
    items.append(("pot-tray", wb.place_pot_tray(wb.build_pot_tray()), "wet"))
    items.append(("pot", pr.place_pot(pr.build_pot()), "wet"))
    # grow zone (middle)
    items.append(("fan-mount", fan.place_fan_mount(fan.build_fan_mount()), "grow"))
    items.append(("cable-channel", rt.place_cable_channel(rt.build_cable_channel()), "grow"))
    items.append(("status-diffuser", rt.place_status_diffuser(rt.build_status_diffuser()), "grow"))
    items.append(("led-fixture", lm.place_led_fixture(lm.build_led_fixture()), "grow"))
    items.append(("light-mount", lm.place_light_mount(lm.build_light_mount()), "grow"))
    # dry zone (top)
    items.append(("electronics-dry-bay", eb.place_dry_bay(eb.build_dry_bay()), "dry"))
    items.append(("electronics-dry-bay-lid", eb.place_dry_bay_lid(eb.build_dry_bay_lid()), "dry"))
    return items


def build_assembly():
    children = [solid for (_lbl, solid, _z) in _placed()]
    asm = Compound(children=children)
    asm.label = "opencanopy-v1"
    return asm


# --------------------------------------------------------------------------- #
# §12.1 CAD verification helpers
# --------------------------------------------------------------------------- #
def envelope_report():
    asm = build_assembly()
    bb = asm.bounding_box()
    sz = bb.size
    eps = 0.5  # mesh/rounding tolerance
    # must fit the LOCKED compact build dims (480 x 320 x 700), not merely the hard max
    assert sz.X <= P.ENV_W + eps, f"width {sz.X:.1f} exceeds locked {P.ENV_W}"
    assert sz.Y <= P.ENV_D + eps, f"depth {sz.Y:.1f} exceeds locked {P.ENV_D}"
    assert sz.Z <= P.ENV_H + eps, f"height {sz.Z:.1f} exceeds locked {P.ENV_H}"
    P.assert_within_envelope(sz.X, sz.Y, sz.Z)  # secondary: hard maxima
    return sz


# Coarse mass model (kg) for the worst-case stability check (full water + plant).
MASS_MODEL = {
    "water_full":   (4.0, "reservoir"),     # 4 L reservoir full
    "pot_media":   (9.0, "pot"),            # 10 L wet media
    "plant":        (1.5, "pot"),            # mature plant + support
    "led+mount":    (1.2, "led-fixture"),    # fixture head (driver is in dry bay, low-ish)
    "electronics":  (0.8, "electronics-dry-bay"),
    "frame+printed": (4.0, "frame"),
}


def cg_report():
    """Volume/centroid lookup per part, then mass-weighted CG with full load."""
    placed = {lbl: solid for (lbl, solid, _z) in _placed()}
    centroids = {lbl: solid.center(CenterOf.BOUNDING_BOX) for lbl, solid in placed.items()}
    total_m = 0.0
    mz = 0.0
    for _name, (m, ref) in MASS_MODEL.items():
        c = centroids.get(ref) or centroids["frame"]
        total_m += m
        mz += m * c.Z
    cg_z = mz / total_m
    return {"cg_z": cg_z, "total_kg": total_m, "frac_h": cg_z / P.ENV_H}


def assert_zone_separation():
    """No electronics part may intrude into the wet-zone Z band (§6.2)."""
    wet_top = P.POT_DECK_Z          # everything below the pot deck is the wet zone
    for (lbl, solid, zone) in _placed():
        if zone == "dry":
            zmin = solid.bounding_box().min.Z
            assert zmin > wet_top, f"{lbl} dips into wet zone (z={zmin} <= {wet_top})"
    return True


if __name__ == "__main__":
    sz = envelope_report()
    print(f"assembly bbox: {sz.X:.0f} x {sz.Y:.0f} x {sz.Z:.0f} mm "
          f"(envelope {P.ENV_W:.0f} x {P.ENV_D:.0f} x {P.ENV_H:.0f})")
    assert_zone_separation()
    print("zone separation: OK (no dry part in wet band)")
    cg = cg_report()
    print(f"CG height (full reservoir + plant): {cg['cg_z']:.0f} mm "
          f"= {cg['frac_h']*100:.0f}% of H, total ~{cg['total_kg']:.1f} kg")
