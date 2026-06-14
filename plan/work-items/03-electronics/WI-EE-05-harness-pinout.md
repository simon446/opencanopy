# WI-EE-05 — Harness & connector pinout

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-03 |
| Depends on | WI-EE-03 |
| Spec refs | §7.9, §8.5 |
| Status | Done — [harness-table.csv](../../../electronics/wiring/harness-table.csv), [wiring-diagram.svg](../../../electronics/wiring/wiring-diagram.svg), [README](../../../electronics/wiring/README.md), **+ [connector-spec.md](../../../electronics/wiring/connector-spec.md)** (chosen connectors + mating cable-side parts + mechanical requirements — the handoff for the mechanical wiring team) |

## Objective

Produce the wiring harness documentation so assembly is unambiguous and matches the mechanical
cable-routing plan.

## Deliverables

- [x] `electronics/wiring/harness-table.csv` — every connector, pin, signal, polarity, voltage, gauge. *(11 connectors, 32 pins; every row has polarity + voltage.)*
- [x] `electronics/wiring/wiring-diagram.svg` — full system wiring. *(Drawn against dry/wet zone model; well-formed.)*
- [x] Labels matching mechanical routing (pump, LED, moisture, reservoir, leak) per §8.5. *(Labels + DR-08 supply-tube rule noted; **no fan in V1** — ECO-001, `J_FAN` DNP.)*
- [x] Strain-relief and drip-loop callouts at each removable module. *(Drip loops on all wet-zone modules; clamps/grommets documented.)*

## Acceptance criteria

- Every connector has polarity + voltage labeled (spec §15.5 M4-03).
- Harness labels match [WI-ME-07](../04-mechanical/WI-ME-07-cable-tube-routing.md) cable channel.
