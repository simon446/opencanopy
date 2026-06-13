# Tabletop Automated Pepper Grower — V1 Engineering Specification

**Document status:** Draft v1.1 — revised for compact open-frame tabletop target  
**Target crop:** Carolina Reaper / superhot pepper, treated as a hot-pepper profile for *Capsicum chinense* with environmental targets informed by broader pepper (*Capsicum* spp.) and controlled-environment agriculture research.  
**Target product:** Open-source, compact, non-enclosed tabletop indoor grow unit for one productive pepper plant.  
**Primary interface:** Zero-config appliance with LED status indicators.  
**Revision summary:** Default footprint reduced; default architecture changed to open-frame/non-enclosed; 22–25°C room operation accepted.  
**Non-goals for V1:** Camera AI, cloud plant health, pH/EC auto-dosing, active room-temperature control, mobile app dependency, multiple species profiles, high-yield commercial production.

---

## 1. Executive Summary

V1 is a compact, visually pleasant, open-source tabletop grow system for a single hot pepper plant, optimized for Carolina Reaper-class plants. It should be engineerable as a public repository containing:

- Mechanical CAD, STL/STEP files, print settings, and assembly drawings.
- Electronics schematics, PCB files, wiring diagrams, and hardware verification procedures.
- Firmware with deterministic control rules, simulation tests, hardware abstraction tests, and hardware-in-loop verification.
- Plant-growing rules embedded as a fixed pepper profile so the user does not need to configure species or growth recipes.
- Validation plans covering software, electronics, pump/irrigation behavior, thermal behavior, noise, mechanical fit, water safety, and real grow trials.

Revision v1.1 incorporates two product decisions:

- The default unit is **open-frame / non-enclosed**, assuming normal indoor room temperature around 22–25°C.
- The default footprint is reduced; a larger “full-yield” variant remains documented for builders who prefer maximum plant size over table fit.

The system should prioritize:

1. **Reliable plant growth over gadget features.**
2. **Safety around water and electricity.**
3. **Simple visible status.**
4. **Open, testable, reprogrammable architecture.**
5. **A living-room/table-friendly aesthetic.**

V1 should not ship with camera-based plant diagnosis. That can become a V2 expansion module after the basic light, water, airflow, sensor, and safety loops are proven. The most useful intelligence for V1 is not “AI,” but well-tested feedback loops around light schedule, substrate moisture, reservoir state, temperature, humidity/VPD, pump safety, and fault detection.

---

## 2. Research Basis and Key Design Decisions

### 2.1 Core plant facts used by this spec

The system targets a warm-season pepper. Relevant research and extension guidance support these facts:

- Peppers are warm-season crops. They grow poorly in cool conditions and fruit set is reduced at temperature extremes.
- Multiple extension sources report best pepper production around warm day conditions and mild nights, with fruit-set problems when days exceed roughly 90°F / 32°C or nights are too cold or too warm.
- Consistent moisture is important for flower/fruit retention and blossom-end-rot reduction, but waterlogging harms roots.
- Container-grown plants dry quickly and need frequent but not excessive watering.
- Peppers perform best around soil/potting media pH 6.0–6.8 in soil/container media.
- For controlled-environment grow lighting, DLI and PPFD are the primary engineering quantities, not lumens.
- Fruiting peppers should be treated as high-light plants. V1 should target a DLI around 20–25 mol·m⁻²·day⁻¹ at fruiting if heat and fixture capability allow, and it should be able to deliver at least ~400 µmol·m⁻²·s⁻¹ PPFD across the useful canopy area.
- A broad-spectrum white horticultural LED with sufficient blue and red content is preferred over a decorative or “blurple-only” LED for a tabletop living-space product.

### 2.2 Main sources used

The design values in this document are based on the following sources and their cited extension/research guidance:

| ID | Source | Used for |
|---|---|---|
| R1 | Virginia Tech Extension, “Calculating and Using Daily Light Integral (DLI)” — https://pubs.ext.vt.edu/SPES/spes-720/spes-720.html | DLI concept, PPFD/DLI relationship, 400–700 nm PAR framing |
| R2 | University of Missouri Extension, “Controlled Environment Agriculture: Understanding Grow Lights” — https://extension.missouri.edu/publications/g6987 | CEA grow-light selection, LED considerations, system-level light planning |
| R3 | Oklahoma State University Extension, “LED Grow Lights for Plant Production” — https://extension.okstate.edu/fact-sheets/led-grow-lights-for-plant-production | PPFD and DLI ranges; high-light crop framing |
| R4 | Oklahoma State University Extension, “Pepper Production” — https://extension.okstate.edu/fact-sheets/pepper-production | Pepper temperature, fruit-set constraints, soil pH |
| R5 | Michigan State University Extension, “How to Grow Peppers - Part 2” — https://www.canr.msu.edu/resources/how_to_grow_peppers_part_2 | Pepper day/night temperatures, moisture, blossom drop |
| R6 | University of Minnesota Extension, “Growing peppers” — https://extension.umn.edu/vegetables/growing-peppers | Seed starting, germination warmth, moisture, watering practice |
| R7 | University of Minnesota Extension, “Fertilizing and watering container plants” — https://extension.umn.edu/managing-soil-and-nutrients/fertilizing-and-watering-container-plants | Container moisture behavior and fertilizer caution |
| R8 | Illinois Extension, “Watering | Container Gardens” — https://extension.illinois.edu/container-gardens/watering | Container watering, avoiding waterlogged roots, avoiding full dry-out |
| R9 | Oregon State University Extension, “Grow your own peppers” — https://extension.oregonstate.edu/catalog/ec-1227-grow-your-own-peppers | Container size, drainage, greenhouse/container pepper guidance |
| R10 | Ohio State University Ohioline, “Growing Peppers in the Home Garden” — https://ohioline.osu.edu/factsheet/hyg-1618 | pH, drip/soaker watering, wet-leaf disease avoidance |
| R11 | Oklahoma State University Extension, “Electrical Conductivity and pH Guide for Hydroponics” — https://extension.okstate.edu/fact-sheets/electrical-conductivity-and-ph-guide-for-hydroponics | Hydroponic pH/EC reference values for peppers |
| R12 | University of Missouri Extension, “Hydroponic Nutrient Solutions” — https://extension.missouri.edu/publications/g6984 | Nutrient solution management, pH/EC monitoring limitations and calibration |
| R13 | Michigan State University Extension, “Why should greenhouse growers pay attention to vapor-pressure deficit and not relative humidity?” — https://www.canr.msu.edu/news/why_should_greenhouse_growers_pay_attention_to_vapor_pressure_deficit_and_n | VPD logic and humidity disease/transpiration framing |
| R14 | Liu et al., “Effects of LED Light Quality on the Growth of Pepper Seedlings and Development after Transplanting,” Agronomy, 2022 — https://www.mdpi.com/2073-4395/12/10/2269 | Pepper seedling response to blue/UV-A/far-red light quality |
| R15 | Michigan State University Extension, “Blossom end rot: Understanding a perennial problem” — https://www.canr.msu.edu/news/blossom_end_rot_understanding_a_perennial_problem | Blossom-end-rot relationship to calcium transport and inconsistent water |
| R16 | Ohio State University Small Farm Team, “Blossom End Rot of Tomatoes and Peppers” — https://u.osu.edu/gofarmohio/2022/07/15/blossom-end-rot-of-tomatoes-and-peppers/ | Blossom-end-rot stress and dry/wet soil swings |
| R17 | PuckerButt Pepper Company, Carolina Reaper growing guide — https://puckerbuttpeppercompany.com/blogs/insights/the-complete-guide-to-growing-your-very-first-carolina-reaper-pepper-plant | Carolina Reaper-specific seed-starting and temperature context |

### 2.3 Important limitation

This is not a commercial greenhouse crop plan. It is a small indoor device plan. Therefore:

- The DLI target is ambitious but can be approached in a compact tabletop footprint only if the canopy area is constrained, the LED has a real PPFD map, and the plant is pruned/trained.
- The default compact geometry trades some maximum yield for table fit and visual acceptability. This is acceptable for V1 because the target is credible indoor fruiting, not commercial yield.
- The default design is not enclosed. It does not actively heat or cool the room. It only manages LED heat contribution, airflow, watering, and warning states.
- A 22–25°C indoor room is acceptable for hot pepper growth. It is not severe enough to justify an enclosure in V1. Germination may still benefit from a removable heat mat or external warm-start method.
- The system must support dimming and heat management rather than running a high-power LED blindly.

---

## 3. Product Definition

### 3.1 Product goal

Build an open-source tabletop grow system that can maintain a single hot pepper plant indoors with minimal user involvement.

The user should only need to:

1. Fill the reservoir.
2. Add nutrients/fertilizer manually according to a simple written schedule.
3. Prune/support the plant when necessary.
4. Harvest fruit.
5. Clean the reservoir and pump/filter periodically.
6. Reset the grow cycle when starting a new plant.

### 3.2 Intended user

- Technical hobbyist, open-source hardware builder, engineer, maker, or advanced indoor gardener.
- Comfortable assembling electronics and 3D-printed/mechanical parts.
- Does not want to tune a dashboard daily.
- Wants a living-room/table-friendly object rather than a black grow tent.

### 3.3 Physical footprint target

| Dimension | Compact V1 target | Hard max / full-yield variant |
|---|---:|---:|
| Width | 450–500 mm | 550 mm |
| Depth | 300–350 mm | 400 mm |
| Height | 650–750 mm | 850 mm |
| Pot volume | 8–10 L | 12–19 L if yield is prioritized |
| Reservoir | 2.5–4 L | 4–6 L |
| Useful canopy footprint | 300–400 mm W × 220–300 mm D | 400–450 mm W × 300–350 mm D |
| Wet electronics exposure | None | None |
| Plant count | 1 | 1 |

