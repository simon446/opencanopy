# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
wet_bay.py — Bottom wet bay + pot tray (spec §6.2, §7.7, §8.4, §12.3; WI-ME-04).

The entire water system lives here, below and isolated from the electronics:
  * leak tray under everything, with a sensor sump + leak-sensor boss (pump lockout)
  * reservoir cradle rails for tool-free drawer removal (CLEAR_SLIDE fit)
  * rubber-isolated pump retention clip
  * controlled front overflow weir so spills run to the FRONT, never toward the
    rear/upper electronics (§7.7)
  * pot tray at the pot deck: locates the pot rim and drains overflow straight
    down into the leak tray via a downspout (water fails downward, §6.2)

Local frames: each part built on z=0; assembly placement helpers at the bottom.
"""

from build123d import Box, BuildPart, Cylinder, Locations, Mode, Pos, PolarLocations

from . import params as P
from .util import CX, CENTER_MIN

TRAY_W = 440.0
TRAY_D = 300.0
TRAY_WALL = 3.0
TRAY_RIM = 24.0           # perimeter rim height (catches splashes)
WEIR_DROP = 8.0          # front weir is this much lower -> overflow runs forward
RAIL_H = 6.0
RAIL_W = 8.0
SUMP = 30.0              # leak-sensor sump pocket side


def build_leak_tray():
    with BuildPart() as tray:
        # shallow open tray
        Box(TRAY_W, TRAY_D, TRAY_RIM, align=CENTER_MIN)
        with Locations((0, 0, TRAY_WALL)):
            Box(TRAY_W - 2 * TRAY_WALL, TRAY_D - 2 * TRAY_WALL, TRAY_RIM,
                align=CENTER_MIN, mode=Mode.SUBTRACT)
        # front overflow weir: notch the FRONT (-Y) wall lower than the rest
        with Locations((0, -TRAY_D / 2, TRAY_RIM - WEIR_DROP + TRAY_RIM / 2)):
            Box(120, TRAY_WALL * 3, TRAY_RIM, mode=Mode.SUBTRACT)
        # reservoir cradle rails on the floor (drawer slides front-to-back)
        for sx in (-1, 1):
            with Locations((sx * (P.RES_W / 2 - RAIL_W), 20, TRAY_WALL)):
                Box(RAIL_W, TRAY_D - 80, RAIL_H, align=CENTER_MIN)
        # leak-sensor sump pocket at the front-left low corner + boss
        with Locations((-TRAY_W / 2 + SUMP, -TRAY_D / 2 + SUMP, 0)):
            Box(SUMP, SUMP, TRAY_WALL * 2, align=CENTER_MIN)  # local thickening
            Cylinder(radius=4, height=TRAY_RIM, align=CENTER_MIN)   # sensor post
            Cylinder(radius=2, height=TRAY_RIM + 1, align=CENTER_MIN, mode=Mode.SUBTRACT)
    tray.part.label = "leak-tray"
    return tray.part


def build_pump_clip():
    """Rubber-isolated pump retention clip (spec §7.4 mount, §8.7 acoustics).

    A back plate fixes to the wet-bay rear rail; a C-cradle (open front) wraps the
    pump body on a silicone pad so the pump lifts out tool-free and vibration is
    decoupled from the frame."""
    plate_w = P.PUMP_DIA + 20
    cradle_y = -(P.PUMP_DIA / 2 + 12)      # cradle centre, forward of the plate
    with BuildPart() as clip:
        # back plate
        Box(plate_w, 6, P.PUMP_H * 0.8, align=CENTER_MIN)
        # C-cradle ring (4 mm wall), overlapping back into the plate so it fuses
        with Locations((0, cradle_y, P.PUMP_H * 0.25)):
            Cylinder(radius=P.PUMP_DIA / 2 + P.PUMP_ISO_PAD + 4, height=14, align=CENTER_MIN)
        with Locations((0, cradle_y, P.PUMP_H * 0.25 - 1)):
            Cylinder(radius=P.PUMP_DIA / 2 + P.PUMP_ISO_PAD, height=16,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
        # neck linking cradle to plate (guarantees a solid bridge)
        with Locations((0, cradle_y / 2, P.PUMP_H * 0.25 + 7)):
            Box(P.PUMP_DIA * 0.5, abs(cradle_y) + 6, 14)
        # open the FRONT of the C (toward -Y) so the pump lifts straight out
        with Locations((0, cradle_y - P.PUMP_DIA, P.PUMP_H * 0.25 + 7)):
            Box(P.PUMP_DIA * 0.8, P.PUMP_DIA, 20, mode=Mode.SUBTRACT)
    clip.part.label = "pump-clip"
    return clip.part


def build_pot_tray():
    """Pot tray at the pot deck: rim ring + drain holes + downspout to leak tray.

    Rectangular to fit the frame interior (wide side gutters, tight front/back); the
    pot rim seats on a central ring so it never sits in standing water."""
    tray_w = 400.0                                    # within ~428 interior width
    tray_d = 262.0                                    # within ~268 interior depth
    ring_outer_r = P.POT_TOP_DIA / 2 + P.POT_RIM + 1  # 132: pot rim drops onto this
    gutter_drain_x = ring_outer_r + 22                # in the wide side gutters
    with BuildPart() as ptray:
        Box(tray_w, tray_d, TRAY_RIM, align=CENTER_MIN)
        with Locations((0, 0, TRAY_WALL)):
            Box(tray_w - 2 * TRAY_WALL, tray_d - 2 * TRAY_WALL, TRAY_RIM,
                align=CENTER_MIN, mode=Mode.SUBTRACT)
        # locating ring that the pot rim drops onto (keeps pot above standing water)
        with Locations((0, 0, TRAY_WALL)):
            Cylinder(radius=ring_outer_r, height=10, align=CENTER_MIN)
            Cylinder(radius=ring_outer_r - 5, height=12,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
        # drain holes in the side gutters (clear of the ring) -> water falls down
        for sx in (-1, 1):
            for sy in (-1, 0, 1):
                with Locations((sx * gutter_drain_x, sy * 70, 0)):
                    Cylinder(radius=4, height=TRAY_RIM * 2, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # downspout stub on the underside, off to one side, aims into leak tray
        with Locations((gutter_drain_x, tray_d / 2 - 22, -18)):
            Cylinder(radius=8, height=20, align=CENTER_MIN)
        with Locations((gutter_drain_x, tray_d / 2 - 22, -20)):
            Cylinder(radius=5, height=48, align=CENTER_MIN, mode=Mode.SUBTRACT)
    ptray.part.label = "pot-tray"
    return ptray.part


# --- assembly placement ----------------------------------------------------- #
def place_leak_tray(part):
    return Pos(CX, P.ENV_D / 2, P.WET_BAY_FLOOR_Z) * part


def place_pump_clip(part):
    # inside the reservoir footprint, against its rear wall (pump is submersible)
    return Pos(CX + 70, P.RES_D - 10, P.WET_BAY_FLOOR_Z + 6) * part


def place_pot_tray(part):
    return Pos(CX, P.ENV_D / 2, P.POT_DECK_Z) * part


if __name__ == "__main__":
    for fn in (build_leak_tray, build_pump_clip, build_pot_tray):
        p = fn()
        print(f"{p.label}: bbox {p.bounding_box().size}")
