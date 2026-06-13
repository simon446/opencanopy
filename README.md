# OpenCanopy — Tabletop Automated Pepper Grower

Open-source, zero-config, offline-first tabletop grow unit for a single hot pepper plant.

OpenCanopy is an open-hardware project: a compact, living-room-friendly appliance that keeps one hot
pepper plant healthy indoors with minimal user involvement. It combines a broad-spectrum horticultural
LED, automated pump irrigation with soil-moisture sensing, climate/VPD monitoring, water-safety
interlocks, and a simple LED status interface — all driven by deterministic, fully local firmware.

The project is built **work item by work item**, organized by engineering discipline. The full
engineering plan and the per-discipline work-item breakdown live in `plan/`, which is independently
version-controlled and **gitignored** by this repo — it is the design source, not a build artifact.

## Where things live

| What | Path |
|---|---|
| Work items, by discipline track | `plan/work-items/` (start at `plan/work-items/README.md`) |
| V1 engineering spec | `plan/tabletop_pepper_grower_v1_spec_v1_1.md` |
| Build run-sequence | `plan/IGNITION.md` |
| Implementation (firmware, electronics, mechanical, …) | this repo, created as the build progresses |

## Scope

- **In V1:** one plant, fixed hot-pepper profile, dimmable full-spectrum LED with automated schedule,
  substrate moisture sensing, automated pump watering, reservoir-level + leak safety, temp/humidity
  sensing with VPD, circulation fan, 5-LED status interface, local deterministic firmware, and open
  hardware/firmware/CAD with simulation, hardware-in-loop, and grow-trial validation.
- **Not in V1:** camera/AI plant diagnosis, cloud control, automatic pH/EC dosing, multi-plant
  support, required mobile app, or AC mains inside the unit.

## Repository layout

The build populates the standard open-hardware structure: `firmware/`, `electronics/`, `mechanical/`,
`docs/`, `validation/`, and `scripts/`. See `plan/work-items/00-project-setup/` for the skeleton and
`plan/tabletop_pepper_grower_v1_spec_v1_1.md` §14 for the full layout.

## Release gate

V1 is tagged only when the acceptance criteria in `plan/tabletop_pepper_grower_v1_spec_v1_1.md` §21
are all met, as proven by the Validation & QA track.

## License

Per-asset open-source licensing (CERN-OHL-S for hardware/mechanical, Apache-2.0 for firmware, CC BY
for docs); see `plan/work-items/00-project-setup/WI-PS-02-licensing.md`.
