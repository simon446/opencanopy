<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# Wiring harness (WI-EE-05)

- [`harness-table.csv`](harness-table.csv) — every connector, pin, signal, polarity, voltage, wire
  gauge/colour, destination, and strain-relief callout (11 connectors, 32 pins). Every row carries
  polarity + voltage (spec §15.5 M4-03).
- [`wiring-diagram.svg`](wiring-diagram.svg) — full system wiring, drawn against the dry/wet zone
  model (§6.2).

## Strain-relief & drip-loop callouts (§8.5, §17.1)

Every removable wet-zone module gets a **drip loop** so water runs off the cable below the connector,
not into it: pump (`J_PUMP`), moisture probe (`J_MOIST`), reservoir level (`J_RES`), leak sensor
(`J_LEAK`). Each is also clamped/grommeted at the case wall or module so the wire cannot transmit
strain to the connector or PCB pad. Dry-zone runs (LED, SHT40, status board) get a service loop and a
clamp at the module end. *(No fan in V1 — [ECO-001](../analysis/ECO-001-fan-removal.md); the `J_FAN`
header is DNP and carries no harness.)*

## Labelling & routing rules

- Harness labels (`pump`, `LED`, `moisture`, `reservoir`, `leak`) match the mechanical cable
  channel in [WI-ME-07 cable/tube routing](../../plan/work-items/04-mechanical/WI-ME-07-cable-tube-routing.md)
  (§8.5).
- **DR-08 routing rule:** the pump **supply tube** must never cross over the electronics bay; the
  `J_PUMP` electrical run drip-loops below the electronics. This is called out on the diagram and is a
  hard constraint on the mechanical routing.
