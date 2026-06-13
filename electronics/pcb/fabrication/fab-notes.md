<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# WI-EE-07 — Fabrication package

**Status:** BOM (`bom.csv` + `alternates.csv`) complete and **passing `scripts/bom_check.py`** (incl.
`--strict`); fab notes + stackup specified. **Gerbers, drill, pick-and-place, and interactive BOM are
generated from the KiCad PCB source and are pending those source files.**
**Spec refs:** §14.1, §16.1, §16.3.

## 1. What's in the package

| Artifact | Path | State |
|---|---|---|
| BOM | [`bom/bom.csv`](../../bom/bom.csv) | ✔ complete, passes §16.3 check |
| Alternates | [`bom/alternates.csv`](../../bom/alternates.csv) | ✔ complete |
| Fab notes / stackup | this file | ✔ |
| Gerbers + drill | [`pcb/gerbers/`](../gerbers/) | ⏳ `kicad-cli pcb export gerbers/drill` |
| Pick-and-place | [`pcb/fabrication/`](.) | ⏳ `kicad-cli pcb export pos` |
| Interactive BOM | [`pcb/ibom/`](../ibom/) | ⏳ InteractiveHtmlBom plugin |

The three ⏳ artifacts are mechanical exports from `pcb/kicad/` once the board is laid out
([WI-EE-04](../../analysis/WI-EE-04-pcb-layout.md)); the commands are listed in §3 so generation is a
single CI/script step.

## 2. Fabrication spec (controller PCB, PCB1)

| Parameter | Value |
|---|---|
| Layers | 4 (signal / GND / power / signal) — [stackup](../../analysis/WI-EE-04-pcb-layout.md#1-stackup) |
| Copper | 1 oz outer (high-current paths as filled pours, [WI-EE-06](../../test/pcb-verification.md)) |
| Finish | ENIG (fine-pitch QFN/DFN parts) or HASL |
| Min track/gap | per DRC net classes; power nets widened ([WI-EE-04 §4](../../analysis/WI-EE-04-pcb-layout.md)) |
| Min drill | 0.3 mm |
| Soldermask / silk | both sides; silk carries connector name/polarity/voltage/warnings (§7.9) |

Status LED PCB (PCB2): 2-layer, same finish, small front-panel board ([WI-EE-09](../../analysis/WI-EE-09-status-led-board.md)).

## 3. Generation commands (run once KiCad source exists)

```sh
# from electronics/pcb/kicad/
kicad-cli pcb export gerbers  -o ../gerbers/  opencanopy-ctrl.kicad_pcb
kicad-cli pcb export drill    -o ../gerbers/  opencanopy-ctrl.kicad_pcb
kicad-cli pcb export pos      -o ./           opencanopy-ctrl.kicad_pcb   # pick-and-place
# interactive BOM via the InteractiveHtmlBom plugin -> ../ibom/
```

These slot into CI alongside the existing `bom_check.py` gate (spec §10.5).

## 4. BOM compliance (§16.1, §16.3)

- All §16.1 core-electronics rows present: MCU, temp/RH, moisture, reservoir, leak, fan driver, pump
  driver (logic-level + flyback + current sense), LED-driver interface, status LEDs, **battery-backed
  RTC** (DR-05), certified 24 V PSU, locking/keyed connectors.
- **Grow light** carried as `LIGHT-CANDIDATE-60W` with full §16.3 data (power, PPFD/PPF, dimming,
  horticultural spectrum, thermal mounting, cert) so the gate runs — **status `CANDIDATE-DR01-HOLD`:
  not ordered** until the [WI-PL-06](../../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md)
  photometric half of DR-01 passes (thermal half done, [WI-EE-10](../../analysis/WI-EE-10-thermal-budget-model.md)).
  The 100 W full-yield candidate is in `alternates.csv`.
- `python3 scripts/bom_check.py --strict electronics/bom/bom.csv` → **PASS**.
