# validation/

Test plans, measurement data, and trial records for OpenCanopy. Owned by the **Validation & QA**
track (spec §13). This track owns proving each V1 release criterion in spec §21.

## Layout

- `test-plans/` — dry-run, wet-run, thermal, acoustic, fault-injection, and grow-trial plans.
- `logs/` — captured firmware/device logs from validation runs.
- `photos/` — trial photos (CC BY 4.0 **only after** personal info is removed — see `LICENSES/`).
- `ppfd-measurements/` — PPFD/PPF maps and DLI calculations for the grow light.
- `thermal/` — canopy/LED/driver thermal measurements.
- `acoustic/` — fan/pump noise measurements.
- `grow-trials/` — end-to-end real-plant trial records.

## Release gate

V1 is tagged only when every acceptance criterion in spec §21 is met and documented here.
