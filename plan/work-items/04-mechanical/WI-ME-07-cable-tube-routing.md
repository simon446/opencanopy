# WI-ME-07 — Cable & tube routing

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-07 |
| Depends on | WI-ME-03, WI-ME-04 |
| Spec refs | §8.5 |
| Status | Done — **re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md)** |

> **🔄 Re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md):** **no water tubing** (no pump) and
> **no in-base electronics**. Power (**USB-C**) enters at the **top block**; the only base↔top run is a
> **low-voltage sensor bundle** (moisture, reservoir level, leak/status-LED) up the **rear flat of the
> right pillar**, through a **sealed grommet** with strain relief, to the top controller. No cable
> crosses the open water. Harness labels drop `pump`/`fan`; the `J_PWR` input becomes USB-C at the top.
> The dual wire/tube channel below is the superseded design.

## Objective

Route the **low-voltage sensor bundle** up the rear of the right pillar (sealed grommet + strain
relief) to the top controller; **USB-C power enters at the top block**. No water tubing (no pump),
no cable across the open water.

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
