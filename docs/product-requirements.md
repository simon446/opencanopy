# OpenCanopy V1 — Product Requirements (Locked)

**Status:** Locked (M0-04, M0-05) · **Owner:** Project & Repo track · **Spec refs:** §3.3, §4.1, §4.2, §20

This document freezes the physical envelope and the core hardware decisions for OpenCanopy V1. It is
the **design contract** every other track designs against. The values below are decisions, not
ranges-for-discussion.

> **Change control:** Any change to a locked value after this point requires a corresponding entry in
> [`risk-register.md`](risk-register.md) and sign-off from the Project & Repo track. Downstream tracks
> may treat these numbers as fixed.
>
> **🔄 REDESIGN — [ECO-003](ECO-003-v1-redesign.md) (2026-06-14, maintainer-approved).** V1 moved to a
> **two-pillar Scandinavian form** with **electronics in the top LED block** and **passive
> self-watering (no pump, no fan)**. Locked-value changes ratified here: wet/dry separation is now
> **top (electronics) vs bottom (water)**; **reservoir 4 L → 6 L**; the separate **10 L pot → an
> integrated removable grow insert**; **status LEDs 5 → 4**; power input **24 VDC barrel → USB-C**
> (pending electrical validation). Change-control trail: see [`risk-register.md`](risk-register.md).

---

## 1. Physical envelope (locked)

Two build variants are defined. **Compact V1 is the default build**; the full-yield variant is an
optional larger configuration. Every dimension below is a hard limit, not a target band.

| Parameter | Compact V1 (default) — committed nominal | Compact V1 — allowed band | Full-yield variant — hard max |
|---|---:|---:|---:|
| Width (W) | **480 mm** | 450–500 mm | **550 mm** |
| Depth (D) | **320 mm** | 300–350 mm | **400 mm** |
| Height (H) | **680 mm** | 650–750 mm | **850 mm** |
| Grow media (integrated insert) | **≈5 L effective** | 4–8 L | TBD (full-yield) |
| Reservoir volume | **6 L** (6.6 L gross modelled) | 5–7 L | **7 L** |
| Useful canopy footprint | 240–300 mm W × 170–210 mm D | — | 300–400 mm W × 220–300 mm D |
| Plant count | **1** | — | **1** |
| Wet electronics exposure | **None** (electronics are in the top block) | — | **None** |

**Hard maximums (no build may exceed):** W ≤ 550 mm, D ≤ 400 mm, H ≤ 850 mm, reservoir ≤ 7 L.

**Rationale for the committed compact nominals (ECO-003):**

- **Passive self-watering** replaces the pumped pot: a **6 L base reservoir** feeds a **removable grow
  insert** (slotted/perforated, semi-hydro, wick contact). Reservoir raised to 6 L to lengthen the
  unattended refill interval without a pump.
- The grow insert is a **raised planter** above a low (135 mm) base, so the base holds the reservoir at
  ≤ 130 mm visible height. Effective media (~5 L) is a documented trade-off vs the old 10 L pot,
  **gated on the grow trial** (risk-register R2/R9; ECO-003 Open Q3).
- H nominal 680 mm: two pillars place the LED with generous clearance over the raised insert.

## 2. Architecture & safety envelope (locked — ECO-003)

- **Top/bottom layout (current):** the **electronics live in the top LED block** (small 1.6 mm
  controller+driver PCB, encapsulated, USB-C input); the **base is a single wet zone** (6 L passive
  reservoir + removable grow insert). Water and electronics are separated by **geography (top vs
  bottom)**, not an in-base wall — no water path can reach the electronics (§17.1). Only sealed
  low-voltage sensor leads + status-LED light pipes touch the base, via a grommet at a pillar.
- **No AC mains inside the unit.** Power is **external low-voltage DC via USB-C** (pump-class loads
  removed; LED is the only significant load). USB-C-PD-vs-DC-barrel feasibility for 50–80 W is an
  open electrical-validation item (ECO-003 Open Q2); no mains inside, ever (§17.1).
- **Open-frame / non-enclosed default** (§3.6): not a sealed grow chamber. Room temperature is assumed
  **22–25 °C**. Firmware does **not** maintain an independent air-temperature setpoint. No humidifier,
  heater, refrigeration, or sealed chamber in V1.

## 3. Controller & user interface (locked)

| Decision | Locked value |
|---|---|
| MCU | **ESP32-S3** (dev module or custom PCB module) |
| Display | **None** — no LCD/e-ink in V1 |
| User controls | **Hidden service/reset control only** (factory reset / grow-cycle reset / test mode); no user-facing screen |
| Status interface | **4 front status LEDs** (Water, Moisture, Light, System); state by color **and** blink pattern + position (do not rely on color alone). *(Was 5 — the Climate LED is dropped: no fan / no active climate control; climate warnings fold into System. ECO-003.)* |
| Connectivity | **Optional, offline-first.** **USB-C** input doubles as the serial/dev interface; web/MQTT/Home Assistant telemetry allowed but **must not be required** |

