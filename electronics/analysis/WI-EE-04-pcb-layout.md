<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# WI-EE-04 — PCB layout (design)

**Status:** Layout fully specified (stackup, floorplan, grounding, pours, test points, connector
placement) + a deterministic [net-class & design-rule recipe](design-rules.md). The board is built by
the headless tscircuit flow ([ECO-002](ECO-002-pcb-toolchain.md); KiCad retired); **residual: real
footprints + a reviewed placement/route** ([programmatic/](../pcb/programmatic/)).
**Spec refs:** §7.9, §7.10.
**Trace-width numbers:** [WI-EE-06 trace report](../test/pcb-verification.md) (this doc states the targets; EE-06 proves them).

## 1. Stackup

**4-layer preferred** (spec §7.9 "4-layer preferred for power/ground"); 2-layer acceptable as a cost
fallback.

| Layer | Use |
|---|---|
| L1 (top) | Components, signal, high-current power fills (24 V, pump, LED) |
| L2 | **Solid GND plane** (return reference for all signals) |
| L3 | Power distribution (12 V, 5 V, 3V3 pours) |
| L4 (bottom) | Signal, analog sensor routing, test points |

A continuous L2 ground plane is the single biggest win for the moisture/ADC analog quality (§7.10)
and for the high-current return paths.

## 2. Floorplan — physical partitioning (§7.10)

The board is partitioned so the **high-current switching domain** never shares copper or return paths
with the **analog sensor domain**:

```text
 ┌──────────────────────────────┬───────────────────────────┐
 │  POWER IN / PROTECTION        │   ANALOG / SENSOR ZONE     │
 │  24V jack, F1, P-FET, TVS,    │   moisture ADC front end,  │
 │  bulk caps, DC/DC regulators  │   leak comparator, I2C,    │
 │                               │   reservoir, LED NTC       │
 ├──────────────────────────────┤   (quiet, star-tied GND)   │
 │  HIGH-CURRENT OUTPUTS         │                            │
 │  pump MOSFET + pour, LED dim, ├───────────────────────────┤
 │  flybacks (fan-drive DNP)     │   MCU + USB/UART + status  │
 │  (LED current loop kept tight)│   LED connector            │
 └──────────────────────────────┴───────────────────────────┘
   field/power connectors along this edge   sensor connectors along this edge
```

- **LED/pump current loops kept small and on the power side**, away from the moisture/ADC lines
  (§7.10). High-current traces do **not** route through or under the analog zone.
- Connectors grouped by domain along the board edges so the harness ([WI-EE-05](../wiring/README.md)) routes
  cleanly to the mechanical cable channel.

## 3. Grounding

- **L2 solid ground plane** as the universal return.
- **Star / partitioned ground** for the analog front end: the moisture/leak/ADC ground ties to the
  main plane at a single point near the MCU ADC reference, so high-current pump/LED return currents do
  not flow under the analog references (§7.10).
- Each DC/DC regulator has a local ground stitch and input/output ceramics close to the part.

## 4. High-current paths & copper pours (§7.10)

| Path | Worst-case current | Target trace / pour | Notes |
|---|---:|---|---|
| 24 V input → regulators / LED | 4.2 A (100 W) | ≥2.5 mm @ 2 oz, or filled pour | proven in [WI-EE-06](../test/pcb-verification.md) |
| LED driver feed | 4.2 A | pour / wide trace | tight loop, power side only |
| Pump MOSFET drain/source | 0.63 A peak | ≥0.6 mm + **copper pour around FET** | pour doubles as heatsink (§7.10) |
| ~~Fan 12 V~~ | — | DNP (no fan in V1, ECO-001) | header footprint only |
| Logic/sensor | <0.1 A | default 0.2 mm | — |

**Copper pours for MOSFET heat dissipation** on the pump FET and the regulators (§7.10); thermal
relief on these pads is minimized so it does not choke the current-carrying path.

## 5. Test points (§7.9 — every rail and control)

Accessible probe points (loop or pad), labeled on silkscreen:

- **Rails:** 24 V, 12 V, 5 V, 3V3, GND (several GND points).
- **Buses/control:** I²C SDA/SCL, UART TX/RX, pump drive (gate + drain), LED dim, moisture ADC,
  reservoir, leak. *(Fan PWM/tach test points are on the DNP fan-drive footprint only — no fan in V1.)*

This set satisfies M4-05 ("all rails and control signals have accessible test points") and is what the
[bring-up procedure (WI-EE-08)](../test/bringup.md) probes.

## 6. Connectors, mounting, silkscreen

- **Locking/keyed connectors** only (JST VH/XH/PH, screw terminals for >1 A); no Dupont (§7.9).
- Silkscreen: connector name, **polarity, voltage, and warning marks** on every field connector.
- Mounting standoffs at corners; no board flex over connectors; serviceable in the upper dry bay.
- Board outline + mount-hole pattern coordinated with the mechanical electronics-bay
  ([WI-ME-*](../../plan/work-items/04-mechanical/)) before fabrication.

## 7. DRC plan (M4-02)

Net classes drive the rules: power nets (24 V/12 V/5 V/3V3) get the wider clearance/width from §4;
24 V↔logic clearance ≥0.5 mm (creepage with possible condensation in mind, even though the board is
in the dry bay). The full rule set is in [design-rules.md](design-rules.md); it is applied (and DRC'd)
in the tscircuit layout step.

## 8. Deliverable status

| Deliverable | State |
|---|---|
| PCB (4-layer preferred) | ✔ stackup + floorplan designed; board generated (tscircuit draft) |
| High-current paths sized, not through control traces | ✔ targets set (proven in WI-EE-06) |
| LED loop away from analog; partitioned/star grounds | ✔ floorplan + grounding specified |
| Copper pours for MOSFET heat | ✔ specified |
| Test points on every rail/control | ✔ enumerated (§5) |
| Locking/keyed connectors; silkscreen polarity/voltage/warnings | ✔ specified |
| DRC clean | ⏳ at the reviewed-layout step (tscircuit); draft is autorouter-grade |
