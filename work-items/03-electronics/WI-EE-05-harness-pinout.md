# WI-EE-05 — Harness & connector pinout

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-03 |
| Depends on | WI-EE-03 |
| Spec refs | §7.9, §8.5 |
| Status | Not started |

## Objective

Produce the wiring harness documentation so assembly is unambiguous and matches the mechanical
cable-routing plan.

## Deliverables

- [ ] `electronics/wiring/harness-table.csv` — every connector, pin, signal, polarity, voltage, gauge.
- [ ] `electronics/wiring/wiring-diagram.svg` — full system wiring.
- [ ] Labels matching mechanical routing (pump, fan, LED, moisture, reservoir, leak) per §8.5.
- [ ] Strain-relief and drip-loop callouts at each removable module.

## Acceptance criteria

- Every connector has polarity + voltage labeled (spec §15.5 M4-03).
- Harness labels match [WI-ME-07](../04-mechanical/WI-ME-07-cable-tube-routing.md) cable channel.