### 3.4 Visual design target

V1 should look like a small furniture object or tabletop appliance, not like lab equipment.

Recommended visual language:

- White or matte neutral frame.
- Light wood/bamboo/oak-colored front trims.
- Hidden wiring.
- Translucent LED-status diffuser strip.
- Open front for plant visibility.
- Rear/top service access.
- Bottom reservoir not visibly industrial.
- No LCD/e-ink display in the default v1 design.

### 3.5 Primary UX

The system should be usable with no app and no normal controls.

Visible user interface:

| Indicator | Position | Normal | Warning | Fault |
|---|---|---|---|---|
| Water | Front LED 1 | Green steady | Amber slow pulse | Red blink |
| Moisture | Front LED 2 | Green steady | Amber pulse | Red blink |
| Light | Front LED 3 | Green when active / dim when sleeping | Amber if dimmed due to heat | Red if LED fault |
| Climate | Front LED 4 | Green steady | Amber if hot/dry/humid | Red if critical |
| System | Front LED 5 | Green heartbeat | Amber maintenance | Red fault |

Design requirements:

- Do not rely on color alone. Blink pattern and LED position must convey state.
- Optional hidden service button is allowed for factory reset / grow-cycle reset / test mode.
- No user-facing screen in V1.
- Optional serial/USB development interface is allowed.
- Optional web/MQTT/Home Assistant telemetry is allowed but must not be required.

### 3.6 Open-frame / non-enclosed default

V1 should be an **open-frame appliance**, not a sealed grow chamber.

Design consequences:

- Room temperature is assumed to be 22–25°C.
- The firmware does not attempt to maintain an independent air-temperature setpoint.
- The fan is for gentle canopy airflow, heat dispersion, and stale-air prevention, not active cooling below ambient.
- LED intensity is the only meaningful heat source the device can control.
- The system may derate the LED if canopy temperature rises, but it should not overcomplicate temperature control.
- No humidifier, heater, refrigeration module, or sealed chamber is included in V1.
- A removable seed-starting dome or separate heat mat may be documented for germination, but it is not part of the core appliance.

Engineering interpretation: 22–25°C room conditions are close enough to published pepper production ranges to proceed without an enclosure. The likely penalties are slower germination without bottom heat and possibly less optimal fruit set if nights remain warm, but these are not severe enough to justify enclosure complexity in V1.

---

## 4. V1 Scope and Non-Scope

### 4.1 V1 includes

- One plant.
- Fixed hot-pepper profile.
- Broad-spectrum horticultural LED with dimming.
- Automated light schedule.
- Soil/substrate moisture sensing.
- Automated pump watering.
- Reservoir-level sensing.
- Temperature/humidity sensing.
- VPD calculation.
- Circulation fan control.
- Leak/flood safety sensor.
- LED status interface.
- Local deterministic firmware control.
- Open-source firmware.
- Open-source PCB and wiring.
- Open-source mechanical design.
- Simulation tests.
- Hardware verification.
- Grow validation protocol.

### 4.2 V1 excludes

- Camera plant health analysis.
- Cloud AI plant diagnosis.
- Plant species auto-ID.
- Automatic pH dosing.
- Automatic EC/nutrient dosing.
- Multi-plant support.
- Commercial yield optimization.
- Voice assistant dependency.
- Required mobile app.
- AC mains inside the grow unit.

### 4.3 Optional V1 expansion headers

Include connectors/pads for future modules, but do not block V1 on them:

- Camera module.
- PAR/light sensor.
- Load cell under pot.
- EC sensor.
- pH sensor.
- CO₂ sensor.
- E-ink status module.
- External telemetry module.

---

## 5. Plant-Growing Specification

### 5.1 Growth stages

The firmware should implement a built-in hot-pepper lifecycle profile. No user species selection is required.

| Stage | Approx. duration | Trigger | Main objective |
|---|---:|---|---|
| S0 Germination | 0–21 days | Grow-cycle reset | Warm, moist media until emergence |
| S1 Seedling | 21–56 days | Time-based | Compact growth, avoid stretch |
| S2 Vegetative | 56–100 days | Time-based / plant size | Strong canopy and root system |
| S3 Flowering | 100–140 days | Time-based / first flower observed manually optional | Stable moisture, pollination support |
| S4 Fruiting/ripening | 140+ days | Time-based | Stable water, high light, avoid heat stress |
| S5 Maintenance/overwinter | optional | Manual dev setting only | Lower light/water, survival mode |

Because V1 has no camera, stage detection is time-based. The hidden service button can reset the grow cycle. This is not considered “configuration”; it is equivalent to starting a timer when a new plant is inserted.

If the build is intended only for purchased transplants, the firmware can skip S0/S1 by flashing a `TRANSPLANT_PROFILE=true` build flag. The public default should include all stages.

### 5.2 Environmental targets

The table below gives biological targets. Because V1 is **non-enclosed**, firmware should treat temperature and RH as monitored conditions, not as actively controlled setpoints. Light and watering remain actively controlled.

| Stage | Preferred air temp | Acceptable open-room assumption | RH target | VPD target | DLI target | Photoperiod | PPFD target at canopy |
|---|---:|---:|---:|---:|---:|---:|---:|
| Germination | 26–30°C media | Room 22–25°C acceptable but slower; optional external heat mat recommended | 65–80% | n/a | 0–6 after emergence | 0–16 h | 0–100 µmol/m²/s |
| Seedling | 23–27°C day, 18–22°C night | 22–25°C acceptable | 60–75% | 0.5–0.9 kPa | 8–12 | 16 h | 140–210 µmol/m²/s |
| Vegetative | 22–28°C | 22–25°C ideal/acceptable | 55–75% | 0.7–1.2 kPa | 14–20 | 16 h | 245–350 µmol/m²/s |
| Flowering | 22–27°C day, 18–22°C night preferred | 22–25°C acceptable; avoid LED raising canopy above 28–30°C | 55–70% | 0.8–1.2 kPa | 18–24 | 16 h | 315–420 µmol/m²/s |
| Fruiting | 22–27°C day, 18–22°C night preferred | 22–25°C acceptable; prioritize stable moisture and high light | 55–70% | 0.8–1.3 kPa | 20–25 | 16 h | 350–435 µmol/m²/s |

Formula:

```text
DLI = PPFD × photoperiod_hours × 0.0036
PPFD = DLI / (photoperiod_hours × 0.0036)
```

Example for fruiting:

```text
Target DLI: 23 mol·m⁻²·day⁻¹
Photoperiod: 16 h
Required average PPFD: 23 / (16 × 0.0036) ≈ 399 µmol·m⁻²·s⁻¹
```

### 5.3 Temperature behavior in open-frame mode

Firmware should monitor temperature and use fan/LED derating to avoid adding avoidable heat stress. It should not pretend it can cool the plant below room temperature.

| Condition | Action |
|---|---|
| <16°C air | Climate LED amber; fan minimum only; do not increase watering due to slow uptake |
| 16–20°C | Accept; possible slower growth; no active heating in V1 |
| 20–25°C | Normal open-room operating band |
| 25–28°C | Normal if transient; increase fan slightly during lights-on |
| 28–30°C | Climate LED amber if sustained; fan high; prevent additional LED heat if rising |
| 30–32°C | Fan high; reduce LED 20–40% if LED heat is contributing |
| >32°C | Climate LED red; reduce LED 40–70%; pump only if substrate is dry; log heat fault |
| >35°C | Critical over-temp; LED off or minimum; fan high; system fault if sustained |

Fruit-set protection:

- Avoid sustained canopy temperatures above 32°C. Published pepper guidance repeatedly flags poor fruit set/blossom drop around this range and above.
- Constant 22–25°C indoor room temperature is acceptable for V1 and does not justify an enclosure.
- Flowering may improve if nights naturally fall closer to 18–22°C, but this is an optimization, not a V1 requirement.
- Avoid watering decisions based only on high temperature; always verify substrate moisture.

### 5.4 Humidity and VPD behavior

Use VPD as the internal decision variable, but expose only climate status to the user.

Basic VPD logic:

| VPD | Interpretation | Action |
|---:|---|---|
| <0.4 kPa | Air too humid / low transpiration | Fan increase, avoid watering unless dry |
| 0.5–1.2 kPa | Normal productive range | Normal control |
| 1.2–1.6 kPa | Dry air / high transpiration | Watch moisture, shorter dryback allowed |
| >1.6 kPa | Stress risk | Fan may not help; alert if persistent; avoid LED heat increase |

RH guardrails:

| RH | Action |
|---:|---|
| >85% sustained | Climate amber/red; fan high; disease-risk warning |
| 70–85% | Accept if short-term, increase airflow |
| 55–70% | Preferred flowering/fruiting range |
| <40% | Dry-air warning if VPD high |

V1 does not include humidification. It should only warn and adjust fan/light/watering conservatively.

### 5.5 Substrate and potting media

Recommended V1 medium:

- High-quality peat/coco-based potting mix.
- Perlite or similar aeration amendment.
- Drainage-capable pot.
- Optional internal wicking mat/reservoir, but not a fully sealed no-drainage pot.

Hard constraints:

- Pot volume: 8–10 L compact V1 baseline. Use 12–19 L for the optional full-yield variant. Do not go below 8 L for a Reaper-class plant unless the project explicitly accepts ornamental/low-yield behavior.
- Pot must drain into a controlled tray or internal basin.
- Pot must be removable without disconnecting electronics.
- Pot must not sit in stagnant water unless intentionally designed as a self-watering planter with an air gap.
- Avoid native garden soil indoors due to drainage, compaction, and pest issues.