## 4. Actuators (locked — ECO-003)

The **only actuator in V1 is the grow light.** Watering is passive; there is no fan.

| Subsystem | Locked decision |
|---|---|
| Pump | **REMOVED (ECO-003).** V1 is **passive self-watering** — a base reservoir feeds a removable slotted grow insert by capillary wicking. No pump, no pump driver/connector, no watering actuation; the firmware **monitors and warns** (low water / abnormal moisture) only. (A pump/aeration retrofit may be provisioned but is not populated.) |
| Fan | **REMOVED ([ECO-001](../electronics/analysis/ECO-001-fan-removal.md)).** No fan in V1; the LED runs on a passive heatsink under natural convection. |
| Light | **50–80 W dimmable full-spectrum white** horticultural LED for compact V1; **100 W only** for the full-yield variant. Dimmable via PWM/0–10 V matching the selected driver. The only significant electrical load (drives the USB-C-vs-barrel question, Open Q2). |

**Grow-light data requirement (§16.3):** a candidate light may be specified in the BOM **only** if it
provides actual power draw, a PPF/PPFD map, dimming method, full-spectrum/horticultural spectrum data,
thermal mounting information, and electrical certification/safety data where available. Lights
advertising only lumens, "equivalent watts," or vague "red/blue plant lamp" claims are rejected.
This is enforced in CI by `scripts/bom_check.py`.

## 5. Sensor set (locked)

Required V1 sensors (§7.5, §20): **temp/RH** (SHT31/SHT4x-class, I²C), **soil/media moisture**
(capacitive, corrosion-resistant, replaceable), **reservoir low-level**, and **leak/flood**
(overflow). VPD is **computed** from temp/RH, not separately sensed. *(Fan tach removed — no fan,
ECO-001. No pump current-sense — no pump, ECO-003.)*

## 6. Expansion headers (provisioned, not populated)

V1 PCB/mechanical may include connectors/pads for future modules but must **not** be blocked on them
(§4.3): camera, PAR/light sensor, load cell under pot, EC sensor, pH sensor, CO₂ sensor, e-ink status,
external telemetry. None of these are functional in V1 — see [`scope.md`](scope.md).

## 7. Traceability

| Locked item | Spec source |
|---|---|
| Footprint, grow insert, reservoir | §3.3, §20; ECO-003 |
| MCU / display / controls | §3.5, §7.1, §20 |
| Light (pump + fan removed) | §7.2–§7.4, §16, §20; ECO-003, ECO-001 |
| Sensor set | §7.5–§7.6, §20 |
| Open-frame / power / top-bottom zones | §3.6, §7.8, §17; ECO-003 |
| Expansion headers | §4.3 |

## 8. Rationale & research basis

These decisions are grounded in extension/research guidance (full list in
[`references.md`](references.md); design detail in spec §2 and §5). The key "whys":

- **Open-frame, room 22–25 °C, no enclosure.** A 22–25 °C indoor room is adequate for hot peppers, so
  the cost/complexity of a sealed, actively climate-controlled chamber isn't justified for V1; the
  device manages only LED heat, airflow, watering, and warnings (§2.3, §3.6). [R4, R5]
- **Integrated grow insert + passive semi-hydro (ECO-003).** Root volume still drives plant health, so
  the insert is sized as large as the low base allows (~5 L effective, raised) feeding from a 6 L
  reservoir; adequacy for a mature plant is grow-trial-gated (risk-register R2/R9). [R7, R8, R9]
- **Light specified in DLI/PPFD, broad-spectrum white, dimmable.** Fruiting peppers are high-light:
  the target is DLI ≈ 20–25 mol·m⁻²·day⁻¹ and ≥ ~400 µmol·m⁻²·s⁻¹ PPFD across the canopy, achievable
  in a tabletop footprint only with a constrained canopy, a real PPFD map, and pruning. Lumens / "plant
  lamp" claims are rejected (§2.1, §7.2, §16.3). [R1, R2, R3, R14]
- **Passive self-watering, no pump (ECO-003).** Steady moisture supports fruit retention and reduces
  blossom-end rot, but waterlogging harms roots — a **wicking semi-hydro** insert with an air gap aims
  for steady moisture **without a pump** (so there is no flood/overwater failure mode); the system
  monitors and warns instead of actuating. Root-zone moisture adequacy is the key grow-trial question
  (risk-register R3/R10). [R5, R8, R15, R16]
- **VPD computed from temp/RH, not RH alone.** VPD is the physiologically meaningful driver of
  transpiration and disease pressure (§5.4). [R13]

See [`references.md`](references.md) for the cited sources and [`scope.md`](scope.md) for what these
constraints deliberately exclude.
