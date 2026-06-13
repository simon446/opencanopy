# SPDX-License-Identifier: CERN-OHL-S-2.0
"""Shared geometry helpers and assembly-frame landmarks."""

from build123d import Align

from . import params as P

# Convenience: everything is modelled in the assembly frame (see params.py).
CX = P.ENV_W / 2.0          # 240 — width centreline
CORNER = P.EXTRUSION + P.UPRIGHT_INSET   # face-to-centre of a corner upright

# Centre lines of the four corner uprights (x, y), in the assembly frame.
UPRIGHT_XY = {
    "front_left":  (P.UPRIGHT_INSET + P.EXTRUSION / 2, P.UPRIGHT_INSET + P.EXTRUSION / 2),
    "front_right": (P.ENV_W - P.UPRIGHT_INSET - P.EXTRUSION / 2, P.UPRIGHT_INSET + P.EXTRUSION / 2),
    "back_left":   (P.UPRIGHT_INSET + P.EXTRUSION / 2, P.ENV_D - P.UPRIGHT_INSET - P.EXTRUSION / 2),
    "back_right":  (P.ENV_W - P.UPRIGHT_INSET - P.EXTRUSION / 2, P.ENV_D - P.UPRIGHT_INSET - P.EXTRUSION / 2),
}

# Common align tuples.
MINMIN = (Align.MIN, Align.MIN, Align.MIN)          # build from a corner
CENTER_MIN = (Align.CENTER, Align.CENTER, Align.MIN)  # centred in XY, sit on Z