### 5.6 Watering target

V1 should not try to keep the substrate constantly wet. It should maintain moist-but-aerated conditions with controlled dryback.

Control target:

| State | Meaning | Action |
|---|---|---|
| Too wet | Sensor above wet threshold for long period | Block watering; amber moisture LED; increase fan if humid |
| Target | Within range | No action |
| Dry | Below dry threshold | Dose water in pulses |
| Critically dry | Well below dry threshold | Dose water with stricter observation; red/amber LED if not recovering |

Watering windows:

- Prefer watering during the light period.
- Prefer first 60–70% of the light period.
- Avoid routine watering in the last 2 hours before lights-off.
- Emergency watering is allowed at any time if critically dry.
- Do not water if reservoir is low.
- Do not water if leak sensor is active.
- Do not water if pump-fault state is active.
- Do not water continuously; use pulses and remeasure.

Initial defaults:

| Stage | Dry threshold | Wet threshold | Pulse size target | Recheck delay |
|---|---:|---:|---:|---:|
| Seedling | calibrated 35% VWC-equivalent | calibrated 55% | 20–50 mL | 15–20 min |
| Vegetative | calibrated 30% | calibrated 55% | 50–150 mL | 20–30 min |
| Flowering | calibrated 35% | calibrated 60% | 75–200 mL | 20–30 min |
| Fruiting | calibrated 35% | calibrated 60% | 100–250 mL | 20–30 min |

These percentages are not raw sensor percentages. They are normalized wet/dry calibration units. The firmware must never assume raw capacitive ADC readings correspond directly to volumetric water content without calibration.

### 5.7 Nutrients

V1 should not include automatic nutrient dosing. It should document a manual feeding schedule.

Recommended default documentation:

- Use a complete pepper/tomato fertilizer suitable for container vegetables.
- Avoid over-fertilization.
- Start with low-strength feeding after seedlings are established.
- Increase potassium-supported fruiting fertilizer only after flowering/fruit set.
- Include a monthly flush/refresh recommendation if using soluble fertilizer and drainage exists.
- Optional: use slow-release container fertilizer plus plain water in reservoir for simplest operation.

Hydroponic/DWC is not the recommended V1 baseline because pH/EC management adds significant calibration and maintenance burden. If a variant is hydroponic, require pH/EC manual testing and separate documentation.

### 5.8 pH and EC

For the default potting-mix/drip V1:

- No continuous pH/EC sensors.
- Include optional manual test guidance.
- Target potting media pH: 6.0–6.8.
- Avoid repeated heavy fertilization without drainage/flush.

For optional hydroponic variant:

- Nutrient solution pH: 5.5–6.0.
- Root-zone pH: approximately 6.0–6.5.
- Pepper EC reference: 0.8–1.8 mS/cm from OSU hydroponic table, with caution that commercial greenhouse pepper guidance can run higher depending on stage/system.
- Require weekly pH/EC meter calibration if pH/EC automation is ever added.

### 5.9 Pollination and pruning

V1 should not mechanically pollinate. It should support airflow and include grow-guide instructions:

- Gently shake plant or tap flowers several times per week during flowering.
- Maintain airflow, but not strong wind.
- Support branches as fruit load increases.
- Prune only enough to maintain clearance and airflow.
- Keep fruit from touching LED or fan.
- Leave enough leaf canopy to protect fruit from intense light and heat.

Optional V2 feature:

- Tiny vibration motor mounted to frame or trellis, run briefly at midday during flowering. Not required in V1.

---

## 6. System Architecture

### 6.1 High-level architecture

```text
External certified AC/DC power brick
        |
        v
24 VDC input into dry upper electronics bay
        |
        +--> fuse / protection / power switch
        |
        +--> LED driver / dimmer --> grow light
        |
        +--> DC/DC 12 V rail --> fan / pump if 12 V
        |
        +--> DC/DC 5 V rail --> sensors / logic
        |
        +--> 3.3 V rail --> MCU
        |
        v
MCU control board
        |
        +--> temp/humidity sensor
        +--> capacitive moisture sensor
        +--> reservoir level sensor
        +--> leak sensor
        +--> fan PWM/tach
        +--> pump MOSFET/driver
        +--> LED dimming output
        +--> front status LEDs
        +--> optional UART/USB/debug
```

### 6.2 Physical zone model

```text
TOP / UPPER DRY ZONE
- Controller PCB
- LED driver
- Power distribution
- Status LED board
- Optional comms/camera expansion
- Service access

MIDDLE GROW ZONE
- Plant canopy
- Pot
- LED fixture above
- Fan at rear/side
- Temp/humidity sensor shielded from direct LED heat
- Moisture probe routed into pot

BOTTOM WET ZONE
- Removable water reservoir
- Pump
- Intake filter
- Water-level sensor
- Leak tray/sensor
- Drainage/overflow path
```

Design rule:

> Water may fail downward. Electronics must live upward and sideways from the wet path.

### 6.3 Control philosophy

V1 control is deterministic:

```text
time-of-day + plant age + sensor inputs + safety states
→ light/fan/pump outputs
→ LED user feedback
```

No cloud decision-making is required. Cloud AI is explicitly deferred.

---

## 7. Hardware Specification

### 7.1 Controller selection

Recommended V1 MCU:

| Option | Recommendation | Reason |
|---|---|---|
| ESP32-S3 | Preferred | Good firmware ecosystem (C/C++ **and** Rust via `esp-hal`), Wi-Fi/BLE, enough RAM/IO, future camera compatibility |
| ESP32-C6 | Acceptable | Wi-Fi 6 + BLE + Thread/Zigbee/Matter path |
| ESP32-C3 | Acceptable low-cost | Wi-Fi/BLE, fewer IO |
| Raspberry Pi Pico W | Acceptable | Simple control, Wi-Fi, but less integrated CEA ecosystem |
| ESP32-H2 only | Not preferred | No Wi-Fi; better as Zigbee/Thread node |

V1 should standardize on **ESP32-S3** unless Matter/Thread is a hard requirement. This leaves camera expansion possible without adding a second compute board later.

### 7.2 Light requirements

This is the most important hardware component.

#### Required light type

Use a **full-spectrum white horticultural LED bar or panel** with enough blue and red content. Avoid decorative “plant lamps” with no PPFD map. Avoid blurple-only fixtures for a living-room tabletop design unless a white-light cover mode is added.

Minimum fixture requirements:

| Parameter | Minimum compact V1 | Preferred compact V1 / full-yield headroom |
|---|---:|---:|
| Electrical power | 40–50 W dimmable | 50–80 W dimmable; 100 W only for larger variant |
| PPF | 100 µmol/s | 140–220 µmol/s |
| PPE | 2.2 µmol/J | ≥2.5 µmol/J |
| Dimming | PWM or 0–10 V | 0–10 V isolated or logic PWM |
| Spectrum | Broad white horticultural | White + 660 nm deep red; ≥10% blue content |
| PPFD at canopy | 350–450 µmol/m²/s average | 400–550 µmol/m²/s average if heat allows |
| Uniformity | ≥0.6 min/avg | ≥0.7 min/avg |
| Driver | Remote or thermally isolated | Remote driver in upper dry bay |
| Thermal protection | Required | NTC or driver thermal foldback |
| Ingress protection | Splash-resistant preferred | IP54+ or conformal protected board |
| Color rendering | Not critical for plants | Pleasant enough for living area |

#### Canopy area assumption

V1 useful canopy target:

```text
Compact canopy width: 300–400 mm
Compact canopy depth: 220–300 mm
Compact canopy area: ~0.07–0.12 m²
Full-yield variant canopy area: ~0.12–0.16 m²
```

If targeting 450 µmol/m²/s across 0.10 m²:

```text
Required delivered PPF ≈ 450 × 0.10 = 45 µmol/s
```

Because optics, height, side losses, canopy shape, and non-uniformity reduce delivered photons, fixture PPF should be much higher than the delivered target. A 140–220 µmol/s fixture is a practical compact range for dimmed operation with headroom. A larger 220–250 µmol/s fixture belongs in the optional larger/full-yield variant.

#### Light mounting

- LED must be mounted above canopy.
- Height should be adjustable if possible.
- Minimum clearance from mature canopy: 150 mm unless PPFD/thermal measurements prove safe.
- Thermal path must not dump heat into electronics.
- Driver heat should be isolated from plant canopy and plastic parts.
- Use mechanical secondary retention so the light cannot fall into the plant/wet area.

### 7.3 Pump requirements

Recommended V1 pump type:

**Brushless DC submersible centrifugal pump**, 12 V or 24 V.

Why this over peristaltic for V1:

| Pump type | Pros | Cons | V1 decision |
|---|---|---|---|
| Brushless submersible centrifugal | Quiet, cheap, low maintenance, continuous-rated, long life | Must stay submerged, less precise dosing, can clog | **Preferred V1** |
| Peristaltic | Precise, self-priming, no wetted pump internals, dry-run tolerant | Noisier, tubing wear, lower flow, more expensive | Optional precision variant |
| Diaphragm | Self-priming, decent pressure | Pulsing noise, vibration | Not preferred |
| Solenoid valve + gravity | Quiet, simple if elevated tank | Requires elevated reservoir, leak risk | Not preferred |

Required pump constraints:

