# WI-ME-06 — Fan mount

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-06 |
| Depends on | WI-ME-01 |
| Spec refs | §7.4, §8.7 |
| Status | **Obsolete for V1 — superseded by [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md) (no fan in V1)** |

> **V1 has no fan** (maintainer-approved divergence; see ECO-001). This work-item is
> retained for history and as a possible future option only — the controller keeps an
> **unpopulated (DNP)** 4-pin fan-drive provision so a fan can be added later without a
> respin, but no fan mount is printed or fitted for V1. The fan-guard child-safety risk
> (**S18**) is moot with no blades. Canopy heat is handled by the **passive LED heatsink**
> in [WI-ME-05](WI-ME-05-light-mount.md) (the 60 W V1 LED is comfortably passively cooled —
> ECO-001 §3); canopy air circulation, the fan's other job, is flagged to the plant-science
> track in ECO-001 §5. The deliverables below are **not built for V1**.

## Objective

Design a guarded, vibration-isolated fan mount that provides gentle canopy circulation without
blasting seedlings or coupling noise into the frame.

## Deliverables

- [x] CAD/STL for a 92 mm fan mount (`fan_mount.build_fan_mount`) with 4 rubber-grommet bores and an
      **integral required guard** (hub + concentric rings + spokes).
- [x] Placement high at the rear, offset +80 mm in X → circulation across the canopy, not a direct
      drying stream at the seedling (§7.4).
- [x] Optional removable intake-filter recess (shallow lip on the intake face).
- [x] Solid 4 mm plate with ribbed guard avoids resonant thin panels; grommet isolation decouples the
      fan from the frame (§8.7).

## Acceptance criteria

**N/A for V1** — no fan is fitted (ECO-001). The criteria below applied to the original
fan-bearing design and are retained for history / a future fan option:

- Guarded and isolated (spec §15.6 M5-06). *(historical)*
- Supports the noise targets validated in [WI-QA-04](../05-validation-qa/WI-QA-04-acoustic.md).
  *(historical — with no fan, the dominant acoustic source is removed; WI-QA-04 is for the
  plant-science / QA tracks to re-baseline for a fan-less unit.)*
