# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
electronics_bay.py — Upper dry electronics bay (spec §6.2, §8.4, §8.5, §17.1; WI-ME-03).

Houses the controller PCB, remote LED driver and power distribution at the TOP of
the stack, isolated from the wet path. Cables enter through grommeted floor bosses
with raised collars + drip-loop hooks so water can never track in. Serviced from the
top via a removable lid — no need to open the wet bay (§8.4).

Local frame: bay floor on z=0, centred in X; placed at the top-rear in the assembly.
"""

from build123d import (Box, BuildPart, Cylinder, Locations, Mode)

from . import params as P
from .util import CX, CENTER_MIN

BAY_W = 420.0
BAY_D = 170.0
BAY_WALL = 3.0
# rear-back placement: back face near the rear uprights
BAY_BACK_Y = P.ENV_D - P.UPRIGHT_INSET          # 314
BAY_CY = BAY_BACK_Y - BAY_D / 2                  # centre Y in assembly frame


def _pcb_holes(z, dia, depth, mode):
    hx = (P.PCB_W - 2 * P.PCB_HOLE_INSET) / 2
    hy = (P.PCB_D - 2 * P.PCB_HOLE_INSET) / 2
    # PCB sits on the left half of the bay; driver on the right.
    cx0 = -BAY_W / 2 + 30 + P.PCB_W / 2
    for sx in (-1, 1):
        for sy in (-1, 1):
            with Locations((cx0 + sx * hx, sy * hy, z)):
                Cylinder(radius=dia / 2, height=depth, align=CENTER_MIN, mode=mode)


def build_dry_bay():
    with BuildPart() as bay:
        # --- tray: floor + 4 walls, open top -------------------------------
        Box(BAY_W, BAY_D, P.DRY_BAY_H, align=CENTER_MIN)
        with Locations((0, 0, BAY_WALL)):
            Box(BAY_W - 2 * BAY_WALL, BAY_D - 2 * BAY_WALL, P.DRY_BAY_H,
                align=CENTER_MIN, mode=Mode.SUBTRACT)

        # --- PCB standoffs (heat-set M3) -----------------------------------
        _pcb_holes(BAY_WALL, P.SCREW_BOSS_OD, P.PCB_STANDOFF_H, Mode.ADD)
        _pcb_holes(BAY_WALL, P.HEATSET_M3_DIA, P.PCB_STANDOFF_H + BAY_WALL, Mode.SUBTRACT)

        # --- LED-driver feet (right half) ----------------------------------
        dx = BAY_W / 2 - 30 - P.DRIVER_W / 2
        for sx in (-1, 1):
            for sy in (-1, 1):
                with Locations((dx + sx * (P.DRIVER_W / 2 - 6), sy * (P.DRIVER_D / 2 - 6), BAY_WALL)):
                    Cylinder(radius=P.SCREW_BOSS_OD / 2, height=P.PCB_STANDOFF_H, align=CENTER_MIN)
                    Cylinder(radius=P.HEATSET_M3_DIA / 2, height=P.PCB_STANDOFF_H + 1,
                             align=CENTER_MIN, mode=Mode.SUBTRACT)

        # --- grommeted cable entries in the floor, front edge --------------
        # raised collar + hook post per entry => forced drip loop (§8.5, §17.1)
        entry_y = -BAY_D / 2 + 22
        for ex in (-130, 0, 130):
            with Locations((ex, entry_y, 0)):
                # collar standpipe rising above the floor
                Cylinder(radius=P.GROMMET_DIA / 2 + 3, height=BAY_WALL + 10, align=CENTER_MIN)
            with Locations((ex, entry_y, -1)):
                Cylinder(radius=P.GROMMET_DIA / 2, height=BAY_WALL + 14,
                         align=CENTER_MIN, mode=Mode.SUBTRACT)
            # drip-loop hook post beside the entry
            with Locations((ex + 18, entry_y, BAY_WALL)):
                Cylinder(radius=3, height=16, align=CENTER_MIN)
    bay.part.label = "electronics-dry-bay"
    return bay.part


def build_dry_bay_lid():
    """Removable top lid with a lip; top service access (§8.4)."""
    with BuildPart() as lid:
        # top plate occupies z 0..3; the locating lip hangs DOWN into the tray
        Box(BAY_W, BAY_D, 3, align=CENTER_MIN)
        with Locations((0, 0, -6)):
            Box(BAY_W - 2 * BAY_WALL - 0.6, BAY_D - 2 * BAY_WALL - 0.6, 6, align=CENTER_MIN)
            Box(BAY_W - 2 * BAY_WALL - 0.6 - 8, BAY_D - 2 * BAY_WALL - 0.6 - 8, 6,
                align=CENTER_MIN, mode=Mode.SUBTRACT)
        # finger vents through the top plate
        for vx in (-60, 0, 60):
            with Locations((vx, 0, -1)):
                Box(20, 4, 6, mode=Mode.SUBTRACT)
    lid.part.label = "electronics-dry-bay-lid"
    return lid.part


def place_dry_bay(part):
    from build123d import Pos
    return Pos(CX, BAY_CY, P.DRY_BAY_FLOOR_Z) * part


def place_dry_bay_lid(part):
    from build123d import Pos
    return Pos(CX, BAY_CY, P.ENV_H - 3) * part


if __name__ == "__main__":
    for fn in (build_dry_bay, build_dry_bay_lid):
        p = fn()
        print(f"{p.label}: bbox {p.bounding_box().size}")