| Parameter | Target |
|---|---:|
| Voltage | 12 V or 24 V DC |
| Flow rating | 80–240 L/h nominal; do not oversize for a small tabletop pot |
| Head height | ≥0.8 m, preferred ≥1.0 m |
| Noise | ≤30 dBA at 1 m after mounting, target ≤25 dBA |
| Power | <5 W preferred, <10 W max |
| Intake | Removable filter/screen |
| Mount | Rubber isolated or suspended in reservoir |
| Dry-run protection | Firmware timeout + reservoir sensor |
| Dose repeatability | ±25% acceptable after calibration |
| Tubing | 4/6 mm or 6/8 mm silicone/PVC |
| Outlet | Drip ring or single emitter at substrate surface |
| Backflow/siphon | Anti-siphon hole/check valve/air gap |

Pump dosing should be calibrated in firmware as `ml_per_second`, measured during assembly.

### 7.4 Fan requirements

Recommended fan type:

**Quiet brushless DC PWM fan**, 80 mm or 92 mm, fluid-dynamic or magnetic bearing.

Required constraints:

| Parameter | Target |
|---|---:|
| Voltage | 12 V DC |
| Size | 80×80×25 mm or 92×92×25 mm |
| Control | PWM preferred |
| Tachometer | Preferred |
| Airflow | 5–20 CFM usable range |
| Noise | ≤20 dBA at normal speed, ≤30 dBA at max |
| Bearing | Fluid-dynamic / SSO / magnetic |
| Mounting | Rubber grommets |
| Guard | Required |
| Intake filter | Optional, removable |

Airflow goal:

- Gentle leaf movement.
- No strong drying stream directly at seedling.
- Avoid stagnant humid microclimate.
- Maintain electronics cooling if the same airflow path is used, but avoid pulling moist plant air through electronics.

### 7.5 Sensors

#### Required sensors

| Sensor | Recommended part class | Placement | Purpose |
|---|---|---|---|
| Air temp/humidity | SHT31/SHT35/SHT40/SHT41-class digital sensor | Mid-height, shaded from LED, near canopy but not in direct fan stream | Climate/VPD |
| Soil/substrate moisture | Capacitive probe, corrosion-resistant | Root zone, removable | Watering decision |
| Reservoir level | Float switch + optional analog/pressure/optical | Reservoir | Low-water safety |
| Leak sensor | Conductive trace or sensor strip | Bottom tray below reservoir/pump | Pump lockout |
| Fan tach | Fan output | Fan | Fault detection |
| Pump current optional | Current sense resistor/INA219-class | Pump rail | Pump fault/clog/dry run inference |

#### Optional sensors

| Sensor | V1 status | Reason |
|---|---|---|
| Light/PAR sensor | Optional dev tool | Useful for calibration, not required for every unit |
| Pot weight/load cell | Optional expansion | Strong water-use signal but mechanical complexity |
| pH sensor | Not V1 | Calibration/maintenance burden |
| EC sensor | Not V1 | Calibration/maintenance burden |
| Camera | Not V1 | Scope creep |
| CO₂ | Not V1 | Indoor tabletop product cannot control CO₂ meaningfully |

### 7.6 Soil moisture sensor details

Use a **capacitive** moisture sensor, not a resistive probe, to reduce corrosion.

Requirements:

- Must be conformal-coated except sensing area if needed.
- Must use a replaceable connector.
- Cable must be strain-relieved.
- Firmware must support calibration.
- Default thresholds must be validated against chosen potting mix.
- Sensor must fail safe: implausible reading disables auto-watering and shows fault.

Recommended calibration model:

```text
raw_air
raw_dry_media
raw_field_capacity_media
raw_saturated_media

normalized_moisture = map(raw, dry_media, saturated_media, 0..100)
```

Do not assume one sensor’s raw ADC values apply to all media and all probes.

### 7.7 Reservoir

Target:

| Parameter | Target |
|---|---:|
| Volume | 4–6 L |
| Removal | Tool-free |
| Fill method | Pull-out reservoir or front/top fill |
| Cleaning | Hand-cleanable opening |
| Material | Food-safe plastic preferred |
| Level sensing | Low-level required; full/percent optional |
| Spill handling | Overflow path to leak tray, not electronics |
| Pump access | Tool-free filter cleaning |

Reservoir should be below the pot and electronics. If a fill opening is near electronics, add physical barriers and guttering to route spills away.

### 7.8 Power architecture

Use external certified AC/DC power brick.

Recommended:

```text
AC mains
→ certified external 24 VDC power brick
→ grow unit low-voltage DC only
```

Power budget:

| Load | Typical | Peak |
|---|---:|---:|
| LED | 50–100 W | 100 W |
| Pump | 3–10 W | 15 W |
| Fan | 0.5–3 W | 5 W |
| MCU/sensors/status | <2 W | 5 W |
| Headroom | 20% | 25% |

Recommended PSU:

- 24 VDC, 120 W for 60–80 W LED builds.
- 24 VDC, 150 W for 100 W LED builds.
- Certified CE/UL-equivalent external brick.
- Barrel jack or locking DC connector rated for current.
- Input fuse inside unit.
- Reverse-polarity protection.
- TVS diode.
- Power switch optional.

Rails:

```text
24 V: LED driver, optional 24 V pump
12 V: fan, optional 12 V pump
5 V: sensors/peripherals
3.3 V: MCU
```

### 7.9 PCB requirements

| Area | Requirement |
|---|---|
| Layers | 2-layer acceptable; 4-layer preferred for cleaner power/ground |
| Connectors | Locking JST/Molex/Wago-style; no loose Dupont for production build |
| Test points | Every rail, I2C, UART, pump drive, fan PWM/tach, LED dimming, sensor inputs |
| Protection | Fuse, reverse polarity, TVS, flyback where needed, current limits |
| Wet isolation | PCB in dry bay only; conformal coat optional but not substitute for enclosure |
| Mounting | Standoffs, no board flex, serviceable |
| Silkscreen | Connector names, polarity, voltage, warning marks |
| Debug | USB-C or UART header |
| Boot/reset | Accessible during assembly, hidden in normal use |
| Expansion | Camera/light/pH/EC headers optional but unpopulated |

### 7.10 PCB trace/current constraints

Minimum engineering requirements:

- Use trace-width calculator in CI or documented DRC notes for pump/fan/LED current paths.
- Do not route high-current LED power through thin control traces.
- Keep LED current loop away from sensor ADC lines.
- Separate analog moisture sensor area from high-current switching.
- Star or partition grounds if analog readings are noisy.
- Use screw/lever terminals or locking connectors for >1 A loads.
- Thermal relief should not compromise current-carrying paths.
- Add copper pours for MOSFET heat dissipation.
- MOSFETs must be logic-level at 3.3 V gate drive or use driver.

### 7.11 Status LED board

Use a separate small front-panel PCB.

Requirements:

- 5 indicator positions.
- RGB or separate colored LEDs acceptable.
- Diffuser strip.
- Current-limited.
- PWM dimming for night mode.
- Patterns available:
  - steady
  - slow pulse
  - fast blink
  - double blink
  - off
- Colorblind support via position and pattern.

---

## 8. Mechanical Specification

### 8.1 Layout

Preferred V1 layout is an **open-frame vertical stack**:

```text
[Top/upper rear dry bay]
  PCB, LED driver, status wiring, power distribution

[Upper open grow area]
  LED fixture overhead, shielded from direct water path

[Middle open grow area]
  plant, support, fan, air sensor

[Lower grow area]
  compact 8–10 L removable pot and drip/watering outlet

[Bottom wet bay]
  removable 2.5–4 L reservoir, pump, filter, leak tray
```

Do not use a sealed box as the default. The frame should be visually open, with only enough structure to hold the light, fan, cable channel, reservoir, electronics bay, and optional trellis.

### 8.2 Mechanical modules

| Module | Files | Notes |
|---|---|---|
| Frame | STEP + printable brackets | Wood/aluminum/printed hybrid |
| Light mount | STEP + STL | Adjustable height preferred |
| Fan mount | STEP + STL | Rubber isolation, guard |
| Pot tray | STEP + STL | Captures overflow |
| Reservoir bay | STEP + STL | Tool-free removal |
| Electronics bay | STEP + STL | Dry, serviceable |
| Cable channel | STEP + STL | Separate water tube and wiring |
| Status LED diffuser | STEP + STL | Clean front UI |
| Sensor clips | STL | Replaceable |
| Trellis/support | STL/STEP | Optional support ring/stake mount |

### 8.3 Materials

Recommended:

| Part | Material |
|---|---|
| Wet bay/reservoir-adjacent printed parts | PETG, ASA, ABS |
| LED-adjacent parts | ASA, ABS, aluminum, or heat-proven PETG |
| Decorative trim | Wood, bamboo, veneer, printed wood-fill only if away from moisture/heat |
| Transparent/diffuser | Frosted acrylic, PETG, polycarbonate |
| Gaskets | Silicone |
| Fasteners | Stainless steel where near water |
| Tubing | Silicone or PVC |
| Pot | Food-safe plastic, ceramic, or coated planter insert |

Avoid:

- PLA near LED heat or persistent humidity.
- Exposed MDF in wet areas.
- Untreated wood in splash zones.
- Hidden screws under reservoir water path.

### 8.4 Serviceability

The user must be able to:

- Remove reservoir without moving the plant.
- Remove pump/filter for cleaning.
- Remove pot without cutting wires.
- Replace moisture probe.
- Replace fan.
- Replace LED fixture.
- Access electronics without opening wet bay.
- Inspect tubing.
- Clean leak tray.

### 8.5 Cable and tube routing

Rules:

