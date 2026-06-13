# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
routing.py — Cable & tube routing + front status diffuser (spec §8.5, §3.4; WI-ME-07).

A divided vertical channel runs up a rear upright keeping WIRING physically separate
from the water TUBE (two pockets, central divider). Snap lips retain the bundles;
the channel feeds the dry-bay grommet entries, where the harness must form a drip
loop before entering (§8.5). Strain-relief slots and replaceable clips are provided
at every removable module. Labels match the harness table (WI-EE-05):
pump / fan / led / moisture / reservoir / leak.

Also defines the translucent front status-LED diffuser (5 positions, §3.4/§3.5).
"""

from build123d import Box, BuildPart, Cylinder, Locations, Mode, Pos, Rotation

from . import params as P
from .util import CX, CENTER_MIN

# Channel runs from just above the pot tray (so its base clears the tray rim) up to
# the dry-bay floor where it delivers the harness into the grommet entries.
CH_BASE_Z = P.POT_DECK_Z + 30                  # 30 mm above the deck (clears tray rim)
CH_LEN = P.DRY_BAY_FLOOR_Z - CH_BASE_Z         # spans the grow zone
W = P.CABLE_CH_W
D = P.CABLE_CH_D
T = P.CABLE_CH_WALL


def build_cable_channel():
    """Divided wire|tube channel, run vertically (length along +Z)."""
    outer_x = 2 * W + 3 * T
    outer_y = D + 2 * T
    with BuildPart() as ch:
        Box(outer_x, outer_y, CH_LEN, align=CENTER_MIN)
        # two pockets (wire side, tube side) separated by the central divider
        for sx in (-1, 1):
            cx = sx * (W / 2 + T / 2)
            with Locations((cx, T, T)):
                Box(W, D, CH_LEN, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # open the front (-Y) mouth of each pocket, leaving retaining lips
        with Locations((0, -outer_y / 2, T)):
            Box(outer_x - 2 * (T + 3), D, CH_LEN, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # strain-relief tie slots every 80 mm down the back
        n = int(CH_LEN // 80)
        for i in range(1, n):
            with Locations((0, outer_y / 2 - T / 2, i * 80)):
                Box(6, T * 3, 3, mode=Mode.SUBTRACT)
    ch.part.label = "cable-channel"
    return ch.part


def build_tube_clip():
    """Snap C-clip for the 8 mm water tube (also a printable spare)."""
    r = P.TUBE_OD / 2
    with BuildPart() as c:
        Box(20, 14, 8, align=CENTER_MIN)
        with Locations((0, 4, 8)):
            Cylinder(radius=r + 2, height=8, align=CENTER_MIN)
            Cylinder(radius=r - P.TUBE_CLIP_GAP, height=10, align=CENTER_MIN, mode=Mode.SUBTRACT)
            # snap mouth opening (slightly narrower than tube -> grips)
            with Locations((0, r, 4)):
                Box(2 * (r - 1.0), 2 * r, 12, mode=Mode.SUBTRACT)
        with Locations((0, 0, -1)):     # screw/heat-set fixing hole
            Cylinder(radius=P.PCB_HOLE_DIA / 2, height=10, align=CENTER_MIN, mode=Mode.SUBTRACT)
    c.part.label = "tube-clip"
    return c.part


def build_cable_clip():
    """Saddle clip for the wiring bundle."""
    with BuildPart() as c:
        Box(22, 16, 6, align=CENTER_MIN)
        with Locations((0, 0, 6)):
            Cylinder(radius=7, height=10, rotation=(90, 0, 0), align=CENTER_MIN)
            Cylinder(radius=5, height=12, rotation=(90, 0, 0), align=CENTER_MIN, mode=Mode.SUBTRACT)
            with Locations((0, 0, 5)):
                Box(8, 20, 12, mode=Mode.SUBTRACT)     # open top to insert bundle
        for sx in (-1, 1):
            with Locations((sx * 8, 0, -1)):
                Cylinder(radius=P.PCB_HOLE_DIA / 2, height=8, align=CENTER_MIN, mode=Mode.SUBTRACT)
    c.part.label = "cable-clip"
    return c.part


def build_sensor_clip():
    """Replaceable clip that holds the capacitive moisture probe at the pot edge."""
    with BuildPart() as c:
        Box(18, 30, 6, align=CENTER_MIN)
        with Locations((0, 6, 6)):
            Cylinder(radius=7, height=20, align=CENTER_MIN)
            Cylinder(radius=5.5, height=22, align=CENTER_MIN, mode=Mode.SUBTRACT)  # probe slot
            with Locations((0, 7, 10)):
                Box(7, 14, 24, mode=Mode.SUBTRACT)
        with Locations((0, -10, -1)):
            Cylinder(radius=P.PCB_HOLE_DIA / 2, height=8, align=CENTER_MIN, mode=Mode.SUBTRACT)
    c.part.label = "sensor-clip"
    return c.part


def build_status_diffuser():
    """Translucent front status strip carrying the 5 status LEDs (§3.4/§3.5)."""
    with BuildPart() as d:
        Box(140, 14, 6, align=CENTER_MIN)
        # 5 LED light-pipe windows
        for i in range(5):
            x = -56 + i * 28
            with Locations((x, 0, 6)):
                Cylinder(radius=4, height=3, align=CENTER_MIN)
            with Locations((x, 0, -1)):
                Cylinder(radius=2.5, height=4, align=CENTER_MIN, mode=Mode.SUBTRACT)
    d.part.label = "status-diffuser"
    return d.part


# --- assembly placement ----------------------------------------------------- #
def place_cable_channel(part):
    # against the back-right upright, inside face; base raised above the pot tray
    x = P.ENV_W - P.UPRIGHT_INSET - P.EXTRUSION - (2 * W + 3 * T) / 2
    return Pos(x, P.ENV_D - P.UPRIGHT_INSET - P.EXTRUSION, CH_BASE_Z) * part


def place_status_diffuser(part):
    return Pos(CX, P.UPRIGHT_INSET + 1, P.POT_DECK_Z - 40) * part


if __name__ == "__main__":
    for fn in (build_cable_channel, build_tube_clip, build_cable_clip,
               build_sensor_clip, build_status_diffuser):
        p = fn()
        print(f"{p.label}: bbox {p.bounding_box().size}")
