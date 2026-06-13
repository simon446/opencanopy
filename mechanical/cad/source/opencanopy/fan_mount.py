# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
fan_mount.py — Guarded, vibration-isolated fan mount (spec §7.4, §8.7; WI-ME-06).

80/92 mm quiet PWM fan on rubber grommets (decoupled from the frame to keep noise
out, §8.7) with a REQUIRED finger guard (concentric rings + spokes, §7.4). Placed
high at the rear and offset to one side so the canopy gets gentle circulation, not
a direct drying stream at the seedling. A thin perimeter recess accepts an optional
removable intake filter.

Built flat with the airflow axis = +Z; the assembly rotates it to face the canopy.
"""

from build123d import Box, BuildPart, Cylinder, Locations, Mode, Pos, Rotation, PolarLocations

from . import params as P
from .util import CENTER_MIN

PLATE = P.FAN_SIZE + 26
PLATE_T = 4.0
GUARD_T = 3.0
GUARD_Z = PLATE_T


def build_fan_mount():
    hp = P.FAN_HOLE_PITCH / 2
    with BuildPart() as fm:
        # --- mounting plate with central airflow bore ----------------------
        Box(PLATE, PLATE, PLATE_T, align=CENTER_MIN)
        with Locations((0, 0, -1)):
            Cylinder(radius=P.FAN_BORE / 2, height=PLATE_T + 2,
                     align=CENTER_MIN, mode=Mode.SUBTRACT)
        # --- 4 grommet bores (rubber isolation) ----------------------------
        for sx in (-1, 1):
            for sy in (-1, 1):
                with Locations((sx * hp, sy * hp, -1)):
                    Cylinder(radius=P.FAN_GROMMET_DIA / 2, height=PLATE_T + 2,
                             align=CENTER_MIN, mode=Mode.SUBTRACT)
        # --- optional removable intake-filter recess (shallow lip) ---------
        with Locations((0, 0, PLATE_T - 1.2)):
            Cylinder(radius=P.FAN_SIZE / 2 + 1, height=1.4, align=CENTER_MIN)
            Cylinder(radius=P.FAN_SIZE / 2 - 2, height=2, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # --- REQUIRED finger guard: hub + concentric rings + spokes --------
        Cylinder(radius=8, height=GUARD_Z + GUARD_T, align=CENTER_MIN)   # hub
        for r in (22, 36, P.FAN_BORE / 2 - 1):
            with Locations((0, 0, GUARD_Z)):
                Cylinder(radius=r, height=GUARD_T, align=CENTER_MIN)
                Cylinder(radius=r - 3, height=GUARD_T + 1, align=CENTER_MIN, mode=Mode.SUBTRACT)
        # spokes tying the rings to the hub
        with Locations((0, 0, GUARD_Z)):
            for ang in (0, 60, 120):
                with PolarLocations(radius=0, count=1, start_angle=ang):
                    Box(P.FAN_BORE, 3, GUARD_T, rotation=(0, 0, ang),
                        align=(CENTER_MIN[0], CENTER_MIN[1], 1))
        # --- mounting tab on the bottom edge to fix to a rear rail ---------
        with Locations((0, -PLATE / 2 - 12, 0)):
            Box(60, 24, PLATE_T)
        for sx in (-1, 1):
            with Locations((sx * 18, -PLATE / 2 - 16, -1)):
                Cylinder(radius=P.PCB_HOLE_DIA / 2, height=PLATE_T + 2,
                         align=CENTER_MIN, mode=Mode.SUBTRACT)
    fm.part.label = "fan-mount"
    return fm.part


def place_fan_mount(part):
    # high at the rear, offset to the right so airflow crosses (not blasts) canopy
    return Pos(P.ENV_W / 2 + 80, P.ENV_D - P.UPRIGHT_INSET - PLATE_T,
               P.POT_DECK_Z + 300) * Rotation(90, 0, 0) * part


if __name__ == "__main__":
    p = build_fan_mount()
    print(f"{p.label}: bbox {p.bounding_box().size}")
