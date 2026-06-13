# OpenCanopy — Tabletop Automated Pepper Grower

Open-source, zero-config, offline-first tabletop grow unit for a single hot pepper plant.

OpenCanopy is an open-hardware project: a compact, living-room-friendly appliance that keeps one hot
pepper plant healthy indoors with minimal user involvement. It combines a broad-spectrum horticultural
LED, automated pump irrigation with soil-moisture sensing, climate/VPD monitoring, water-safety
interlocks, and a simple LED status interface — all driven by deterministic, fully local **Rust**
firmware (`no_std`, `esp-hal`) on an ESP32-S3.

📖 **Docs site:** <https://simon446.github.io/opencanopy/> · 🧩 **Contributing:** [`CONTRIBUTING.md`](CONTRIBUTING.md) · ⚖️ **Licensing:** [`LICENSES/`](LICENSES/)

The project is built **work item by work item**, organized by engineering discipline. The full
engineering plan and the per-discipline work-item breakdown live in `plan/`, which is independently
version-controlled and **gitignored** by this repo — it is the design source, not a build artifact.

## Project status

- **Repository skeleton, licensing, scope lock, risk register, and CI are in place** (the Project &
  Repo track is complete) — every discipline has a home and quality gates run on each PR.
- **Firmware language: Rust** (`no_std` + `esp-hal`), with a host-testable `control` crate and
  `cargo fmt`/`clippy`/`test` gates in CI.
- The remaining discipline tracks (plant science, firmware, electronics, mechanical, validation, docs)
  follow the work-item breakdown in `plan/`. A spec design review (§23) added pre-order modeling and
  bench-characterization gates and an n=2 parallel grow trial.

## Where things live

| What | Path |
|---|---|
| Documentation (rendered) | <https://simon446.github.io/opencanopy/> |
| Documentation (source) | [`docs/`](docs/) — requirements, scope, risk register, guides |
| Contributing & workflow | [`CONTRIBUTING.md`](CONTRIBUTING.md); issue/PR templates in [`.github/`](.github/) |
| Licensing (per asset type) | [`LICENSES/`](LICENSES/) |
| Implementation subtrees | [`firmware/`](firmware/), [`electronics/`](electronics/), [`mechanical/`](mechanical/), [`validation/`](validation/), [`scripts/`](scripts/) |
| Work items & engineering spec (design source) | `plan/` — separate, gitignored repo |

## Scope

- **In V1:** one plant, fixed hot-pepper profile, dimmable full-spectrum LED with automated schedule,
  substrate moisture sensing, automated pump watering, reservoir-level + leak safety, temp/humidity
  sensing with VPD, circulation fan, 5-LED status interface, local deterministic firmware, and open
  hardware/firmware/CAD with simulation, hardware-in-loop, and grow-trial validation.
- **Not in V1:** camera/AI plant diagnosis, cloud control, automatic pH/EC dosing, multi-plant
  support, required mobile app, or AC mains inside the unit.

See [`docs/scope.md`](docs/scope.md) for the locked non-goals and [`docs/product-requirements.md`](docs/product-requirements.md)
for the locked physical/hardware envelope.

## Repository layout

The standard open-hardware structure is in place; each subtree has a README describing its purpose:

- `firmware/` — Rust workspace: `control/` (host-testable `no_std` logic), `controller/` (`esp-hal`
  binary), `sim/`, `hil/`.
- `electronics/` — KiCad PCB, wiring, BOM, and electrical verification.
- `mechanical/` — CAD/STEP/STL, drawings, and print settings.
- `docs/` — requirements, scope, risk register, and the guides that ship with V1.
- `validation/` — test plans, measurements, and grow-trial records.
- `scripts/` — tooling (`bom_check.py` §16.3 grow-light gate, `stl_check.py`, calculators).

Full layout: spec §14 (in `plan/`).

## Release gate

V1 is tagged only when the acceptance criteria in the V1 spec (§21, in `plan/`) are all met, as proven
by the Validation & QA track — including an **n=2 parallel grow trial** (design-review decision, §23).

## License

Per-asset open-source licensing — **Apache-2.0** firmware/scripts, **CERN-OHL-S v2** hardware/mechanical,
**CC BY 4.0** docs. See [`LICENSES/`](LICENSES/) and its [mapping note](LICENSES/README.md).
