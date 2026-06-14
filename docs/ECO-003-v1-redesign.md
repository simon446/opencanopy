<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# ECO-003 — V1 redesign: two-pillar form, electronics-to-top, pump removal (passive watering)

**Type:** Engineering Change Order (product-wide; mechanical/maintainer-originated record)
**Date:** 2026-06-14
**Status:** Approved (maintainer) · Mechanical reconciliation **complete** · cross-track hand-offs **open**
**Owning track:** Mechanical · **Cross-track:** Firmware, Electronics, Plant Science, Validation/QA, Project & Repo
**Supersedes:** the arched-frame product model and the base-electronics two-zone layout.
**Relation:** sibling to [ECO-001](../electronics/analysis/ECO-001-fan-removal.md) (fan removal — folded in here).
**Spec touch:** §3.3/§3.5 (envelope, controls), §4.1/§4.2 (scope), §6.2 (zones), §7.2 (pump), §7.4 (fan),
§7.5 (sensors), §9.4/§9.5 (irrigation/safety), §17.1 (water/elec), §20 — changes to **locked** requirements;
see §"Cross-track hand-offs" for the spec/risk-register updates the Project track must ratify.

## 1. Decision

OpenCanopy V1 is redesigned to a **two-pillar Scandinavian tabletop form** and converted from active
to **passive self-watering**. Maintainer-approved 2026-06-14. The change has four parts:

1. **Form factor.** Replace the arched side-frame + bridge with a **low integrated base**, **two
   vertical wooden pillars**, and a **slim top LED block**. The grow light is centred over the plant.
2. **Electronics to the top.** The controller + LED driver move **out of the base into the top block**
   as a **small 1.6 mm PCB**, encapsulated inside the block on standoff bosses, with a **USB-C input**
   through the block rear face. The base has **no electronics bay**.
3. **No pump (passive watering).** The pump and all pump support (driver, connector, flow/lockout
   logic, daily/runtime caps, pump-fail-off) are **removed**. Watering is **passive semi-hydro**: a
   base reservoir + a removable slotted/perforated grow insert with a capillary wick path. The system
   **monitors and warns** (low water, abnormal moisture); it does not actuate water.
4. **No fan.** Confirms and folds in [ECO-001](../electronics/analysis/ECO-001-fan-removal.md).

**Retained, unchanged:** electronics and water are physically separated; open-frame / non-enclosed;
external low-voltage DC power (now USB-C, no AC mains inside); one pepper plant; 50–80 W dimmable
full-spectrum LED; no screen, no user controls; open-source CAD/HW/FW/docs.

**Changed locked values:** wet/dry separation is now **TOP (electronics) vs BOTTOM (water)** instead of
an in-base wall; **reservoir 4 L → 6 L**; the **separate 10 L pot** becomes an **integrated removable
grow insert**; **status LEDs 5 → 4** (Water, Moisture, Light, System — no Climate LED with no fan/active
climate control); power input **24 VDC barrel → USB-C** (pending electrical validation, §3 below).

## 2. Why passive (clarified intent)

The pump was the active irrigation actuator. Passive semi-hydro (reservoir + wick + slotted insert with
an air gap) is simpler, silent, has **no flood failure mode**, removes a whole class of safety logic
(leak→pump-lockout, fail-off, daily/runtime caps), and suits a single tabletop plant. The trade-off is
that passive delivery must keep a mature Carolina-Reaper-class plant adequately watered between manual
refills — this is **not yet proven** and is gated on a grow trial (see §5 open items, and Open Q below).

## 3. The things that could bite (flagged, not closed here)

- **USB-C power for a 50–80 W LED.** USB-C PD can supply up to 100 W (240 W EPR), but 50–80 W must be
  deliberately engineered (PD negotiation, cable/connector rating, driver input). **Mechanical only
  reserves the connector space + routing**; the **electronics track must validate feasibility** or
  fall back to a DC barrel input. (Open Q2.)
- **Passive watering adequacy.** Whether wick + slotted insert keeps root-zone moisture correct for a
  mature plant is a **plant-science** question (wick area, media choice, air gap, refill cadence vs the
  6 L reservoir). (Open Q3.) Mitigation: validate in the n=2 grow trial; design reserves space for an
  optional pump/aeration retrofit module if passive underperforms.
- **Reservoir stagnation** with no circulation: inert nutrients, opaque tank, documented refill/clean
  cadence (plant-science / docs).

## 4. Mechanical reconciliation (done in this change — mechanical track)

