# OpenCanopy V1 — Product Requirements (Locked)

**Status:** Locked (M0-04, M0-05) · **Owner:** Project & Repo track · **Spec refs:** §3.3, §4.1, §4.2, §20

This document freezes the physical envelope and the core hardware decisions for OpenCanopy V1. It is
the **design contract** every other track designs against. The values below are decisions, not
ranges-for-discussion.

> **Change control:** Any change to a locked value after this point requires a corresponding entry in
> [`risk-register.md`](risk-register.md) and sign-off from the Project & Repo track. Downstream tracks
> may treat these numbers as fixed.

---

## 1. Physical envelope (locked)

Two build variants are defined. **Compact V1 is the default build**; the full-yield variant is an
optional larger configuration. Every dimension below is a hard limit, not a target band.

| Parameter | Compact V1 (default) — committed nominal | Compact V1 — allowed band | Full-yield variant — hard max |
|---|---:|---:|---:|
| Width (W) | **480 mm** | 450–500 mm | **550 mm** |
| Depth (D) | **320 mm** | 300–350 mm | **400 mm** |
| Height (H) | **700 mm** | 650–750 mm | **850 mm** |
| Pot volume | **10 L** | 8–10 L | **19 L** (12–19 L band) |
| Reservoir volume | **4 L** | 2.5–4 L | **6 L** (4–6 L band) |
| Useful canopy footprint | 300–400 mm W × 220–300 mm D | — | 400–450 mm W × 300–350 mm D |
| Plant count | **1** | — | **1** |
| Wet electronics exposure | **None** | — | **None** |

**Hard maximums (no build may exceed):** W ≤ 550 mm, D ≤ 400 mm, H ≤ 850 mm, pot ≤ 19 L, reservoir ≤ 6 L.

**Rationale for the committed compact nominals:**

- Pot locked at the **top** of the compact band (10 L) to maximize root volume — directly mitigates
  risk #2 (too-small pot / root volume) while staying compact.
- Reservoir locked at the **top** of the compact band (4 L) to lengthen the unattended interval
  without enlarging the wet bay.
- W/D/H nominals sit mid-band, leaving margin for the wet/dry bay split (§2) and light clearance.

## 2. Architecture & safety envelope (locked)

- **Two-zone layout (current):** electronics and water both live in the **base**, in two
  side-by-side compartments separated by an **additional isolating wall** — the **wet zone**
  (reservoir + pump) on one side and the sealed **dry electronics** compartment **beside** it on the
  other. Electronics are horizontally walled off from the water; only low-voltage wires cross the wall.
  No water path can overflow into electronics (§17.1).
- **No AC mains inside the unit.** Power is an **external certified 24 VDC PSU** only (§17.1).
- **Open-frame / non-enclosed default** (§3.6): not a sealed grow chamber. Room temperature is assumed
  **22–25 °C**. Firmware does **not** maintain an independent air-temperature setpoint. No humidifier,
  heater, refrigeration, or sealed chamber in V1.

## 3. Controller & user interface (locked)

| Decision | Locked value |
|---|---|
| MCU | **ESP32-S3** (dev module or custom PCB module) |
| Display | **None** — no LCD/e-ink in V1 |
| User controls | **Hidden service/reset control only** (factory reset / grow-cycle reset / test mode); no user-facing screen |
| Status interface | **5 front status LEDs** (Water, Moisture, Light, Climate, System); state conveyed by color **and** blink pattern + position (do not rely on color alone) |
| Connectivity | **Optional, offline-first.** Serial/USB dev interface allowed; web/MQTT/Home Assistant telemetry allowed but **must not be required** |

## 4. Actuators (locked)

| Subsystem | Locked decision |
|---|---|
| Pump | **Brushless DC submersible centrifugal**, quiet; logic-level MOSFET drive with flyback/protection; daily-max + runtime-max limits; fails **off** on MCU reset/brownout (§17.1) |
| Fan | **80 or 92 mm quiet PWM** PC-class fan, tach input preferred; for gentle canopy airflow / heat dispersion only — **not** active cooling below ambient |
| Light | **50–80 W dimmable full-spectrum white** horticultural LED for compact V1; **100 W only** for the full-yield variant. Dimmable via PWM/0–10 V matching the selected driver |

**Grow-light data requirement (§16.3):** a candidate light may be specified in the BOM **only** if it
provides actual power draw, a PPF/PPFD map, dimming method, full-spectrum/horticultural spectrum data,
thermal mounting information, and electrical certification/safety data where available. Lights
advertising only lumens, "equivalent watts," or vague "red/blue plant lamp" claims are rejected.
This is enforced in CI by `scripts/bom_check.py`.

## 5. Sensor set (locked)

Required V1 sensors (§7.5, §20): **temp/RH** (SHT31/SHT4x-class, I²C), **soil moisture** (capacitive,
corrosion-resistant, replaceable), **reservoir low-level**, **leak/flood**, and **fan tach**. VPD is
**computed** from temp/RH, not separately sensed.

## 6. Expansion headers (provisioned, not populated)

V1 PCB/mechanical may include connectors/pads for future modules but must **not** be blocked on them
(§4.3): camera, PAR/light sensor, load cell under pot, EC sensor, pH sensor, CO₂ sensor, e-ink status,
external telemetry. None of these are functional in V1 — see [`scope.md`](scope.md).

## 7. Traceability

| Locked item | Spec source |
|---|---|
| Footprint, pot, reservoir | §3.3, §20 |
| MCU / display / controls | §3.5, §7.1, §20 |
| Pump / fan / light | §7.2–§7.4, §16, §20 |
| Sensor set | §7.5–§7.6, §20 |
| Open-frame / power / two-zone | §3.6, §7.8, §17 |
| Expansion headers | §4.3 |

## 8. Rationale & research basis

These decisions are grounded in extension/research guidance (full list in
[`references.md`](references.md); design detail in spec §2 and §5). The key "whys":

- **Open-frame, room 22–25 °C, no enclosure.** A 22–25 °C indoor room is adequate for hot peppers, so
  the cost/complexity of a sealed, actively climate-controlled chamber isn't justified for V1; the
  device manages only LED heat, airflow, watering, and warnings (§2.3, §3.6). [R4, R5]
- **Pot locked at 10 L (top of the compact band).** Container plants dry fast and root volume drives
  plant health and yield; maximizing root volume within the compact envelope directly mitigates the
  "too-small pot" risk (§3.3, risk-register R2). [R7, R8, R9]
- **Light specified in DLI/PPFD, broad-spectrum white, dimmable.** Fruiting peppers are high-light:
  the target is DLI ≈ 20–25 mol·m⁻²·day⁻¹ and ≥ ~400 µmol·m⁻²·s⁻¹ PPFD across the canopy, achievable
  in a tabletop footprint only with a constrained canopy, a real PPFD map, and pruning. Lumens / "plant
  lamp" claims are rejected (§2.1, §7.2, §16.3). [R1, R2, R3, R14]
- **Consistent-moisture watering with safety caps.** Steady moisture supports fruit retention and
  reduces blossom-end rot, but waterlogging harms roots — hence closed-loop watering with daily/runtime
  caps and leak lockout (§5.6, §17; risk-register R3). [R5, R8, R15, R16]
- **VPD computed from temp/RH, not RH alone.** VPD is the physiologically meaningful driver of
  transpiration and disease pressure (§5.4). [R13]

See [`references.md`](references.md) for the cited sources and [`scope.md`](scope.md) for what these
constraints deliberately exclude.
