# docs/

Project documentation for OpenCanopy. These are the human-readable references that ship with V1.

Per spec §14.1, this subtree holds:

| File | Owner track | Purpose |
|---|---|---|
| `product-requirements.md` | Project & Repo | Locked physical envelope and hardware decisions (the design contract). |
| `scope.md` | Project & Repo | V1 scope and explicit non-goals. |
| `risk-register.md` | Project & Repo | Engineering and safety risks, owners, and mitigations. |
| `plant-profile-hot-pepper.md` | Plant Science | Lifecycle stages, setpoints, DLI, watering, VPD, nutrients. |
| `safety.md` | Documentation | Water/electrical, thermal, food-contact, child/pet safety. |
| `assembly.md` | Documentation | Build/assembly guide. |
| `calibration.md` | Documentation | Moisture and pump calibration procedure. |
| `operation.md` | Documentation | Day-to-day operation and grow guide. |
| `troubleshooting.md` | Documentation | Common faults and fixes. |
| `validation-plan.md` | Validation & QA | Dry/wet/grow-trial acceptance plan. |
| `led-status-legend.md` | Documentation | Front-panel LED meaning (state + blink pattern). |
| `maintenance.md` | Documentation | User and developer maintenance schedule. |
| `references.md` | Documentation / Plant Science | Cited sources behind the design decisions. |

The minimum documentation set required before a V1 tag is listed in spec §14.3.

## Engineering & design references (rendered)

Published design docs with renders, one per hardware track:

| Doc | Track | What |
|---|---|---|
| [`electronics-design.md`](electronics-design.md) | Electronics | Controller board: per-subsystem schematics + 3D/board renders, safety design, connectors, BOM, the headless netlist→Gerbers flow. |
| [`mechanical-build.md`](mechanical-build.md) | Mechanical | v1 product-model renders (CAD), joints, conduit, collision/physics checks. |
| [`firmware-api.md`](firmware-api.md) | Firmware | Pointer to the published Rust API docs (`/api`). |
