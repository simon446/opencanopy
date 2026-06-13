# WI-PL-01 — Hot-pepper lifecycle profile

| Field | Value |
|---|---|
| Track | Plant Science |
| Milestone | M1-01 |
| Depends on | WI-PS-04 |
| Spec refs | §5.1, §5.2, §2 (research basis) |
| Status | Not started |

## Objective

Author the authoritative hot-pepper (Carolina Reaper-class *Capsicum chinense*) lifecycle profile
that the firmware encodes as a fixed, no-config recipe.

## Deliverables

- [ ] `docs/plant-profile-hot-pepper.md` containing:
  - [ ] Stage table S0–S5 with durations and triggers (spec §5.1).
  - [ ] Per-stage environmental targets: temp, RH, VPD, DLI, photoperiod, PPFD (spec §5.2).
  - [ ] Explicit note that stage detection is **time-based** (no camera) and reset via service button.
  - [ ] `TRANSPLANT_PROFILE` build-flag behavior (skip S0/S1) documented.
- [ ] Each target value cited back to a source ID (R1–R17) from spec §2.2.

## Acceptance criteria

- Stage table and setpoints match spec §5.1/§5.2 and are traceable to cited research.
- Output is machine-translatable into the firmware profile module ([WI-FW-03](../02-firmware/WI-FW-03-plant-profile.md)).

## Notes

This is the single source of truth for "what the plant wants." Firmware must not hardcode setpoints
that diverge from this doc.