| Artifact | Change |
|---|---|
| `mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad` | Full rewrite to base + 2 pillars + top block + raised grow insert + 6 L reservoir; PCB encapsulated in the block (1.6 mm, mounting holes, USB-C rear port); no pump/fan/base-bay/in-base-wall. |
| `mechanical/cad/render_block.py`, `audit.py` | New parts/views; **geometry audit CLEAN** (no interpenetration > 80 mm³, none floating); LED centred over the grow module (offset 0.0/0.0). |
| `docs/mechanical-build.md`, `docs/fastening.md` | New architecture, pillar joints, encapsulated-PCB note. |
| `plan/work-items/04-mechanical/WI-ME-01..08` | Re-scoped: assembly (two-pillar), pot→insert, electronics-bay (obsolete in base), wet-bay (passive reservoir, no pump mount), cable (sensor leads up a pillar), fan-mount (obsolete, ECO-001), light-mount (passive heatsink, ECO-001), tolerance/alpha. |
| `mechanical/stl/` (build123d arch/pump/fan parts) | Flagged **superseded** (see `mechanical/stl/README.md`); to be regenerated from the OpenSCAD model in a later CAD pass. |
| `docs/product-requirements.md`, `docs/scope.md`, `docs/risk-register.md` | Updated by mechanical under this ECO with maintainer (Project & Repo) sign-off; see change-control trail in the risk register. |

## 5. Cross-track hand-offs (NOT changed here — flagged to owners)

- **Firmware:** remove pump/irrigation actuation. `WATERING`, `PUMP_FAULT`, `PUMP_TIMEOUT` states and
  the leak→pump-lockout / daily-runtime-cap / pump-fail-off logic become **N/A**. New behaviour is
  **monitor + warn**: `LOW_WATER`, `MOISTURE_LOW`, `MOISTURE_HIGH`, `SENSOR_FAULT`, `OVER_TEMP_LED`,
  `MAINTENANCE`. The fan channel/`FAN_FAULT` are already N/A (ECO-001). Affects
  `firmware/control/src/irrigation_controller.rs`, `app_state.rs`, `safety_controller.rs`, and the sim
  scenarios/tests. (Firmware track.)
- **Electronics:** remove the **pump driver + pump connector + pump current-sense** from the BOM/netlist
  (mark DNP-optional if a retrofit provision is wanted, as the fan was in ECO-001); **validate USB-C PD
  for the 50–80 W LED** or specify a DC input. Update power budget, harness, schematic, PCB. (Electronics
  track.) Note: the just-completed controller netlist/BOM still carry the pump.
- **Plant Science:** convert `docs/watering-model.md` from **active pump watering** to **passive
  semi-hydro** (wick area, media, air gap, refill cadence vs 6 L, stagnation mitigation); confirm a
  raised insert with ~5 L effective media + 6 L reservoir suffices for a mature plant, or set the
  documented trade-off. (Plant-science track — single source of truth for plant targets.)
- **Validation / QA:** drop pump flow-rate / timeout / fail-off / leak-lockout-of-pump and fan
  PWM/acoustic tests; add **passive wicking, reservoir drawdown, refill-cadence, root-zone moisture
  stability, overflow, wet/dry-separation leak, insert removal/clean, water-level calibration** tests.
  (QA track.)
- **Project & Repo:** ratify the locked-value changes in the **main spec** (`plan/…spec_v1_1.md`
  §7.2 pump, §7.4 fan, §6.2 zones, §3.3 envelope, §3.5 controls, §7.5 sensors, §9.4/§9.5) — this ECO and
  the updated `product-requirements.md`/`scope.md`/`risk-register.md` are the record; the engineering
  spec text is the Project track's to update.

## 6. Open questions (documented, non-blocking)

1. Does the low base fit 6 L reservoir + the insert without looking too tall? **Resolved in CAD:** the
   insert is a **raised planter** above a 135 mm base; reservoir is 6.6 L gross.
2. **Is USB-C PD sufficient for the 50–80 W LED**, or is a DC barrel required? — electronics.
3. **Does passive watering hold root-zone moisture for a mature chili?** — plant-science + grow trial.
4. Should the reservoir be removable, or is fill/drain service enough? — currently fill-from-top +
   insert lift-out for cleaning.
5. Pillars: real wood dowels vs printed faux-wood vs composite (affects the threaded-insert joint).
6. Fixed vs adjustable light height — **fixed** for V1 (clean form); adjustable is a V2 question.