- Wires and tubes must be separated where possible.
- Use drip loops before entering electronics bay.
- No cable enters electronics bay from directly below without a drip loop and grommet.
- Tubing should be visible or inspectable enough to find leaks.
- Add strain relief at all moving/removable modules.
- Use labels for pump, fan, LED, moisture, reservoir, leak.

### 8.6 Light/plant clearance

- LED height must allow 150–300 mm clearance above typical canopy.
- If fixed-height, design total grow zone height ≥600 mm.
- Plant support should keep branches away from fan blades and LED.
- Include pruning guidance because Carolina Reaper can outgrow the tabletop unit.

### 8.7 Acoustic design

Noise target:

| Mode | Target |
|---|---:|
| Normal day mode | ≤30 dBA at 1 m |
| Night mode | ≤25 dBA at 1 m |
| Pump active | ≤35 dBA at 1 m, short duration |
| Fault fan max | ≤40 dBA acceptable |

Mechanical noise mitigations:

- Rubber fan mounts.
- Rubber pump pad/suction cups.
- Avoid hard tubing vibrating against frame.
- Use PWM fan frequency outside audible whine range.
- Use soft-start pump if possible.
- Avoid resonant thin panels.

---

## 9. Firmware Specification

### 9.1 Firmware goals

- Fully local operation.
- Deterministic rules.
- No internet dependency.
- Safe defaults.
- Fault-first design.
- Testable control logic independent of hardware.
- Optional telemetry, not required.

### 9.2 Firmware language and architecture

**Language: Rust** (`no_std`), using the `esp-hal` bare-metal ecosystem for the ESP32-S3. Rust is
chosen for this safety- and fault-first controller (§9.1): ownership/borrow checking removes whole
classes of memory bugs, and the type system lets the state machine (§9.3) and fault priorities be
encoded as compiler-checked state types rather than runtime conventions. The core control logic lives
in a platform-agnostic `no_std` crate that compiles and unit-tests on the host with stable Rust, so no
hardware is needed to validate rules (§10.1).

Toolchain note: the ESP32-S3 is Xtensa, which is not on upstream stable Rust; install the Espressif
Rust channel with `espup` and pin it via `rust-toolchain.toml`. Flashing/monitoring uses `espflash`.
Host-only crates build with ordinary stable Rust. (Telemetry, §9.11, uses `esp-wifi` behind a Cargo
feature and is never required for control.)

Recommended structure (a Cargo workspace):

```text
firmware/
  Cargo.toml              # workspace manifest
  rust-toolchain.toml     # pins the esp Rust channel (espup)
  .cargo/config.toml      # default target xtensa-esp32s3-none-elf + espflash runner
  control/                # no_std, platform-agnostic control logic — host-testable
    src/
      lib.rs
      app_state.rs
      plant_profile.rs
      scheduler.rs
      irrigation_controller.rs
      light_controller.rs
      climate_controller.rs
      safety_controller.rs
      led_status.rs
      hal.rs              # sensor/actuator/clock traits (the hardware seam)
    tests/                # host unit + integration tests
  controller/             # no_std esp-hal binary for ESP32-S3 — binds traits to real peripherals
    src/
      main.rs
      sensors/
      actuators/
      drivers/
  sim/                    # host scenario runner driving the control crate (no hardware)
    src/
    scenarios/
    models/
  hil/
    fixtures/
  tools/
```

Dependency rule: `control/` depends only on its own `hal.rs` traits, never on `esp-hal`. `controller/`
and `sim/` provide concrete trait implementations (real peripherals vs. simulated models).

### 9.3 Firmware state machine

Required states:

```text
BOOT
SELF_TEST
NORMAL
WATERING
LOW_WATER
LEAK_DETECTED
SENSOR_FAULT
PUMP_FAULT
FAN_FAULT
LED_FAULT
OVER_TEMP
MAINTENANCE
SAFE_SHUTDOWN
```

State priority:

```text
LEAK_DETECTED
> OVER_TEMP critical
> PUMP_FAULT
> SENSOR_FAULT affecting watering
> LOW_WATER
> NORMAL/WATERING
```

### 9.4 Boot behavior

On boot:

1. Initialize hardware.
2. Run self-test.
3. Read persistent config/calibration.
4. Validate sensor ranges.
5. Restore grow-cycle age from RTC/NVS.
6. Determine light schedule state.
7. Ensure pump is off.
8. Apply fan minimum if needed.
9. Set LEDs.

If RTC time is invalid:

- Use monotonic fallback schedule from boot.
- Light schedule defaults to conservative 16h on / 8h off.
- System LED amber pulse.
- Do not block watering if moisture sensing is valid.

### 9.5 Light control

#### Default schedule

| Stage | Photoperiod | Ramp | Intensity target |
|---|---:|---:|---:|
| Germination pre-emergence | 0–8 h optional | none | off/low |
| Germination after emergence | 16 h | 30 min | 20–30% |
| Seedling | 16 h | 30 min | PPFD 140–210 |
| Vegetative | 16 h | 30 min | PPFD 245–350 |
| Flowering | 16 h | 30 min | PPFD 315–420 |
| Fruiting | 16 h | 30 min | PPFD 350–435 |

Light starts at a default time if RTC known:

```text
Lights on: 06:00 local
Lights off: 22:00 local
```

If no RTC/time setup:

```text
First boot time = start of light period
16h on / 8h off thereafter
```

#### Thermal derating

If air temperature is high:

| Condition | LED action |
|---|---|
| 28–30°C | No derate; fan increases |
| 30–32°C | Reduce LED up to 20% if temperature rising |
| >32°C | Reduce LED 30–60%; climate fault |
| >35°C | LED off/minimum; critical fault |

If LED heat sink sensor is added:

| LED temp | Action |
|---|---|
| <60°C | Normal |
| 60–70°C | Fan high / dim slightly |
| >70°C | Derate |
| >80°C | LED off fault |

### 9.6 Irrigation control

#### Inputs

- Normalized moisture.
- Moisture sensor validity.
- Reservoir low/not low.
- Leak detected/not detected.
- Pump calibration ml/s.
- Current stage.
- VPD.
- Time in photoperiod.
- Recent watering history.
- Pump current optional.

#### Decision loop

Run every 5 minutes for checks, but water only when conditions require.

Pseudo-code:

```pseudo
if leak_detected:
    pump.off()
    state = LEAK_DETECTED
    return

if reservoir_low:
    pump.off()
    state = LOW_WATER
    return

if moisture_sensor_invalid:
    pump.off()
    state = SENSOR_FAULT
    return

if pump_fault:
    pump.off()
    return

stage = plant_profile.stage(age_days)
thresholds = irrigation_thresholds(stage, vpd, time_of_day)

if moisture < thresholds.critical_dry:
    dose = thresholds.emergency_pulse_ml
    water_even_if_night = true
elif moisture < thresholds.dry and within_watering_window():
    dose = thresholds.normal_pulse_ml
else:
    pump.off()
    return

if daily_watered_ml + dose > daily_max_ml(stage):
    state = PUMP_FAULT_OR_WATERING_LIMIT
    pump.off()
    return

run_pump_for(dose / calibrated_ml_per_second)
wait(recheck_delay)
remeasure moisture

if moisture did not increase by minimum_expected_delta after N pulses:
    state = PUMP_FAULT
```

#### Daily maximums

Initial defaults:

| Stage | Daily max |
|---|---:|
| Seedling | 250 mL/day |
| Vegetative | 800 mL/day |
| Flowering | 1200 mL/day |
| Fruiting | 1800 mL/day |

These are safety caps, not target consumption. They should be adjusted after grow trials.

#### Pump timeout

- Absolute single-run max: 30 seconds, or lower depending on calibrated flow.
- Maximum pulses per hour: 3.
- Maximum pulses per day: stage-dependent.
- Pump always off on firmware crash/reset due to hardware pull-down.

### 9.7 Climate/fan control

Fan minimum:

| Stage | Lights on | Lights off |
|---|---:|---:|
| Seedling | 15–25% | periodic 5 min/hour |
| Vegetative | 20–35% | periodic 5–10 min/hour |
| Flowering | 25–45% | periodic 10 min/hour |
| Fruiting | 25–50% | periodic 10 min/hour |

Fan boosts:

| Condition | Action |
|---|---|
| RH >75% lights on | +15% |
| RH >85% | +30%, amber climate |
| VPD <0.5 kPa | +20% |
| Temp >28°C | +20% |
| Temp >30°C | +40% |
| Temp >32°C | max fan, LED derate |
| Fan tach missing | FAN_FAULT |

Avoid blasting seedlings directly. Fan should produce circulation, not wind stress.

### 9.8 LED status logic

| LED | Green | Amber | Red |
|---|---|---|---|
| Water | Reservoir OK | Low soon / level uncertain | Empty / low lockout |
| Moisture | In target | Dry soon / wet high | Sensor fault / critical dry / waterlogged |
| Light | Schedule active/normal | Thermal dimming / schedule uncertain | LED fault / over-temp shutdown |
| Climate | Temp/VPD OK | Outside preferred range | Critical temp/humidity fault |
| System | Normal heartbeat | Maintenance/calibration due | Self-test/fatal fault |

Blink patterns:

| Pattern | Meaning |
|---|---|
| Steady | OK |
| Slow pulse | Warning |
| Fast blink | User action needed |
| Double blink | Sensor fault |
| Off at night | Normal only if system LED heartbeat remains dim |

### 9.9 Calibration

V1 must separate hidden/developer calibration from user configuration.

Required calibrations:

