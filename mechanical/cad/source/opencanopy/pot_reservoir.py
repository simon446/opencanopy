# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
pot_reservoir.py — Pot and reservoir fit models (spec §3.3, §5.5, §7.7, §16.2; WI-ME-02).

These represent the *selected* commodity parts modelled to prove fit, removal paths
and drainage in the assembly — they are not necessarily printed. The pot is a
drain-capable 10 L food-safe planter; the reservoir is a 4 L food-safe pull-out tub.

Placement (assembly frame): both centred in X; the pot sits on the pot deck, the
reservoir sits on the wet-bay floor as a front-loading drawer.
"""

from build123d import Box, BuildPart, Cone, Cylinder, Locations, Mode, Pos, PolarLocations

from . import params as P
from .util import CX, CENTER_MIN


def build_pot():
    """10 L tapered round pot, drain-capable. Local frame: base on z=0."""
    rb, rt = P.POT_BOT_DIA / 2, P.POT_TOP_DIA / 2
    with BuildPart() as pot:
        Cone(bottom_radius=rb, top_radius=rt, height=P.POT_H, align=CENTER_MIN)
        # hollow it out, leaving a floor of POT_WALL
        with Locations((0, 0, P.POT_WALL)):
            Cone(bottom_radius=rb - P.POT_WALL, top_radius=rt - P.POT_WALL,
                 height=P.POT_H, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # rolled rim that locates the pot on the tray ring
        with Locations((0, 0, P.POT_H - P.POT_RIM)):
            Cylinder(radius=rt + P.POT_RIM, height=P.POT_RIM, align=CENTER_MIN)
            Cylinder(radius=rt - P.POT_WALL, height=P.POT_RIM,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
        # central + ring drain holes (water fails downward into the pot tray)
        with Locations((0, 0, 0)):
            Cylinder(radius=P.POT_DRAIN_DIA / 2, height=P.POT_WALL * 3,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
        with PolarLocations(radius=rb * 0.55, count=6):
            Cylinder(radius=P.POT_DRAIN_DIA / 2, height=P.POT_WALL * 3,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
    pot.part.label = "pot-10L"
    return pot.part


def build_reservoir():
    """4 L food-safe pull-out reservoir (open-top tub + front handle lip)."""
    w, d, h, t = P.RES_W, P.RES_D, P.RES_H, P.RES_WALL
    with BuildPart() as res:
        Box(w, d, h, align=CENTER_MIN)
        with Locations((0, 0, t)):
            Box(w - 2 * t, d - 2 * t, h, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # front pull handle: lip projecting toward -Y at the top front edge
        with Locations((0, -d / 2 - P.RES_HANDLE / 2, h - 18)):
            Box(w * 0.5, P.RES_HANDLE, 14)
        # hand-cleanable rounded fill notch in the top front wall
        with Locations((0, -d / 2, h)):
            Cylinder(radius=P.RES_FILL_DIA / 2, height=t * 3,
                     align=(CENTER_MIN[0], CENTER_MIN[1], CENTER_MIN[2]),
                     mode=Mode.SUBTRACT)
    res.part.label = "reservoir-4L"
    return res.part


# --- assembly placement ----------------------------------------------------- #
def place_pot(part):
    return Pos(CX, P.ENV_D / 2, P.POT_DECK_Z + 6) * part   # 6 mm above tray ring


def place_reservoir(part):
    # front-loaded drawer: front wall near the open face, sitting on wet-bay floor
    y = (P.RES_D / 2) + 24                       # leave 24 mm front gutter
    return Pos(CX, y, P.WET_BAY_FLOOR_Z + 6) * part


if __name__ == "__main__":
    for fn in (build_pot, build_reservoir):
        p = fn()
        print(f"{p.label}: bbox {p.bounding_box().size}")
