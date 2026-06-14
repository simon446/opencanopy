# electronics/

PCB, wiring, BOM, and electrical verification for OpenCanopy. Owned by the **Electronics** track
(spec §7, §11, §16).

## Layout

- `pcb/` — custom controller board (designed with the headless tscircuit flow; KiCad retired — [ECO-002](analysis/ECO-002-pcb-toolchain.md)).
  - `netlist/` — **the schematic as data**: `controller_netlist.py` (source of truth — every part +
    pin-level net + ERC checker), generated `controller.net` (optional interchange) and `controller-netlist.csv`.
  - `programmatic/` — **headless code→PCB→Gerbers** flow (tscircuit): the generator, the board, the
    per-subsystem render blocks, and `out/` (draft Gerbers/PnP/BOM + KiCad-PCB bridge + previews).
  - `3d-models/` — checked-in STEP/WRL models of off-the-shelf parts (e.g. the WS2812B-2020 status LED) for mechanical CAD.
  - `fabrication/` — fab notes + stackup (the fab package itself lives in `programmatic/out/`).
- `wiring/` — system harness.
  - `wiring-diagram.svg` — full system wiring diagram.
  - `harness-table.csv` — connector/pin/wire/gauge harness table.
  - `connector-spec.md` — chosen connectors + mating cable-side parts (handoff for mechanical).
- `bom/` — bill of materials.
  - `bom.csv` — primary BOM (must satisfy the §16 constraint list).
  - `alternates.csv` — approved alternate parts.
  - `component-sourcing.md` — real buyable picks for pump / grow light / status LEDs / cables, with
    3D-model sources and the firmware/mechanical/project flags they raise.
- `test/` — electrical verification.
  - `bringup.md` — bench bring-up procedure (WI-EE-08).
  - `pcb-verification.md` — trace/current and thermal verification report (WI-EE-06).
  - `hil-fixture.md` — HIL fixture design + automated fault-test matrix (WI-EE-08).
  - `poc-logs/` — breadboard PoC bench-log templates (WI-EE-01).
- `analysis/` — engineering design/analysis artifacts.
  - thermal budget model + script (WI-EE-10), power budget (WI-EE-02), component PoC plan (WI-EE-01),
    schematic capture + pin map (WI-EE-03), PCB layout design (WI-EE-04) + `design-rules.md` (net
    classes/DRC), status-LED board (WI-EE-09), trace-width calculator, `ECO-001-fan-removal.md`.
    The board layout is generated from the `pcb/netlist/` netlist by the tscircuit flow + these docs.

## Key constraints

ESP32-S3 controller, 24 VDC external certified PSU, electronics in the upper dry bay, locking/keyed
connectors. The grow-light BOM entry must carry real photometric/thermal/electrical data (spec §16.3);
`scripts/bom_check.py` enforces this in CI.
