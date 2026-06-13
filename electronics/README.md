# electronics/

PCB, wiring, BOM, and electrical verification for OpenCanopy. Owned by the **Electronics** track
(spec §7, §11, §16).

## Layout

- `pcb/` — custom controller board.
  - `kicad/` — KiCad source project (schematic + layout).
  - `gerbers/` — fabrication Gerbers + drill files (generated).
  - `fabrication/` — fab notes, stackup, assembly drawings, pick-and-place.
  - `ibom/` — interactive HTML BOM output.
- `wiring/` — system harness.
  - `wiring-diagram.svg` — full system wiring diagram.
  - `harness-table.csv` — connector/pin/wire/gauge harness table.
- `bom/` — bill of materials.
  - `bom.csv` — primary BOM (must satisfy the §16 constraint list).
  - `alternates.csv` — approved alternate parts.
- `test/` — electrical verification.
  - `bringup.md` — bench bring-up procedure (WI-EE-08).
  - `pcb-verification.md` — trace/current and thermal verification report (WI-EE-06).
  - `hil-fixture.md` — HIL fixture design + automated fault-test matrix (WI-EE-08).
  - `poc-logs/` — breadboard PoC bench-log templates (WI-EE-01).
- `analysis/` — engineering design/analysis artifacts.
  - thermal budget model + script (WI-EE-10), power budget (WI-EE-02), component PoC plan (WI-EE-01),
    schematic capture + pin map (WI-EE-03), PCB layout design (WI-EE-04), status-LED board (WI-EE-09),
    trace-width calculator. KiCad source is entered from these (`pcb/kicad/`).

## Key constraints

ESP32-S3 controller, 24 VDC external certified PSU, electronics in the upper dry bay, locking/keyed
connectors. The grow-light BOM entry must carry real photometric/thermal/electrical data (spec §16.3);
`scripts/bom_check.py` enforces this in CI.
