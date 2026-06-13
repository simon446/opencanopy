# Print settings (spec §8.3, §12.2, §16.2)

Recommended material and slicer settings for the OpenCanopy printed parts. Material
choice follows the §8.3 zone rules: **no PLA** anywhere near LED heat or persistent
humidity.

## Material by part

| Part (STL) | Zone | Material | Why |
|---|---|---|---|
| `leak-tray`, `pot-tray`, `pump-clip` | wet | **PETG** | Food-/water-adjacent, tough, hydrolysis-OK |
| `reservoir` (if printed, else bought) | wet | **PETG** (food-safe) | Hand-cleanable, no PLA in humidity |
| `electronics-dry-bay`, `-lid` | dry/upper | **PETG** or **ASA** | Splash protection, dimensional stability |
| `cable-channel`, `cable-clip`, `tube-clip`, `sensor-clip` | mixed | **PETG** | Toughness, repeated flexing |
| `light-mount` | LED-adjacent | **ASA** or **ABS** (or heat-proven PETG) | Sits near the driver/LED heat (§8.3) |
| `fan-mount` | grow | **PETG** or **ASA** | Vibration, mild warmth |
| `status-diffuser` | front trim | **frosted PETG / acrylic** | Translucent light pipe (§3.4) |
| decorative trim | dry/front | wood / bamboo / wood-fill PLA **only away from heat & moisture** | Aesthetic (§3.4) |

Fasteners near water: **stainless steel** (§16.2). Heat-set inserts: **M3 brass**.

## Slicer baseline (0.4 mm nozzle)

| Setting | Value | Notes |
|---|---|---|
| Layer height | 0.20 mm | 0.28 mm OK for `leak-tray`/`frame brackets` |
| Walls / perimeters | 3 (≥1.2 mm) | 4 for `light-mount`, `fan-mount` (load/heat) |
| Top/bottom layers | 5 | watertightness for trays |
| Infill | 30 % gyroid | 40–50 % for load-bearing mounts |
| PETG temp | 235–245 °C / bed 75–85 °C | per filament |
| ASA/ABS temp | 240–260 °C / bed 95–110 °C, enclosure | manage warping |
| Cooling | PETG 30–50 %, ASA/ABS low/off | |
| Seam | rear / hidden faces | |

## Orientation & supports

- **Trays (`leak-tray`, `pot-tray`):** print open-side up; no supports; brim for ASA.
- **`light-mount`:** arms flat on the bed; the eyelets and bosses print without supports.
- **`fan-mount`:** guard side up; the guard rings + spokes bridge cleanly at 0.2 mm.
- **`cable-channel`:** print standing or laid on its back; the open mouth needs no support.
- **`electronics-dry-bay`:** floor down; the grommet collars and standoffs print upright.
- **Coupons:** flat as exported; print the whole set first (see `fit-tests.md`).

## Dimensional fits (validated by coupons → `fit-tests.md`)

| Fit | Param | Nominal |
|---|---|---|
| Snap-fit interference | `CLEAR_SNAP` | 0.20 mm |
| Sliding rail (reservoir) | `CLEAR_SLIDE` | 0.30 mm |
| Press / heat-set bore | `CLEAR_PRESS` / `HEATSET_M3_DIA` | 0.10 mm / 4.0 mm |
| Tube clip grip (8 mm OD) | `TUBE_CLIP_GAP` | 0.4 mm |

> All clearances live in `cad/source/opencanopy/params.py`. After running the
> coupons, set the values that fit **your** printer/material there and re-run
> `build.py` so every part inherits the calibrated fit.

## 40 °C survival (§12.2 acceptance)

Parts near the LED/driver and in the warm bay must not deform at 40 °C. PETG (Tg
≈ 80 °C) and ASA/ABS clear this comfortably; PLA (Tg ≈ 60 °C, creeps far lower
under load) does **not** and is excluded from heat/humidity zones. The coupon set is
heat-soaked at 40 °C for 4 h as the gate (see `fit-tests.md`).