| Calibration | Method |
|---|---|
| Moisture dry/wet | Factory/dev mode with chosen media |
| Pump ml/s | Run pump into measuring cylinder for 30 s |
| Reservoir low point | Fill-drain test |
| Fan min PWM | Find lowest reliable spinning duty |
| LED dim map | Measure PPFD grid at 25/50/75/100% |
| Temp/humidity sanity | Compare against reference sensor |
| Leak sensor | Wet test |

Calibration data stored in NVS/flash:

```json
{
  "moisture_raw_dry": 1234,
  "moisture_raw_wet": 2870,
  "pump_ml_per_sec": 3.8,
  "fan_min_pwm": 28,
  "led_ppfd_map": {
    "25": 120,
    "50": 240,
    "75": 360,
    "100": 480
  },
  "reservoir_low_adc": 600
}
```

### 9.10 Data logging

Store rolling logs locally:

- Sensor readings every 5–15 minutes.
- Watering events.
- Fault events.
- LED derating events.
- Reservoir-low events.
- Firmware version.
- Calibration version.

Minimum persistent log size:

- 7 days onboard.
- Export via serial/USB.
- Optional Wi-Fi telemetry.

### 9.11 Connectivity

V1 must work offline.

Optional connectivity modes:

| Mode | Status |
|---|---|
| USB serial logs | Required for dev |
| Local Wi-Fi setup | Optional |
| MQTT | Optional |
| Home Assistant discovery | Optional |
| Matter | Future |
| Cloud | Not V1 |

No control loop may depend on cloud.

---

## 10. Software Testing and Simulation

### 10.1 Testing philosophy

Every control rule should be testable without physical hardware.

Design firmware so the core logic lives in the `no_std` `control` crate, which compiles for the host
with stable Rust and is tested with `cargo test` (no target hardware, no Xtensa toolchain needed):

```text
hardware drivers        → trait mocks (host impls of the hal.rs traits)
sensor readings         → simulated
time                    → simulated (injected Clock trait)
control decisions       → asserted
```

### 10.2 Unit tests

Required unit tests:

| Module | Tests |
|---|---|
| DLI calculator | PPFD/DLI conversion correctness |
| Plant profile | Stage selection by age |
| Light scheduler | On/off/ramp behavior |
| LED derating | Derate thresholds |
| Moisture normalization | Raw-to-normalized mapping |
| Irrigation thresholds | Stage/VPD/time modifiers |
| Pump dose calculator | ml to seconds conversion |
| Pump safety | timeout, daily max, low water, leak lockout |
| VPD calculator | temp/RH to kPa |
| Fan controller | temp/RH/VPD duty behavior |
| LED status | state-to-pattern mapping |
| Fault priority | highest-priority state wins |
| Calibration store (flash) | defaults, missing/corrupt calibration |

### 10.3 Simulation tests

Create a simple plant/environment simulator sufficient for control validation.

Simulated variables:

```text
air_temp_c
relative_humidity
vpd_kpa
moisture_normalized
reservoir_ml
light_on
led_power_percent
fan_percent
pump_ml_per_sec
plant_stage
```

Simulated behaviors:

- Moisture declines faster during light period and high VPD.
- Pump increases moisture after delay.
- Reservoir decreases when pump runs.
- Fan slightly reduces humidity.
- LED increases heat.
- Leak sensor can be injected.
- Sensor failure can be injected.

Required scenarios:

| Scenario | Expected result |
|---|---|
| Normal 7-day seedling | No overwatering, light schedule stable |
| Normal 7-day fruiting | Moisture maintained, reservoir consumed |
| Reservoir empty | Pump lockout, water LED red |
| Moisture sensor stuck wet | No watering, sensor fault after plausibility window |
| Moisture sensor stuck dry | Pump capped by daily max, fault if no response |
| Pump disconnected | Current/timing fault if current sensor present; moisture no-rise fault otherwise |
| Leak detected | Immediate pump off, red water/system |
| Hot room | Fan high, LED derate, no runaway |
| Humid night | Fan pulses, no watering unless critical dry |
| RTC invalid | Safe schedule fallback, amber system |
| Power loss mid-watering | Pump off after reboot, event logged |

### 10.4 Hardware-in-loop tests

Build a HIL test fixture with:

- Programmable moisture input or digital mock.
- Reservoir switch simulator.
- Leak switch simulator.
- Pump dummy load.
- Fan tach simulator.
- LED dimming dummy input.
- Current measurement.
- UART log capture.

HIL tests:

- Flash firmware.
- Verify boot self-test.
- Toggle each sensor state.
- Confirm pump output never enables during leak/low-water.
- Confirm LED dimming command changes.
- Confirm fan PWM changes.
- Confirm status LED patterns.
- Confirm watchdog resets safely.

### 10.5 Continuous integration

Repo CI should run:

- Markdown lint.
- Firmware formatting (`cargo fmt --check`).
- Static analysis / lints (`cargo clippy -D warnings`).
- Host unit tests (`cargo test` on the `control` crate, stable Rust).
- Simulation tests (`sim` scenarios over the `control` crate).
- Schematic ERC if tool supports CLI.
- PCB DRC if tool supports CLI.
- BOM generation check.
- CAD file presence check.
- STL manifold check if feasible.
- Link/reference check for docs.

---

## 11. PCB and Electronics Verification

### 11.1 Design reviews

Before ordering PCB:

- Schematic review.
- Power budget review.
- Connector/pinout review.
- Protection review.
- Wet/dry isolation review.
- Firmware pin mapping review.
- PCB DRC/ERC pass.
- Trace current review.
- Thermal review for MOSFETs/regulators.
- Assembly review with mechanical CAD.

### 11.2 Bench bring-up plan

1. Visual inspection.
2. Continuity checks:
   - 24 V to GND no short.
   - 12 V to GND no short.
   - 5 V to GND no short.
   - 3.3 V to GND no short.
3. Power with current-limited bench supply.
4. Verify rails unloaded.
5. Verify regulator temperatures.
6. Flash test firmware.
7. Verify USB/UART.
8. Verify each sensor bus.
9. Verify each output with dummy load.
10. Verify pump MOSFET with real pump in water.
11. Verify fan PWM and tach.
12. Verify LED dimming with driver.
13. Verify status LED board.
14. Run 24-hour dry burn-in without water.
15. Run 24-hour wet-bay test with water but no plant.

### 11.3 PCB trace tests

Required artifacts:

- Trace-width calculation for each power path.
- Maximum current table.
- Thermal camera image at max pump/fan/LED-control load.
- Voltage-drop measurement under max load.
- MOSFET temperature measurement at 100% pump and fan.
- Regulator temperature measurement at worst-case ambient.
- Connector current rating table.

### 11.4 Electrical safety verification

Even with low-voltage DC:

- External PSU must be certified.
- Add input fuse.
- Add reverse polarity protection.
- No AC mains inside the grow unit.
- Drip loops required.
- Electronics bay splash-protected.
- Pump output fails off on MCU reset.
- Watchdog enabled.
- Brownout detector enabled.
- Fault states persist until safe conditions return; leak fault requires manual clear.

---

## 12. Mechanical Verification

### 12.1 CAD verification

Before printing/building:

- Full assembly CAD complete.
- Pot inserted/removed path checked.
- Reservoir inserted/removed path checked.
- Pump/filter access checked.
- Cable bend radius checked.
- Tubing path checked.
- LED height/clearance checked.
- Fan clearance checked.
- Electronics bay access checked.
- Tool access checked.
- Center of gravity checked with full reservoir and plant.
- Drip/leak path checked.

### 12.2 Printed part verification

Use tolerance coupons before printing large parts:

- Snap-fit test.
- Screw boss test.
- Heat-set insert test.
- Tube clip test.
- Diffuser slot test.
- Cable channel clip test.
- Reservoir rail/slide test.

Acceptance:

- No cracking.
- No excessive force required.
- No sharp edges near tubing.
- No loose fan/light mounts.
- Parts survive 40°C warm environment without warping.

### 12.3 Water path verification

Tests:

| Test | Method | Acceptance |
|---|---|---|
| Reservoir fill | Fill to max | No splash into dry bay |
| Reservoir removal | Remove when full | No more than minor drip into wet tray |
| Pump run | 100 cycles | No leaks |
| Tube disconnect simulation | Controlled failure | Water goes to tray/leak sensor |
| Overflow | Overfill intentionally | Overflow path avoids electronics |
| Leak sensor | Add small water amount | Pump locked out |

### 12.4 Thermal verification

Measure:

- Air near canopy.
- LED heat sink.
- Electronics bay.
- Driver case.
- Printed parts near LED.
- Reservoir water temperature.

Test cases:

| Case | Duration |
|---|---:|
| LED 50% + fan normal | 4 h |
| LED 100% + fan normal | 4 h |
| LED 100% + fan failed | until safety trip |
| Hot room 30°C | 4 h |
| Night fan off/pulse | 8 h |

Acceptance:

- No plastic deformation.
- LED derating triggers before unsafe temperatures.
- Electronics bay remains within component ratings.
- Canopy air does not exceed critical threshold without fault.

### 12.5 Acoustic verification

Measure dBA at 1 m front.

Modes:

- Night idle.
- Day normal.
- Fan max.
- Pump active.
- Fault mode.

Acceptance:

- Normal day ≤30 dBA.
- Night ≤25 dBA.
- Pump active ≤35 dBA if possible.
- No high-frequency PWM whine.

---

## 13. End-to-End Validation Plan

### 13.1 Validation phases

| Phase | Name | Goal |
|---|---|---|
| V0 | Research + requirements | Lock plant/hardware targets |
| V1A | Electronics dev kit | Validate sensors/outputs on breadboard |
| V1B | Firmware simulator | Validate control rules before PCB |
| V1C | PCB prototype | Validate board and harness |
| V1D | Mechanical alpha | Validate fit, service, water routing |
| V1E | Integrated dry run | 7+ days no plant |
| V1F | Wet run | 7+ days with water/media, no plant |
| V1G | Plant trial | 60–120 days |
| V1H | Release candidate | Docs, BOM, tests, known limitations |

