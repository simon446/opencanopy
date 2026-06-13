# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
params.py — Single source of truth for the OpenCanopy V1 mechanical design.

Every dimension that more than one module depends on lives here, so the locked
envelope (docs/product-requirements.md) and the cross-track interfaces (PCB size,
fan size, tube bore, …) are defined exactly once. Changing a locked value here
requires a risk-register entry (per the scope-lock contract, WI-PS-04).

All units are millimetres unless noted. The coordinate frame used by every module
and by the assembly:

    origin  = front-left-bottom corner of the bounding envelope, on the table top
    +X      = to the right   (width,  W)
    +Y      = toward the back (depth,  D)   -> the open face is the -Y / front
    +Z      = up             (height, H)

Spec refs: §3.3 footprint, §6.2 zone model, §7.2-7.4/7.7 hardware, §8 mechanical,
§16.2 mechanical BOM.
"""

from dataclasses import dataclass

# --------------------------------------------------------------------------- #
# 1. Locked envelope (docs/product-requirements.md §1; spec §3.3, §20)
# --------------------------------------------------------------------------- #
# Compact V1 build values (the numbers we model). Hard maxima are recorded for
# reference / assertions only — no V1 build may exceed them.
ENV_W = 480.0          # width   (compact target 450-500, hard max 550)
ENV_D = 320.0          # depth   (compact target 300-350, hard max 400)
ENV_H = 700.0          # height  (compact target 650-750, hard max 850)

ENV_W_MAX = 550.0
ENV_D_MAX = 400.0
ENV_H_MAX = 850.0

# Useful canopy footprint target (compact): 300-400 W x 220-300 D.
CANOPY_W = 360.0
CANOPY_D = 260.0

# --------------------------------------------------------------------------- #
# 2. Frame (spec §8.1 vertical stack, §8.2 frame module)
# --------------------------------------------------------------------------- #
# Open-frame: four corner uprights of 20x20 extrusion (V-slot-class) joined by
# top and mid rails. Uprights are inset from the envelope edge by the trim wall.
EXTRUSION = 20.0                 # 20x20 mm aluminium extrusion (2020-class)
TRIM_WALL = 6.0                  # decorative trim / panel thickness at edges
UPRIGHT_INSET = TRIM_WALL        # upright outer face sits this far in from envelope

# Z-stations of the structural decks (floor = table top = z 0).
FOOT_H = 15.0                    # rubber/levelling feet height
WET_BAY_FLOOR_Z = FOOT_H         # leak-tray floor sits on the feet
WET_BAY_H = 165.0                # clear height of the bottom wet bay
POT_DECK_Z = WET_BAY_FLOOR_Z + WET_BAY_H   # = 180: deck the pot tray sits on
DRY_BAY_H = 92.0                 # upper dry electronics bay height
DRY_BAY_FLOOR_Z = ENV_H - DRY_BAY_H        # = 608

# Grow zone is everything between the pot deck and the dry-bay floor.
GROW_ZONE_H = DRY_BAY_FLOOR_Z - POT_DECK_Z  # = 428 mm of open grow space

# --------------------------------------------------------------------------- #
# 3. Pot (spec §3.3, §5.5, §16.2; WI-ME-02)
# --------------------------------------------------------------------------- #
# 10 L compact pot, modelled as a lightly tapered round nursery pot.
#   V(cone frustum) = (pi/3) * h * (R^2 + R*r + r^2)
#   Top dia is bounded by the interior DEPTH between uprights (~268 mm), so the pot
#   is taller/narrower than a typical squat pot: 250/210 dia, 255 tall -> ~10.6 L
#   gross (~10 L usable), and clears the uprights with margin.
POT_TOP_DIA = 250.0
POT_BOT_DIA = 210.0
POT_H = 255.0
POT_WALL = 4.0
POT_RIM = 6.0                    # rolled rim overhang (sits/locates on tray ring)
POT_DRAIN_DIA = 16.0            # central + ring drain holes to tray below

# --------------------------------------------------------------------------- #
# 4. Reservoir (spec §3.3, §7.7, §16.2; WI-ME-02/04)
# --------------------------------------------------------------------------- #
# 4 L pull-out reservoir (drawer) in the wet bay. Outer box; ~4.1 L usable.
#   240 x 150 x 125 outer -> ~4.5 L gross, ~4.1 L usable.
RES_W = 240.0
RES_D = 150.0
RES_H = 125.0
RES_WALL = 3.0
RES_HANDLE = 22.0                # front pull-handle depth (toward -Y)
RES_FILL_DIA = 70.0             # hand-cleanable / fill opening
RES_LOW_LINE = 25.0             # low-level sensor trip height above inner floor

# --------------------------------------------------------------------------- #
# 5. Pump (spec §7.3; WI-ME-04)
# --------------------------------------------------------------------------- #
PUMP_DIA = 45.0                  # submersible centrifugal pump body envelope
PUMP_H = 60.0
PUMP_ISO_PAD = 4.0               # silicone isolation pad / suction-cup standoff

# Tubing (§7.3): 6/8 mm silicone -> 8 mm OD.
TUBE_OD = 8.0
TUBE_ID = 6.0
TUBE_CLIP_GAP = 0.4              # snap-clip nominal interference for 8 mm tube

# --------------------------------------------------------------------------- #
# 6. Electronics dry bay + PCB (spec §6.2, §7.9, §8.4; WI-ME-03)
# --------------------------------------------------------------------------- #
# PCB outline is an interface owned by Electronics (WI-EE-04, not yet frozen).
# We design the bay to a generous reserved board envelope and publish it back to
# the EE track as the mechanical budget. See cad-verification-checklist.md.
PCB_W = 120.0                    # reserved controller PCB envelope (X)
PCB_D = 90.0                     # reserved controller PCB envelope (Y)
PCB_STANDOFF_H = 8.0             # board sits this high off the bay floor
PCB_HOLE_INSET = 5.0             # mounting-hole inset from board edge
PCB_HOLE_DIA = 3.2               # clearance for M3 / M3 heat-set boss
PCB_CLEARANCE_TOP = 30.0         # headroom above board for tall parts/driver

DRIVER_W = 110.0                 # remote LED driver brick (kept in dry bay)
DRIVER_D = 40.0
DRIVER_H = 30.0

GROMMET_DIA = 12.0               # cable grommet bore at bay entries (§8.5)

# --------------------------------------------------------------------------- #
# 7. Light mount (spec §7.2 mounting, §8.6; WI-ME-05)
# --------------------------------------------------------------------------- #
# 50-80 W full-spectrum LED bar/panel, remote driver. Adjustable height on the
# rear uprights; 150-300 mm clearance above canopy.
LED_FIXTURE_W = 320.0            # LED bar/panel footprint (X)
LED_FIXTURE_D = 120.0            # LED bar/panel footprint (Y)
LED_FIXTURE_H = 30.0
LED_CLEAR_MIN = 150.0            # min clearance above mature canopy (§8.6)
LED_CLEAR_MAX = 300.0            # max useful clearance
LED_RETENTION_DIA = 4.0          # secondary-retention cable/tether bore (§7.2)

# --------------------------------------------------------------------------- #
# 8. Fan (spec §7.4, §8.7; WI-ME-06)
# --------------------------------------------------------------------------- #
FAN_SIZE = 92.0                  # 80 or 92 mm; modelled at 92
FAN_THK = 25.0
FAN_HOLE_PITCH = 82.5            # 92 mm fan mounting-hole pitch (80 mm -> 71.5)
FAN_GROMMET_DIA = 6.0            # rubber isolation grommet bore
FAN_BORE = 88.0                  # airflow bore through mount plate
FAN_GUARD_RING = 5.0             # guard wire/ring thickness (required, §7.4)
# Fan height + its rear support cross-rail. Kept above the pot rim (441 mm) so the
# guard clears the pot and circulates the canopy rather than sitting beside the pot.
FAN_MOUNT_Z = POT_DECK_Z + 340   # 520 mm

# --------------------------------------------------------------------------- #
# 9. Cable & tube routing (spec §8.5; WI-ME-07)
# --------------------------------------------------------------------------- #
CABLE_CH_W = 24.0                # cable channel internal width
CABLE_CH_D = 16.0                # cable channel internal depth
CABLE_CH_WALL = 2.4
DRIP_LOOP_R = 20.0               # min drip-loop / cable bend radius
# Harness labels — MUST match electronics/wiring/harness-table.csv (WI-EE-05).
HARNESS_LABELS = ("pump", "fan", "led", "moisture", "reservoir", "leak")

# --------------------------------------------------------------------------- #
# 10. Print / tolerance defaults (spec §8.3, §12.2; WI-ME-08)
# --------------------------------------------------------------------------- #
# Nominal printed-fit clearances validated by the coupons (fit-tests.md).
CLEAR_SNAP = 0.20                # snap-fit nominal interference
CLEAR_SLIDE = 0.30               # sliding rail clearance (reservoir drawer)
CLEAR_PRESS = 0.10               # press / heat-set boss
HEATSET_M3_DIA = 4.0             # boss bore for M3 brass heat-set insert
SCREW_BOSS_OD = 8.0              # outer dia of a screw/insert boss
WALL_MIN = 2.0                   # minimum printed wall


@dataclass(frozen=True)
class Envelope:
    w: float = ENV_W
    d: float = ENV_D
    h: float = ENV_H


def assert_within_envelope(w: float, d: float, h: float) -> None:
    """Guard used by the assembly build to keep us inside the hard maxima."""
    assert w <= ENV_W_MAX, f"width {w} exceeds hard max {ENV_W_MAX}"
    assert d <= ENV_D_MAX, f"depth {d} exceeds hard max {ENV_D_MAX}"
    assert h <= ENV_H_MAX, f"height {h} exceeds hard max {ENV_H_MAX}"
