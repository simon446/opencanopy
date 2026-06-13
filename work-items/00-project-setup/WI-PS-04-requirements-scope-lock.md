# WI-PS-04 — Requirements & scope lock

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | M0-04, M0-05 |
| Depends on | WI-PS-01 |
| Spec refs | §3.3, §4.1, §4.2, §20 |
| Status | Not started |

## Objective

Freeze the physical envelope and the V1 scope so downstream tracks design against fixed targets.

## Deliverables

- [ ] `docs/product-requirements.md` capturing the locked decisions from spec §20:
  - [ ] Footprint: 450–500 × 300–350 × 650–750 mm compact; 550 × 400 × 850 mm full-yield max.
  - [ ] Pot 8–10 L compact / 12–19 L full-yield; reservoir 2.5–4 L / 4–6 L.
  - [ ] MCU = ESP32-S3; no display; hidden service control only.
  - [ ] Pump = brushless DC submersible centrifugal; fan = 80/92 mm PWM.
  - [ ] Light = 50–80 W dimmable full-spectrum white; 100 W only for full-yield variant.
  - [ ] Open-frame / non-enclosed default; room 22–25 °C assumed.
- [ ] `docs/scope.md` listing V1 non-goals (camera, cloud AI, pH/EC dosing, multi-plant, app dependency).

## Acceptance criteria

- Max W/D/H, pot, and reservoir sizes are stated as hard numbers, not ranges-for-discussion.
- Excluded features (camera/cloud/pH/EC) are explicitly out of scope and reference the §4.3 expansion headers.

## Notes

This is the contract every other track designs to. Changes after lock require a risk-register entry.