### 13.2 Dry-run acceptance

No plant, no water or dummy loads.

Pass if:

- No firmware crash in 7 days.
- Light schedule stable.
- Logs persist.
- Status LEDs correct.
- Watchdog not repeatedly firing.
- Fan control stable.
- No overheating.

### 13.3 Wet-run acceptance

Water reservoir, pump, potting media, no plant.

Pass if:

- No leaks in 7 days.
- Pump doses are repeatable within ±25%.
- Leak sensor works.
- Reservoir low lockout works.
- Moisture loop does not oscillate excessively.
- No water enters dry bay.
- Pump/filter service is easy.

### 13.4 Plant-trial acceptance

Use one Carolina Reaper or similar hot pepper.

Minimum trial:

- 60 days for seedling/vegetative validation.
- 120+ days for fruiting validation.

Track:

- Germination date.
- Stage.
- Plant height.
- Canopy width.
- First flower date.
- First fruit set date.
- First ripe fruit date.
- Daily min/max temp/RH/VPD.
- Water used per day.
- Reservoir refill interval.
- Faults.
- Pruning events.
- Fertilizer events.
- Photos weekly.
- Final yield if fruiting completed.

Acceptance:

- Plant remains healthy.
- No chronic overwatering symptoms.
- No chronic wilting from dryback.
- At least one successful flowering/fruiting event under sufficient plant maturity.
- User intervention limited to refill/feed/prune/support.
- No water/electrical safety events.

---

## 14. Repository Plan

### 14.1 Repository structure

```text
tabletop-pepper-grower/
  README.md
  LICENSES/
    firmware-license.txt
    hardware-license.txt
    docs-license.txt

  docs/
    product-requirements.md
    plant-profile-hot-pepper.md
    safety.md
    assembly.md
    calibration.md
    operation.md
    troubleshooting.md
    validation-plan.md
    led-status-legend.md
    maintenance.md
    references.md

  firmware/
    Cargo.toml            # Rust workspace manifest
    rust-toolchain.toml   # pins the esp Rust channel (espup)
    .cargo/
      config.toml         # default target xtensa-esp32s3-none-elf + espflash runner
    control/              # no_std, platform-agnostic control logic — host-testable
      src/
      tests/
    controller/           # no_std esp-hal firmware binary for ESP32-S3
      src/
    sim/
      README.md
      src/
      scenarios/
      models/
    hil/
      README.md
      fixtures/

  electronics/
    pcb/
      kicad/
      gerbers/
      fabrication/
      ibom/
    wiring/
      wiring-diagram.svg
      harness-table.csv
    bom/
      bom.csv
      alternates.csv
    test/
      bringup.md
      pcb-verification.md

  mechanical/
    cad/
      source/
      step/
    stl/
      printable/
      prototypes/
    drawings/
    print-settings.md
    fit-tests.md

  validation/
    test-plans/
    logs/
    photos/
    ppfd-measurements/
    thermal/
    acoustic/
    grow-trials/

  scripts/
    bom_check.py
    dli_calculator.py
    pump_calibration.py
    moisture_calibration.py
    log_parser.py

  .github/
    workflows/
      ci.yml
      docs.yml
```

### 14.2 Licenses

Recommended:

| Asset | License |
|---|---|
| Firmware | Apache-2.0 or MIT |
| Hardware/PCB | CERN-OHL-S or CERN-OHL-P |
| Mechanical CAD/STL | CERN-OHL-S or CC BY-SA 4.0 |
| Documentation | CC BY 4.0 |
| Photos/logs | CC BY 4.0 unless personal info removed |

For a strongly reciprocal open hardware project, use CERN-OHL-S for hardware and mechanical files.

### 14.3 Documentation requirements

Minimum docs before v1 release:

- README with project overview.
- Safety notes.
- BOM.
- Wiring diagram.
- Assembly guide.
- Calibration guide.
- Firmware flashing guide.
- LED status legend.
- Maintenance schedule.
- Grow guide.
- Validation reports.
- Known limitations.

---

## 15. Work Breakdown: Professional Engineering Task Plan

### 15.1 Milestone M0 — Requirements lock

**Objective:** Convert this spec into repo issues and acceptance criteria.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M0-01 | Create repo skeleton | Public/private repo with folders | Structure matches section 14 |
| M0-02 | Add licenses | License files | Explicit license per asset type |
| M0-03 | Create issue templates | GitHub/GitLab templates | Bug, hardware, firmware, mechanical, docs |
| M0-04 | Lock physical envelope | Requirements doc | Max W/D/H, pot, reservoir fixed |
| M0-05 | Lock V1 non-goals | Scope doc | Camera/cloud/pH/EC excluded |
| M0-06 | Create risk register | `docs/risk-register.md` | Safety/scope risks tracked |

### 15.2 Milestone M1 — Plant profile and control rules

**Objective:** Make the plant-growing logic explicit and testable.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M1-01 | Create hot-pepper profile | `docs/plant-profile-hot-pepper.md` | Stage table and setpoints documented |
| M1-02 | Define DLI targets | DLI calculator script | Outputs PPFD for photoperiods |
| M1-03 | Define moisture thresholds | Control rules doc | Stage-specific thresholds |
| M1-04 | Define watering windows | Control rules doc | Early/mid-day watering logic |
| M1-05 | Define VPD calculation | Firmware utility | Unit-tested |
| M1-06 | Define fault priorities | State machine doc | Testable state priority |

### 15.3 Milestone M2 — Electronics proof of concept

**Objective:** Validate sensor and actuator choices before custom PCB.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M2-01 | Choose MCU dev board | BOM entry | ESP32-S3-class selected |
| M2-02 | Test temp/RH sensor | Bench log | Stable readings vs reference |
| M2-03 | Test moisture sensor | Calibration log | Dry/wet/media readings recorded |
| M2-04 | Test reservoir sensor | Bench log | Low-level detection reliable |
| M2-05 | Test leak sensor | Bench log | Pump lockout signal reliable |
| M2-06 | Test pump | Flow/noise log | ml/s and noise measured |
| M2-07 | Test fan | PWM/tach log | Min duty and noise measured |
| M2-08 | Test LED dimming | PPFD log | PPFD map at dim levels |
| M2-09 | Power budget | Spreadsheet | PSU sized with headroom |

### 15.4 Milestone M3 — Firmware core and simulation

**Objective:** Control logic works before hardware integration.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M3-01 | Create firmware project | Buildable firmware | CI builds |
| M3-02 | Hardware abstraction layer | Interfaces/mocks | Unit tests can run on host |
| M3-03 | Plant profile module | Code + tests | Age/stage tests pass |
| M3-04 | Light controller | Code + tests | Schedule/ramp/derate tests pass |
| M3-05 | Irrigation controller | Code + tests | Pulse/lockout tests pass |
| M3-06 | Climate controller | Code + tests | Fan/VPD tests pass |
| M3-07 | Safety controller | Code + tests | Fault priority tests pass |
| M3-08 | LED status module | Code + tests | Pattern map tests pass |
| M3-09 | Simulator | Rust host sim over `control` (Python allowed for models) | 10 required scenarios pass |
| M3-10 | Logging | Ring log | Exportable logs |

### 15.5 Milestone M4 — PCB and harness

**Objective:** Produce and verify the electronics board.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M4-01 | Schematic | KiCad schematic | Review passed |
| M4-02 | PCB layout | KiCad PCB | DRC clean |
| M4-03 | Connector pinout | Harness table | Polarity and voltage labeled |
| M4-04 | Protection circuits | Schematic section | Fuse/TVS/reverse protection present |
| M4-05 | Test points | PCB | All rails and controls accessible |
| M4-06 | Trace current report | Markdown/report | All high-current paths checked |
| M4-07 | Generate fabrication files | Gerbers/BOM/PNP | Fabrication package complete |
| M4-08 | PCB bring-up | Test report | Rails, MCU, sensors, outputs pass |
| M4-09 | HIL fixture | Test fixture | Automated fault tests pass |

### 15.6 Milestone M5 — Mechanical alpha

**Objective:** Build the tabletop open-frame structure and validate fit.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M5-01 | Full assembly CAD | Source CAD + STEP | All modules placed |
| M5-02 | Pot/reservoir selection | BOM + CAD models | Fit verified |
| M5-03 | Electronics bay design | CAD | Dry service bay isolated |
| M5-04 | Wet bay design | CAD | Reservoir/pump removable |
| M5-05 | Light mount | CAD/STL | Adjustable or fixed with clearance |
| M5-06 | Fan mount | CAD/STL | Guarded and isolated |
| M5-07 | Cable/tube routing | CAD | Drip loops and clips |
| M5-08 | Print tolerance coupons | STL + results | Fit confirmed |
| M5-09 | Alpha print/build | Photos + notes | Assembly feasible |

### 15.7 Milestone M6 — Integrated verification

**Objective:** Validate the system as a complete appliance.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M6-01 | Dry electrical run | Log | 7 days stable |
| M6-02 | Wet no-plant run | Log | 7 days no leaks |
| M6-03 | Pump cycle test | Log | 100 cycles no leaks/clogs |
| M6-04 | Thermal test | Report | LED derating works |
| M6-05 | Acoustic test | Report | Meets dBA targets |
| M6-06 | Status LED test | Video/table | All patterns correct |
| M6-07 | Fault injection | Report | Leak/low water/sensor faults pass |
| M6-08 | Calibration guide test | Doc update | New builder can calibrate |

