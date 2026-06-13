# WI-QA-08 — Electrical & water safety verification

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | (gates release) |
| Depends on | WI-EE-08 |
| Spec refs | §11.4, §17 |
| Status | Not started |

## Objective

Independently verify the safety properties before any plant trial or release, treating safety as a
hard gate rather than a side effect of other tests.

## Deliverables

- [ ] Confirm: certified external PSU; input fuse present; reverse-polarity protection; no AC mains
      inside unit (§17.1).
- [ ] Confirm pump fails OFF on MCU reset; watchdog + brownout enabled (§11.4).
- [ ] Confirm leak tray + leak-detection lockout; reservoir cannot overflow into electronics.
- [ ] Confirm fan guard, no exposed sharp metal, no easy impeller access, secure reservoir lid (§17.4).
- [ ] Capsaicin/child/pet handling warning present in docs (§17.4).
- [ ] Sign-off report in `validation/test-plans/`.

## Acceptance criteria

- Every §17 safety requirement verified with evidence; fault states persist until safe (leak = manual clear).
