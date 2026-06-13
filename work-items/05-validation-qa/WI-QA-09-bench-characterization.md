# WI-QA-09 — Bench characterization of sensor/actuator physics

| Field | Value |
|---|---|
| Track | Validation & QA (Firmware input) |
| Milestone | M2.5 (new pre-firmware-trust gate — §23 DR-02) |
| Depends on | WI-EE-01 |
| Spec refs | §10.1, §10.3, §7.6, §9.6, §23 (DR-02) |
| Status | Not started |

## Objective

Measure the **real** physical behavior of the moisture sensor, pump, and thermal response on the
bench, and use that measured data to **parameterize the simulator models** ([WI-FW-09](../02-firmware/WI-FW-09-simulator.md)).
The §10.3 simulator validates control *logic*; it cannot prove reality matches its assumed transfer
functions. This WI closes that gap **before** any closed-loop watering runs on a live plant (§23 DR-02).

## Deliverables

- [ ] Substrate **dry-down curve**: capacitive normalized-moisture vs time for the actual potting mix,
      across multiple sensor placements (captures channeling / placement variability, §7.6).
- [ ] Pump **transfer function**: delivered volume vs on-time (ml/s) across reservoir levels/head, with
      the §7.5 current-sense signature for normal vs dry-run/clog.
- [ ] **Thermal step response**: canopy-air and proxy-heatsink temperature vs an LED step, to set the
      sim's LED→heat time constant.
- [ ] Updated WI-FW-09 sim parameters derived from measured data, with prior *assumed* vs *measured*
      values documented.

## Acceptance criteria

- WI-FW-09 sim models are parameterized from measured bench data (not assumed constants), and the 11
  §10.3 scenarios are re-run against the measured-parameter models.
- The capacitive-only watering path is shown on the bench to detect dry-run/clog via current-sense and
  to track real dry-down within a documented error band — **or** the finding (need load cell / different
  sensor, DR-04) is raised before the grow trial.

## Notes

Feeds [WI-FW-09](../02-firmware/WI-FW-09-simulator.md) and de-risks R3 (overwatering) and the watering
reliability concern (§23 DR-04) with real data before a live plant is committed. This is the gate that
turns "sim passes" into "sim reflects this hardware."
