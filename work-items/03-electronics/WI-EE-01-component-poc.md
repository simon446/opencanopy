# WI-EE-01 — Component selection & breadboard PoC

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M2-01 … M2-08 |
| Depends on | WI-PS-04 |
| Spec refs | §7.1–§7.7, §16 |
| Status | Not started |

## Objective

Validate every sensor and actuator on a breadboard against an ESP32-S3 dev board **before** committing
to a custom PCB. De-risk the parts, not the layout.

## Deliverables

- [ ] MCU: ESP32-S3 dev board selected and logged as BOM entry (§7.1).
- [ ] Temp/RH sensor (SHT31/SHT4x-class): bench log, stable vs reference (§7.5).
- [ ] Capacitive moisture sensor: dry/wet/in-media readings recorded (§7.6).
- [ ] Reservoir low-level sensor: reliable detection logged (§7.7).
- [ ] Leak sensor: pump-lockout signal reliable.
- [ ] Pump (brushless DC submersible): flow (ml/s) + noise (dBA @1 m) measured (§7.3).
- [ ] Fan (80/92 mm PWM): min reliable duty + noise measured (§7.4).
- [ ] LED dimming: PPFD map at 25/50/75/100% recorded (§7.2) using [WI-PL-02](../01-plant-science/WI-PL-02-light-dli-targets.md) calculator.
- [ ] Grow-light candidate meets §16.3 gating (real PPF/PPFD, dimming, spectrum, thermal data).

## Acceptance criteria

- Each component has a bench log meeting its §15.3 acceptance row.
- No component advertised only in lumens/"equivalent watts" passes the light gate (§16.3).

## Notes

Feeds the schematic ([WI-EE-03](WI-EE-03-schematic.md)) and the power budget ([WI-EE-02](WI-EE-02-power-budget.md)).
