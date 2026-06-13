# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
light_mount.py — Overhead LED mount (spec §7.2 mounting, §8.6; WI-ME-05).

Adjustable-height carrier for a 50-80 W full-spectrum LED bar/panel. The carrier
arms pin to a hole row on the rear uprights, giving 150-300 mm clearance above the
canopy. Two captive tether eyelets provide mechanical SECONDARY RETENTION so the
fixture cannot fall into the plant/wet zone (§7.2). The remote driver lives in the
dry bay (light_mount carries the LED head only) so driver heat is kept off the
canopy and plastics (ASA/heat-proven material for this part).

Local frame: fixture-mounting face on z=0 (carrier sits on top of it).
"""

from build123d import Box, BuildPart, Cylinder, Locations, Mode, Pos

from . import params as P
from .util import CX, CENTER_MIN

BAR_W = P.LED_FIXTURE_W + 70      # carrier overhangs the fixture for arms
BAR_T = 12.0
ARM_LEN = 70.0
N_ADJ_HOLES = 6                   # height-adjustment steps on each arm
ADJ_PITCH = 15.0                  # 6 steps x 15 mm ~= 75 mm of travel
# nominal carrier height: fixture underside ~170 mm above the canopy
FIXTURE_UNDERSIDE_Z = 600.0


def build_light_mount():
    with BuildPart() as m:
        # cross carrier bar
        Box(BAR_W, 34, BAR_T, align=CENTER_MIN)
        # LED-fixture mounting bosses on the underside (4)
        for sx in (-1, 1):
            for sy in (-1, 1):
                with Locations((sx * P.LED_FIXTURE_W / 2 * 0.8, sy * 10, -8)):
                    Cylinder(radius=P.SCREW_BOSS_OD / 2, height=8, align=CENTER_MIN)
                    Cylinder(radius=P.HEATSET_M3_DIA / 2, height=9,
                             align=CENTER_MIN, mode=Mode.SUBTRACT)
        # two rear arms reaching toward the uprights, with an adjustment hole row
        for sx in (-1, 1):
            ax = sx * (BAR_W / 2 - 17)
            with Locations((ax, 34 / 2 + ARM_LEN / 2, BAR_T / 2)):
                Box(34, ARM_LEN, BAR_T, align=(CENTER_MIN[0], CENTER_MIN[1], 1))
            for i in range(N_ADJ_HOLES):
                hy = 34 / 2 + 16 + i * ADJ_PITCH
                with Locations((ax, hy, BAR_T / 2)):
                    Cylinder(radius=P.PCB_HOLE_DIA / 2, height=BAR_T * 2,
                             align=CENTER_MIN, mode=Mode.SUBTRACT)
            # secondary-retention tether eyelet at each arm end
            with Locations((ax, 34 / 2 + ARM_LEN, BAR_T / 2)):
                Cylinder(radius=6, height=BAR_T, align=CENTER_MIN)
                Cylinder(radius=P.LED_RETENTION_DIA / 2, height=BAR_T + 1,
                         align=CENTER_MIN, mode=Mode.SUBTRACT)
    m.part.label = "light-mount"
    return m.part


def build_led_fixture():
    """Represented bought LED bar/panel (for clearance + CG checks only)."""
    with BuildPart() as f:
        Box(P.LED_FIXTURE_W, P.LED_FIXTURE_D, P.LED_FIXTURE_H, align=CENTER_MIN)
    f.part.label = "led-fixture"
    return f.part


def place_light_mount(part):
    return Pos(CX, P.ENV_D / 2 - 30, FIXTURE_UNDERSIDE_Z) * part


def place_led_fixture(part):
    return Pos(CX, P.ENV_D / 2, FIXTURE_UNDERSIDE_Z - P.LED_FIXTURE_H) * part


if __name__ == "__main__":
    for fn in (build_light_mount, build_led_fixture):
        p = fn()
        print(f"{p.label}: bbox {p.bounding_box().size}")
