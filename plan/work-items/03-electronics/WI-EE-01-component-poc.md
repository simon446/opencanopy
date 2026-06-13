# WI-EE-01 — Component selection & breadboard PoC

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M2-01 … M2-08 |
| Depends on | WI-PS-04, WI-PL-06, WI-EE-10 |
| Spec refs | §7.1–§7.7, §16, §23 (DR-01) |
| Status | In progress — selection + bench protocol done ([report](../../../electronics/analysis/WI-EE-01-component-poc.md)); bench measurements pending breadboard hardware |

## Objective

Validate every sensor and actuator on a breadboard against an ESP32-S3 dev board **before** committing
to a custom PCB. De-risk the parts, not the layout.

## Deliverables

- [x] MCU: ESP32-S3 dev board selected and logged as BOM entry (§7.1). *(ESP32-S3-DevKitC-1 N8R8.)*
- [ ] Temp/RH sensor (SHT31/SHT4x-class): bench log, stable vs reference (§7.5). *(SHT40 selected; protocol + log template ready — bench run pending hardware.)*
- [ ] Capacitive moisture sensor: dry/wet/in-media readings recorded (§7.6). *(Capacitive v2 selected; 4-point §7.6 calibration protocol ready — pending media.)*
- [ ] Reservoir low-level sensor: reliable detection logged (§7.7). *(Float switch selected; fill/drain protocol ready.)*
- [ ] Leak sensor: pump-lockout signal reliable. *(Conductive-trace pad selected; latched-lockout test defined.)*
- [ ] Pump (brushless DC submersible): flow (ml/s) + noise (dBA @1 m) measured (§7.3). *(24 V centrifugal selected; 30 s-cylinder + SPL protocol ready.)*
- [ ] Fan (80/92 mm PWM): min reliable duty + noise measured (§7.4). *(92 mm 4-pin selected; PWM/tach sweep protocol ready.)*
- [ ] LED dimming: PPFD map at 25/50/75/100% recorded (§7.2) using [WI-PL-02](../01-plant-science/WI-PL-02-light-dli-targets.md) calculator. *(Pending grow-light part + PAR meter + WI-PL-06.)*
- [ ] Grow-light candidate meets §16.3 gating (real PPF/PPFD, dimming, spectrum, thermal data). *(Held on DR-01 — thermal half ✔ WI-EE-10, photometric half ⏳ WI-PL-06.)*
- [x] Grow-light BOM entry **not finalized** until the §23 DR-01 modeling gate passes
      ([WI-PL-06](../01-plant-science/WI-PL-06-photometric-model.md) photometric +
      [WI-EE-10](WI-EE-10-thermal-budget-model.md) thermal). *(Deliberately deferred; carried in BOM as `LIGHT-TBD-DR01` placeholder.)*

## Acceptance criteria

- Each component has a bench log meeting its §15.3 acceptance row.
- No component advertised only in lumens/"equivalent watts" passes the light gate (§16.3).

## Notes

Feeds the schematic ([WI-EE-03](WI-EE-03-schematic.md)) and the power budget ([WI-EE-02](WI-EE-02-power-budget.md)).
