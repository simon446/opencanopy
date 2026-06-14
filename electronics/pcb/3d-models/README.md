<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# 3D models (for mechanical CAD + EDA)

Checked-in **STEP / WRL** models of the off-the-shelf parts, so the mechanical track can drop real
geometry into the enclosure CAD and any EDA tool can render the board. Tool-neutral (STEP = solid for
CAD, WRL = mesh for fast render).

## Models

| Part | BOM ref | Files | Size | Source |
|---|---|---|---|---|
| **WS2812B-2020** addressable LED (status indicators) | `LED_STATUS` (×5 on PCB2) | `WS2812B-2020_C965555.step`, `.wrl` | 2.0 × 2.0 × 0.8 mm | LCSC **C965555** / EasyEDA, fetched with [`easyeda2kicad`](https://github.com/uPesy/easyeda2kicad.py) |

The WS2812B-2020 STEP is a real BREP solid (SolidWorks AP214 origin, ~300 faces), not a placeholder
box — it matches the `LED-SMD_4P-L2.0-W2.0-H0.8` datasheet outline. There are **5** of these on the
front-panel status board (PCB2, [WI-EE-09](../../analysis/WI-EE-09-status-led-board.md)); mechanical
also needs the **PCB2 outline** to place them behind the diffuser.

## Provenance & licensing

The WS2812B-2020 model is the **manufacturer/EasyEDA-provided geometry** for LCSC part `C965555`,
retrieved via `easyeda2kicad`. It is redistributed here only to build/visualise this open-hardware
design; the model geometry remains the vendor's and is subject to LCSC/EasyEDA's model terms — treat it
as a reference/visualisation model, not a re-licensable asset. (If a fully license-clean model is
needed, the 2.0 × 2.0 × 0.8 mm 4-pad outline is trivial to regenerate from the datasheet.)

Reproduce:

```sh
pip install easyeda2kicad
python3 -m easyeda2kicad --full --lcsc_id=C965555 --output=WS2812B-2020
```

## What's *not* here (and why)

- **Grow light (`LED_PANEL`)** — it's an **LED board/module**, not a discrete diode we place, so its
  geometry is the **board outline + heatsink** from the chosen vendor (a rectangle for CAD), paired
  with the WI-ME-05 heatsink — not a single-diode model.
- **Pump, LED driver (LDD-H), connectors** — big-geometry items mechanical designs around; their STEP
  sources are listed in [`../../bom/component-sourcing.md`](../../bom/component-sourcing.md) §5 (Mean Well
  STEP download, TOPSFLO on request, JST/Amass via SnapEDA). Add them here the same way as they're picked.
