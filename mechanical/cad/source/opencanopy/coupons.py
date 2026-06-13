# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
coupons.py — Print-tolerance test coupons (spec §12.2; WI-ME-08).

Small, fast-printing pieces to dial in printer/material fit BEFORE committing the
large parts. Each coupon brackets the relevant clearance with a few graded features
so the assembler can pick the value that fits their machine and records it in
fit-tests.md. Acceptance (§12.2): no cracking, no excessive force, no sharp edges
near tubing, parts survive 40 C without warping.

The seven §12.2 coupons:
  snap-fit, screw boss, heat-set insert, tube clip, diffuser slot,
  cable-channel clip, reservoir rail/slide.
"""

from build123d import Box, BuildPart, Cylinder, Locations, Mode

from . import params as P
from .util import CENTER_MIN


def _base(w, d, h=6.0):
    Box(w, d, h, align=CENTER_MIN)


def snap_fit():
    """Cantilever snap finger with a hook lip — tests flex without cracking.

    Sized so the deflection-to-assemble stays within PETG's allowable bending strain:
    beam t=2.5 mm, free length L=18 mm, hook undercut y=1.2 mm gives
    eps = 3*y*t/(2*L^2) = 1.4 % (< ~2-3 % allowable). See tolerance-analysis.md."""
    t, L, undercut = 2.5, 18.0, 1.2
    with BuildPart() as c:
        _base(40, 20)
        # cantilever finger standing up from the base (t in the bending direction = X)
        with Locations((0, 0, 6)):
            Box(t, 12, L, align=CENTER_MIN)
        # hook lip at the tip (overhangs +X by the undercut)
        with Locations((t / 2, 0, 6 + L - 3)):
            Box(2 * undercut, 12, 3, align=CENTER_MIN)
    c.part.label = "coupon-snap-fit"
    return c.part


def screw_boss():
    """Three bosses with pilot holes graded for an M3 self-tapping screw."""
    with BuildPart() as c:
        _base(54, 20)
        for i, pilot in enumerate((2.5, 2.7, 2.9)):
            x = -18 + i * 18
            with Locations((x, 0, 6)):
                Cylinder(radius=P.SCREW_BOSS_OD / 2, height=10, align=CENTER_MIN)
                Cylinder(radius=pilot / 2, height=12, align=CENTER_MIN, mode=Mode.SUBTRACT)
    c.part.label = "coupon-screw-boss"
    return c.part


def heatset_insert():
    """Three bosses bored for an M3 brass heat-set insert (graded)."""
    with BuildPart() as c:
        _base(54, 20)
        for i, bore in enumerate((3.8, 4.0, 4.2)):
            x = -18 + i * 18
            with Locations((x, 0, 6)):
                Cylinder(radius=P.SCREW_BOSS_OD / 2, height=10, align=CENTER_MIN)
                Cylinder(radius=bore / 2, height=12, align=CENTER_MIN, mode=Mode.SUBTRACT)
    c.part.label = "coupon-heatset-insert"
    return c.part


def tube_clip():
    """Three C-clips for the 8 mm tube with graded snap gaps."""
    r = P.TUBE_OD / 2
    with BuildPart() as c:
        _base(66, 22)
        for i, gap in enumerate((0.2, 0.4, 0.6)):
            x = -22 + i * 22
            with Locations((x, 0, 6)):
                Cylinder(radius=r + 2, height=10, align=CENTER_MIN)
                Cylinder(radius=r - gap, height=12, align=CENTER_MIN, mode=Mode.SUBTRACT)
                with Locations((0, r, 5)):
                    Box(2 * (r - 1.0), 2 * r, 14, mode=Mode.SUBTRACT)  # snap mouth
    c.part.label = "coupon-tube-clip"
    return c.part


def diffuser_slot():
    """Three slots graded for a 6 mm diffuser strip (acrylic/PETG)."""
    with BuildPart() as c:
        _base(60, 24, h=10)
        for i, slot in enumerate((6.0, 6.2, 6.4)):
            x = -18 + i * 18
            with Locations((x, 0, 10)):
                Box(slot, 18, 8, align=(CENTER_MIN[0], CENTER_MIN[1], 1), mode=Mode.SUBTRACT)
    c.part.label = "coupon-diffuser-slot"
    return c.part


def cable_channel_clip():
    """Short channel section to test the retaining-lip snap on a wire bundle."""
    W, D, T = P.CABLE_CH_W, P.CABLE_CH_D, P.CABLE_CH_WALL
    with BuildPart() as c:
        Box(W + 2 * T, D + 2 * T, 24, align=CENTER_MIN)
        with Locations((0, T, T)):
            Box(W, D, 24, align=CENTER_MIN, mode=Mode.SUBTRACT)
        with Locations((0, -(D + 2 * T) / 2, T)):
            Box(W - 6, D, 24, align=CENTER_MIN, mode=Mode.SUBTRACT)  # mouth w/ lips
    c.part.label = "coupon-cable-channel-clip"
    return c.part


def reservoir_rail():
    """A rail + captive slider printed together at the slide clearance."""
    with BuildPart() as c:
        # fixed rail (dovetail-ish: a raised bar)
        _base(60, 30)
        with Locations((0, -8, 6)):
            Box(60, 8, 6, align=(CENTER_MIN[0], CENTER_MIN[1], 1))
        # slider that rides over the rail with CLEAR_SLIDE on each face
        g = P.CLEAR_SLIDE
        with Locations((10, -8, 6)):
            Box(30, 8 + 2 * (4 + g), 6 + 4 + g, align=(CENTER_MIN[0], CENTER_MIN[1], 1))
            Box(30, 8 + 2 * g, 6 + g, align=(CENTER_MIN[0], CENTER_MIN[1], 1), mode=Mode.SUBTRACT)
    c.part.label = "coupon-reservoir-rail"
    return c.part


ALL = (snap_fit, screw_boss, heatset_insert, tube_clip,
       diffuser_slot, cable_channel_clip, reservoir_rail)


def build_coupons():
    return [fn() for fn in ALL]


if __name__ == "__main__":
    for p in build_coupons():
        print(f"{p.label}: bbox {p.bounding_box().size}")
