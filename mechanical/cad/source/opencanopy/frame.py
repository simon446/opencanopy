# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
frame.py — Open-frame vertical stack (spec §8.1, §8.2 "Frame").

Four 20x20 corner uprights joined by top rails and a mid deck-support ring at the
pot-deck station. Deliberately open (no sealed box) per §8.1: just enough structure
to carry the light, fan, cable channel, reservoir, electronics bay and pot.

Rail ends overlap the uprights so the boolean union yields a single watertight solid.
"""

from build123d import Box, BuildPart, Locations

from . import params as P
from .util import UPRIGHT_XY, CENTER_MIN


def build_frame():
    e = P.EXTRUSION
    with BuildPart() as frame:
        # --- 4 corner uprights, full height ---------------------------------
        for (x, y) in UPRIGHT_XY.values():
            with Locations((x, y, 0)):
                Box(e, e, P.ENV_H, align=CENTER_MIN)  # centred in XY, on floor

        # Helper spans between two upright centres along X or Y, overlapping
        # into the uprights so the union fuses (extend by half-extrusion each end).
        xl = UPRIGHT_XY["front_left"][0]
        xr = UPRIGHT_XY["front_right"][0]
        yf = UPRIGHT_XY["front_left"][1]
        yb = UPRIGHT_XY["back_left"][1]
        span_x = (xr - xl) + e          # full overlap into both uprights
        span_y = (yb - yf) + e
        cx = (xl + xr) / 2
        cy = (yf + yb) / 2

        def rail_x(z, front=True):  # rail running along X (front and/or back)
            ys = (yf, yb) if front else (yb,)
            for y in ys:
                with Locations((cx, y, z)):
                    Box(span_x, e, e)  # default align = CENTER on all axes

        def rail_y(z):  # rail running along Y (left and right)
            for x in (xl, xr):
                with Locations((x, cy, z)):
                    Box(e, span_y, e)

        # --- top rails (carry dry bay + light mount reaction) ---------------
        top_z = P.ENV_H - e / 2
        rail_x(top_z)
        rail_y(top_z)

        # --- mid deck-support ring at the pot deck (OPEN FRONT for pot access) --
        deck_z = P.POT_DECK_Z - e / 2
        rail_x(deck_z, front=False)
        rail_y(deck_z)

        # --- lower ring above the feet (OPEN FRONT so the reservoir drawer
        #     can be pulled straight out the service face) --------------------
        low_z = P.WET_BAY_FLOOR_Z + e / 2
        rail_x(low_z, front=False)
        rail_y(low_z)

        # --- rear cross-rail at the fan height (gives the fan mount a member to
        #     bolt to and stiffens the open rear) ----------------------------
        with Locations((cx, yb, P.FAN_MOUNT_Z)):
            Box(span_x, e, e)
    frame.part.label = "frame"
    return frame.part


if __name__ == "__main__":
    p = build_frame()
    print("frame bbox:", p.bounding_box().size, "volume:", round(p.volume, 1))
