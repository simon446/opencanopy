<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# WI-EE-07 — Fabrication package

**Status:** BOM (`bom.csv` + `alternates.csv`) complete and **passing `scripts/bom_check.py`** (incl.
`--strict`); fab notes + stackup specified. A **headless programmatic draft** fab package (Gerbers +
drill + PnP + BOM) is generated from the netlist by the tscircuit flow ([ECO-002](../../analysis/ECO-002-pcb-toolchain.md);
KiCad retired). A **fab-ready** package needs real footprints + a reviewed layout (see
[`programmatic/README.md`](../programmatic/README.md)).
**Spec refs:** §14.1, §16.1, §16.3.

## 1. What's in the package

| Artifact | Path | State |
|---|---|---|
| BOM | [`bom/bom.csv`](../../bom/bom.csv) | ✔ complete, passes §16.3 check |
| Alternates | [`bom/alternates.csv`](../../bom/alternates.csv) | ✔ complete |
| Fab notes / stackup | this file | ✔ |
| Gerbers + drill + PnP + BOM | [`pcb/programmatic/out/controller.gerbers.zip`](../programmatic/out/controller.gerbers.zip) | ◑ **draft** (placeholder IC/connector footprints; autorouter-grade) |
| Board as KiCad PCB | [`pcb/programmatic/out/controller.kicad_pcb`](../programmatic/out/controller.kicad_pcb) | ◑ draft (optional interchange) |

The draft package is produced by [`pcb/programmatic/build.sh`](../programmatic/build.sh) (a single
script step). It is real fab-FORMAT data but not fab-ready — see §3.

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

## 3. Generation (headless, no GUI)

```sh
electronics/pcb/programmatic/build.sh     # netlist -> tscircuit -> autoroute -> out/controller.gerbers.zip
```

This produces the draft Gerbers/drill/PnP/BOM + a `controller.kicad_pcb`. **Before fab**, the draft
needs: (1) real footprints for the ICs, connectors, and the ESP32-S3 module (the draft uses `pinrowN`
placeholders), and (2) a reviewed power/analog/thermal placement per
[`design-rules.md`](../../analysis/design-rules.md) — the autorouter does not encode it. The committed
electrical gate (`controller_netlist.py --selftest`) runs in CI alongside `bom_check.py` (spec §10.5).

## 4. BOM compliance (§16.1, §16.3)

- All §16.1 core-electronics rows present: MCU, temp/RH, moisture, reservoir, leak, pump driver
  (logic-level + flyback + current sense), LED-driver interface, status LEDs, **battery-backed RTC**
  (DR-05), certified 24 V PSU, locking/keyed connectors. *(Fan driver `D3`/`CN_FAN` kept **DNP** —
  no fan in V1, [ECO-001](../../analysis/ECO-001-fan-removal.md).)*
- **Grow light** carried as `LIGHT-CANDIDATE-60W` (panel) with full §16.3 data (power, PPFD/PPF,
  dimming, horticultural spectrum, thermal mounting, cert) so the gate runs — **status
  `CANDIDATE-DR01-PASS`**: both halves of DR-01 now pass
  ([WI-PL-06](../../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md) photometric +
  [WI-EE-10](../../analysis/WI-EE-10-thermal-budget-model.md) thermal). PL-06 shows a **panel** meets
  ≥0.6 uniformity at 150 mm; a bar needs ≥200–225 mm. The final procurement pick is outstanding; the
  100 W full-yield bar and a compact bar alternative are in `alternates.csv`.
- `python3 scripts/bom_check.py --strict electronics/bom/bom.csv` → **PASS**.
