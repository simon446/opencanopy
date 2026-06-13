# WI-ME-06 — Fan mount

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-06 |
| Depends on | WI-ME-01 |
| Spec refs | §7.4, §8.7 |
| Status | Done |

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

- Guarded and isolated (spec §15.6 M5-06). ✅
- Supports the noise targets validated in [WI-QA-04](../05-validation-qa/WI-QA-04-acoustic.md). ✅
  (Grommet isolation + non-resonant plate are the mechanical enablers for that QA test.)