### 15.8 Milestone M7 — Grow trial

**Objective:** Validate actual plant performance.

Tasks:

| ID | Task | Output | Acceptance |
|---|---|---|---|
| M7-01 | Start grow log | `validation/grow-trials/trial-001/` | Template complete |
| M7-02 | Record weekly photos | Photos | Dated, consistent angle |
| M7-03 | Record water use | CSV/log | Daily/weekly totals |
| M7-04 | Record climate | CSV/log | Min/max temp/RH/VPD |
| M7-05 | Record plant milestones | Log | Germination/flower/fruit/ripe dates |
| M7-06 | Inspect roots/media | Notes | No chronic waterlogging |
| M7-07 | Update thresholds | PR | Based on data |
| M7-08 | Release candidate review | Tag | Docs/tests complete |

---

## 16. Bill of Materials Constraints

This is not a final BOM. It is a constraint list that the final BOM must satisfy.

### 16.1 Core electronics

| Item | Required spec |
|---|---|
| MCU | ESP32-S3 dev module or custom PCB module |
| Temp/RH sensor | SHT31/SHT4x-class, I2C |
| Moisture sensor | Capacitive, corrosion-resistant, replaceable |
| Reservoir sensor | Float/optical/pressure; low-level reliable |
| Leak sensor | Conductive trace or sensor strip |
| Fan driver | PWM capable, tach input preferred |
| Pump driver | Logic-level MOSFET, flyback/protection, current sense optional |
| LED driver interface | PWM/0–10V matching selected LED driver |
| Status LEDs | 5 positions, dimmable |
| Power | Certified 24 VDC external PSU |
| Connectors | Locking, keyed, labeled |

### 16.2 Mechanical

| Item | Required spec |
|---|---|
| Pot | 8–10 L compact baseline, drain-capable, removable; 12–19 L optional full-yield variant |
| Reservoir | 2.5–4 L compact baseline, removable, cleanable; 4–6 L optional full-yield variant |
| Tubing | Food-safe or aquarium-safe, kink-resistant |
| Pump filter | Removable |
| Fan guard | Required |
| Frame | Stable with full reservoir and plant |
| Cable clips | Included |
| Drip tray | Required |
| Fasteners | Stainless near water |
| Printed material | PETG/ASA/ABS as appropriate |

### 16.3 Light

The BOM must not list a grow light unless it provides:

- Actual power draw.
- PPF or PPFD map.
- Dimming method.
- Spectrum or at least horticultural full-spectrum claim.
- Thermal mounting information.
- Electrical certification/safety data if available.

Reject lights that only advertise lumens, “equivalent watts,” or vague “red/blue plant lamp” claims.

---

## 17. Safety Requirements

### 17.1 Water/electricity safety

- No AC mains inside enclosure.
- External certified PSU only.
- Electronics above and isolated from wet zone.
- Drip loops on all cables.
- Leak tray below water system.
- Leak detection pump lockout.
- Reservoir cannot overflow into electronics.
- Pump fails off on MCU reset.
- Pump has daily max and runtime max.
- Use low-voltage connectors rated for environment.
- Use strain relief.
- Label voltage rails.

### 17.2 Thermal safety

- LED thermal path validated.
- LED derates or shuts down on high temperature.
- Printed materials near LED heat tested.
- Driver mounted with ventilation.
- Fan failure detected where possible.
- No combustible material directly touching LED heat sink.

### 17.3 Food/contact safety

- Avoid unknown plastics in reservoir.
- Avoid leaching materials.
- Use cleanable reservoir.
- Keep electronics contaminants away from water.
- Document that the project is DIY and not certified for commercial food production.

### 17.4 Child/pet safety

- Fan guard required.
- No exposed sharp metal.
- No easy access to pump impeller.
- Reservoir lid secure.
- Plant itself is a superhot pepper: include warning about capsaicin, pets, children, and handling.

---

## 18. Maintenance Plan

### 18.1 User maintenance

| Interval | Task |
|---|---|
| Every 2–7 days | Refill reservoir as indicated |
| Weekly | Inspect plant, pollinate flowers, check tubing |
| Every 2 weeks | Check pump filter, clean if needed |
| Monthly | Clean reservoir, inspect leak tray |
| Monthly | Inspect moisture probe and cable |
| Every grow cycle | Replace/sterilize media, clean pot/reservoir |
| As needed | Prune/support plant |

### 18.2 Developer/factory maintenance

| Interval | Task |
|---|---|
| On assembly | Calibrate moisture and pump |
| On firmware update | Run HIL smoke test |
| On sensor replacement | Recalibrate affected sensor |
| On pump replacement | Recalibrate ml/s |
| On LED replacement | Re-measure PPFD map |

---

## 19. V2 Expansion Roadmap

V2 features should be added only after V1 grow trials prove the basic system.

### 19.1 Camera/AI module

Potential capabilities:

- Plant health image log.
- Leaf yellowing detection.
- Droop detection.
- Pest/mold detection.
- Flower/fruit detection.
- Growth-rate tracking.

V2 caution:

- Camera health analysis can be wrong under grow-light color shifts.
- Lighting normalization and consistent image capture are required.
- It should advise or adjust within safe bounds, not directly override watering without sensor confirmation.

### 19.2 E-ink status module

Only add if users want more local information.

Display candidates:

- Water remaining estimate.
- Next light-off time.
- Temperature/humidity.
- Last watering.
- Fault text.

Do not add input controls unless user testing shows LEDs are insufficient.

### 19.3 Nutrient/pH/EC module

Only for advanced hydroponic variant.

Requirements:

- Replaceable probes.
- Calibration workflow.
- Storage solution.
- Manual confirmation before dosing.
- Chemical safety documentation.
- Strong isolation from V1 core.

---

## 20. Open Questions and Decisions to Lock

The following should be explicitly decided before ordering parts:

| Question | Recommended V1 decision |
|---|---|
| Exact footprint | 450–500 × 300–350 × 650–750 mm compact target; 550 × 400 × 850 mm full-yield upper target |
| Pot size | 8–10 L compact; 12–19 L optional full-yield variant |
| Reservoir size | 2.5–4 L compact; 4–6 L optional full-yield variant |
| MCU | ESP32-S3 |
| Firmware language | Rust (`no_std`, `esp-hal`); host-testable `control` crate; espup/Xtensa toolchain |
| Display | No display |
| User controls | Hidden reset/service only |
| Pump | Brushless DC submersible centrifugal |
| Fan | 80/92 mm quiet PWM PC fan |
| Light | 50–80 W dimmable full-spectrum white horticultural LED; 100 W only for larger/full-yield variant |
| Sensor set | temp/RH, moisture, reservoir low, leak, fan tach |
| Camera | Not V1 |
| pH/EC | Not V1 |
| Connectivity | Optional, offline-first |
| License | CERN-OHL-S hardware/mech, Apache-2.0 firmware, CC BY docs |

---

## 21. Acceptance Criteria for V1 Release

V1 can be tagged when all are true:

### Documentation

- README complete.
- BOM complete with alternates.
- Assembly guide complete.
- Calibration guide complete.
- LED status legend complete.
- Safety guide complete.
- Plant profile documented.
- Validation results included.

### Electronics

- PCB DRC/ERC clean.
- Board bring-up passed.
- Trace/current report complete.
- HIL fault tests passed.
- Pump/fan/light outputs verified.
- Sensors verified.
- Power/thermal measurements documented.

### Mechanical

- CAD and STL released.
- Assembly fit verified.
- Pot and reservoir removable.
- Electronics bay isolated.
- Water path tested.
- Print settings documented.
- Acoustic/thermal tests documented.

### Firmware

- Host unit tests pass.
- Simulation scenarios pass.
- HIL tests pass.
- Watchdog/brownout enabled.
- Pump fail-safe verified.
- Logs exportable.
- Calibration storage works.

### Grow validation

- At least one real plant trial documented.
- No safety failures.
- Moisture control kept plant alive and healthy.
- Reservoir/pump/LED/fan operated reliably.
- Known limitations listed.

---

## 22. Final V1 Recommendation

Build V1 as a **zero-config, LED-status, offline-first tabletop pepper grower**.

Use:

- ESP32-S3 controller.
- 50–80 W dimmable full-spectrum white horticultural LED for compact V1; 100 W only for larger/full-yield variant.
- 8–10 L removable compact pot, with 12–19 L optional full-yield variant.
- 2.5–4 L bottom reservoir, with 4–6 L optional full-yield variant.
- Quiet brushless DC submersible pump.
- Quiet 80/92 mm PWM fan.
- Capacitive soil moisture probe.
- SHT31/SHT4x-class temp/RH sensor.
- Reservoir low-level sensor.
- Leak sensor.
- 5 front status LEDs.
- External certified 24 VDC power supply.
- Electronics in upper dry bay.
- Water in bottom wet bay.
- Deterministic firmware with simulated and HIL-tested rules.

Do not include camera AI, e-ink, pH/EC, or species menus in V1. Those are valid expansion modules only after the core grow loop is proven.

The key engineering risks are:

1. Underpowered or poorly specified light.
2. Too-small pot.
3. Overwatering due to bad moisture calibration.
4. Water/electronics isolation failure.
5. Heat/noise from forcing a high-light fruiting crop into an overly compact tabletop frame.
6. Excessive yield expectations from the compact 8–10 L baseline.
7. Scope creep from AI/app/display/enclosure features.

Prioritize light quality, root volume, pump safety, moisture calibration, thermal behavior, and serviceability.
