# WI-ME-07 — Cable & tube routing

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-07 |
| Depends on | WI-ME-03, WI-ME-04 |
| Spec refs | §8.5 |
| Status | Done |

## Objective

Design cable channels and tube routing that separate wiring from water, force drip loops before
electronics, and keep tubing inspectable.

## Deliverables

- [x] CAD for the cable channel with a **central divider** separating the wire pocket from the tube
      pocket (`routing.build_cable_channel`).
- [x] Drip loops + grommets at every electronics-bay entry (raised collars + hook posts in the dry-bay
      floor); no cable enters from directly below without one (§8.5, 20 mm min bend radius).
- [x] Strain-relief tie slots down the channel; `tube-clip`/`cable-clip`/`sensor-clip` at removable
      modules; tube pocket leaves tubing visible/inspectable.
- [x] Labels `pump / fan / led / moisture / reservoir / leak` (the canonical §8.5 set, in
      `params.HARNESS_LABELS`) to match [WI-EE-05](../03-electronics/WI-EE-05-harness-pinout.md).

## Acceptance criteria

- Drip loops and clips present (spec §15.6 M5-07). ✅
- Routing matches the harness table labels. ✅ **Verified against the landed
  `electronics/wiring/harness-table.csv`**: connectors `J_PUMP / J_FAN / J_LED / J_MOIST / J_RES /
  J_LEAK` match the six labels; the table's `J_PWR` 24 V input enters through its own grommet + drip
  loop. Field connectors are keyed JST VH/XH/PH, cleared by the channel pockets and grommet bores.
