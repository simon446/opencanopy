# WI-ME-03 — Electronics dry bay

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-03 |
| Depends on | WI-ME-01 |
| Spec refs | §6.2, §8.4, §17.1 |
| Status | Not started |

## Objective

Design the upper dry electronics bay so the controller PCB, LED driver, and power distribution stay
isolated from the wet path and are serviceable.

## Deliverables

- [ ] CAD for the dry bay housing controller PCB, LED driver, power distribution, status wiring.
- [ ] Standoffs / no board flex; serviceable access **without** opening the wet bay (§8.4).
- [ ] Splash protection; cable entries use grommets + drip loops (§8.5).
- [ ] Fits the board from [WI-EE-04](../03-electronics/WI-EE-04-pcb-layout.md).

## Acceptance criteria

- Dry service bay isolated from wet zone (spec §15.6 M5-03).
- Electronics accessible without touching the reservoir/pump.
