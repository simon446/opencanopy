# OpenCanopy V1 — Risk Register

**Status:** Live · **Owner:** Project & Repo track · **Spec refs:** §22 (key engineering risks), §17 (safety)

This register tracks the engineering and safety risks called out by the spec, each with a likelihood,
impact, owning track, mitigation, and the work item(s) that close or verify it. It is the change-control
ledger for the project: any change to a locked requirement
([`product-requirements.md`](product-requirements.md)) or any reopened non-goal
([`scope.md`](scope.md)) must be logged here.

**Scales.** Likelihood: Low / Medium / High (chance the risk materializes if unmitigated).
Impact: Low / Medium / High / **Critical** (Critical = safety or project-killing). Work-item IDs link
to `../plan/work-items/`.

**Basis.** The plant-facing risks (R1 light, R2 root volume, R3 overwatering, R6 yield expectations)
are grounded in extension/research guidance — see [`references.md`](references.md) and spec §2. For
example, R3's overwatering/blossom-end-rot link comes from sources R5/R8/R15/R16; R1's DLI/PPFD framing
from R1/R2/R3.

> **🔄 REDESIGN — [ECO-003](ECO-003-v1-redesign.md) (2026-06-14).** V1 moved to a two-pillar form with
> **electronics in the top block** and **passive self-watering (no pump, no fan)**. This **changes or
> retires several risks below** — kept for ledger history, annotated, not deleted:
> **R3** overwatering → far lower (no pump; passive cannot flood); **R4/S2** isolation → now **top
> (electronics) vs bottom (water)**, not an in-base wall; **R5/S14/S18** (fan) → moot ([ECO-001](../electronics/analysis/ECO-001-fan-removal.md));
> **S5/S6/S7/S20** (pump lockout / fail-off / flood caps / impeller access) → **N/A, no pump**; **R8**
> (pump-fail-off HIL) → pump portion N/A. New redesign risks **R9–R14** are in **§C** below.

---

## A. Key engineering risks (spec §22)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / closing work item(s) |
|---|---|---|---|---|---|---|
| R1 | Underpowered or poorly specified grow light (low PPFD/DLI, vague "plant lamp") | Medium | High | Electronics + Plant Science | Lock DLI/PPFD targets; require real photometric/thermal/safety data in BOM; CI rejects underspecified lights; verify with PPFD map and grow trial | [WI-PL-02](../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md), [WI-EE-01](../plan/work-items/03-electronics/WI-EE-01-component-poc.md), [WI-PS-06](../plan/work-items/00-project-setup/WI-PS-06-ci-pipeline.md) (bom_check §16.3), verified by [WI-QA-07](../plan/work-items/05-validation-qa/WI-QA-07-grow-trial.md) |
| R2 | Too-small pot / insufficient root volume | Medium | High | Mechanical + Plant Science | Lock pot at top of compact band (10 L) with 12–19 L full-yield variant; fit-test removable pot | [WI-PS-04](../plan/work-items/00-project-setup/WI-PS-04-requirements-scope-lock.md), [WI-ME-02](../plan/work-items/04-mechanical/WI-ME-02-pot-reservoir-fit.md), [WI-PL-01](../plan/work-items/01-plant-science/WI-PL-01-lifecycle-profile.md) |
| R3 | Overwatering from bad moisture calibration | High | High | Firmware + Validation | Stage-specific moisture thresholds; watering windows + daily/runtime caps; documented calibration procedure tested on real substrate | [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md), [WI-FW-05](../plan/work-items/02-firmware/WI-FW-05-irrigation-controller.md), [WI-FW-11](../plan/work-items/02-firmware/WI-FW-11-calibration-storage.md), [WI-QA-06](../plan/work-items/05-validation-qa/WI-QA-06-calibration-guide-test.md) |
| R4 | Water/electronics isolation failure | Medium | **Critical** | Mechanical + Electronics | Two-zone wet/dry bay; drip loops; leak tray; leak-triggered pump lockout; wet-run + electrical-safety verification | [WI-ME-03](../plan/work-items/04-mechanical/WI-ME-03-electronics-bay.md), [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md), [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), [WI-QA-02](../plan/work-items/05-validation-qa/WI-QA-02-wet-run-water-path.md), [WI-QA-08](../plan/work-items/05-validation-qa/WI-QA-08-electrical-safety.md) |
| R5 | Heat / noise from forcing a high-light fruiting crop into a compact frame | Medium | Medium | Mechanical + Electronics | LED thermal path + ventilation; quiet PWM fan with tach; thermal and acoustic verification against limits | [WI-ME-05](../plan/work-items/04-mechanical/WI-ME-05-light-mount.md), [WI-ME-06](../plan/work-items/04-mechanical/WI-ME-06-fan-mount.md), [WI-EE-06](../plan/work-items/03-electronics/WI-EE-06-trace-thermal-report.md), [WI-QA-03](../plan/work-items/05-validation-qa/WI-QA-03-thermal.md), [WI-QA-04](../plan/work-items/05-validation-qa/WI-QA-04-acoustic.md) |
| R6 | Excessive yield expectations from the compact 8–10 L baseline | Medium | Low | Documentation + Plant Science | Set realistic yield expectations in scope + grow guide; offer full-yield variant for higher yield | [WI-PS-04](../plan/work-items/00-project-setup/WI-PS-04-requirements-scope-lock.md), [WI-PL-01](../plan/work-items/01-plant-science/WI-PL-01-lifecycle-profile.md), [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |
| R7 | Scope creep (AI / app / display / enclosure) | High | Medium | Project & Repo | Locked scope doc with explicit non-goals; expansion features deferred to §4.3 headers / V2; change control via this register | [WI-PS-04](../plan/work-items/00-project-setup/WI-PS-04-requirements-scope-lock.md) (scope.md), [WI-PS-05](../plan/work-items/00-project-setup/WI-PS-05-risk-register.md) (this register) |
| R8 | Hardware safety verification (pump fail-off, leak lockout) blocked on the unbuilt firmware safety state machine | Medium | High | Firmware + Electronics | EE-08 board bring-up + HIL fault tests depend on [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) (currently *Not started*); these are the **in-hardware** proofs of pump fail-off (S6) and leak lockout (S5). Sequence FW-07 ahead of board fabrication so HIL is ready when silicon arrives; meanwhile the fixture can be built and dry-validated against the firmware sim / HAL mocks. The pump-fail-off hardware guarantee (gate pull-down) is already in the schematic and does not depend on firmware. | **Blocker:** [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md); **blocked:** [WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md); verified [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |

---

## B. Safety risks (spec §17)

All §17 safety requirements are tracked here as risks, each with an owner and a mitigating work item.

### B.1 Water / electricity (§17.1)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / verifying work item(s) |
|---|---|---|---|---|---|---|
| S1 | AC mains present inside the unit | Low | **Critical** | Electronics | External certified 24 VDC PSU only; no mains inside (locked) | [WI-PS-04](../plan/work-items/00-project-setup/WI-PS-04-requirements-scope-lock.md), [WI-EE-02](../plan/work-items/03-electronics/WI-EE-02-power-budget.md), verified [WI-QA-08](../plan/work-items/05-validation-qa/WI-QA-08-electrical-safety.md) |
| S2 | Electronics not isolated above the wet zone | Low | **Critical** | Mechanical | Upper dry bay isolated from lower wet bay | [WI-ME-03](../plan/work-items/04-mechanical/WI-ME-03-electronics-bay.md), verified [WI-QA-02](../plan/work-items/05-validation-qa/WI-QA-02-wet-run-water-path.md) |
| S3 | Cable run wicks water into electronics (no drip loops) | Medium | High | Mechanical | Drip loops on all cables; routed routing/clips | [WI-ME-07](../plan/work-items/04-mechanical/WI-ME-07-cable-tube-routing.md), verified [WI-QA-02](../plan/work-items/05-validation-qa/WI-QA-02-wet-run-water-path.md) |
| S4 | Reservoir overflows into electronics; no leak tray | Medium | **Critical** | Mechanical | Leak tray below water system; reservoir cannot overflow toward electronics | [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md), verified [WI-QA-02](../plan/work-items/05-validation-qa/WI-QA-02-wet-run-water-path.md) |
| S5 | Leak not detected / pump not locked out on leak | Medium | **Critical** | Firmware | Leak detection drives immediate pump lockout in the safety state machine | [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), verified [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |
| S6 | Pump keeps running after MCU reset / brownout | Medium | High | Firmware | Pump output fails **off** on reset/brownout; watchdog/brownout enabled | [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), verified by HIL [WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md) + [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |
| S7 | Pump floods due to no daily/runtime max | Medium | High | Firmware | Daily-max and runtime-max limits enforced in irrigation + safety logic | [WI-FW-05](../plan/work-items/02-firmware/WI-FW-05-irrigation-controller.md), [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), verified [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |
| S8 | Connectors not rated/keyed; no strain relief | Low | Medium | Electronics | Locking, keyed, labeled low-voltage connectors; strain relief | [WI-EE-05](../plan/work-items/03-electronics/WI-EE-05-harness-pinout.md), verified [WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md) |
| S9 | Voltage rails unlabeled | Low | Medium | Electronics | Label voltage rails on board and harness | [WI-EE-03](../plan/work-items/03-electronics/WI-EE-03-schematic.md), [WI-EE-05](../plan/work-items/03-electronics/WI-EE-05-harness-pinout.md) |

### B.2 Thermal (§17.2)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / verifying work item(s) |
|---|---|---|---|---|---|---|
| S10 | LED thermal path not validated → overheating | Medium | High | Electronics + Mechanical | Validate LED thermal path and mounting; thermal report | [WI-EE-06](../plan/work-items/03-electronics/WI-EE-06-trace-thermal-report.md), [WI-ME-05](../plan/work-items/04-mechanical/WI-ME-05-light-mount.md), verified [WI-QA-03](../plan/work-items/05-validation-qa/WI-QA-03-thermal.md) |
| S11 | LED does not derate/shut down on high temperature | Medium | High | Firmware | Firmware derates or shuts down LED on canopy/driver over-temp | [WI-FW-04](../plan/work-items/02-firmware/WI-FW-04-light-controller.md), [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), verified [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |
| S12 | Printed material near LED heat degrades; combustible touching heatsink | Medium | High | Mechanical | Heat-tolerant material near LED (PETG/ASA/ABS); no combustible touching heatsink; clearance | [WI-ME-05](../plan/work-items/04-mechanical/WI-ME-05-light-mount.md), verified [WI-QA-03](../plan/work-items/05-validation-qa/WI-QA-03-thermal.md) |
| S13 | LED driver mounted without ventilation | Low | Medium | Mechanical + Electronics | Driver mounted with ventilation in dry bay | [WI-ME-03](../plan/work-items/04-mechanical/WI-ME-03-electronics-bay.md), [WI-EE-06](../plan/work-items/03-electronics/WI-EE-06-trace-thermal-report.md) |
| S14 | Fan failure not detected | Medium | Medium | Firmware | Fan tach monitored; fault on stall feeds climate/safety logic | [WI-FW-06](../plan/work-items/02-firmware/WI-FW-06-climate-controller.md), [WI-FW-07](../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md), verified [WI-QA-05](../plan/work-items/05-validation-qa/WI-QA-05-fault-injection.md) |

### B.3 Food / contact (§17.3)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / verifying work item(s) |
|---|---|---|---|---|---|---|
| S15 | Unknown/leaching plastics in reservoir | Low | Medium | Mechanical | Food-/aquarium-safe reservoir and tubing materials; no leaching materials | [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md), documented [WI-DOC-02](../plan/work-items/06-documentation/WI-DOC-02-safety.md) |
| S16 | Reservoir not cleanable; contaminants reach water | Low | Medium | Mechanical | Removable, cleanable reservoir; keep electronics contaminants away from water | [WI-ME-02](../plan/work-items/04-mechanical/WI-ME-02-pot-reservoir-fit.md), [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md) |
| S17 | Users assume the unit is certified for food production | Medium | Low | Documentation | Document explicitly: DIY, not certified for commercial food production | [WI-DOC-02](../plan/work-items/06-documentation/WI-DOC-02-safety.md) |

### B.4 Child / pet (§17.4)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / verifying work item(s) |
|---|---|---|---|---|---|---|
| S18 | Exposed fan blades (no guard) | Low | High | Mechanical | Fan guard required | [WI-ME-06](../plan/work-items/04-mechanical/WI-ME-06-fan-mount.md) |
| S19 | Exposed sharp metal edges | Low | Medium | Mechanical | No exposed sharp metal; deburr/cover; verified at alpha build | [WI-ME-01](../plan/work-items/04-mechanical/WI-ME-01-assembly-cad.md), [WI-ME-08](../plan/work-items/04-mechanical/WI-ME-08-tolerance-alpha-build.md) |
| S20 | Access to pump impeller | Low | Medium | Mechanical | No easy access to pump impeller; removable pump filter/guard | [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md) |
| S21 | Reservoir lid not secure | Low | Medium | Mechanical | Secure reservoir lid | [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md) |
| S22 | Capsaicin exposure — the plant is a superhot pepper | Medium | Medium | Documentation | Warning about capsaicin, pets, children, and safe handling in safety + grow guide | [WI-DOC-02](../plan/work-items/06-documentation/WI-DOC-02-safety.md), [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |

---

## C. Redesign risks (ECO-003 — two-pillar, top electronics, passive watering)

| # | Risk | Likelihood | Impact | Owning track | Mitigation | Mitigating / verifying work item(s) |
|---|---|---|---|---|---|---|
| R9 | Passive watering underperforms for a mature pepper (insufficient wick rate / effective media) | Medium | High | Plant Science + Mechanical | Size wick area + reservoir contact; ~5 L raised insert + 6 L reservoir; **validate in the n=2 grow trial**; reserve space for an optional pump/aeration retrofit if passive is inadequate | [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md), [WI-ME-04](../plan/work-items/04-mechanical/WI-ME-04-wet-bay.md), verified [WI-QA-07](../plan/work-items/05-validation-qa/WI-QA-07-grow-trial.md) |
| R10 | Roots waterlogged / no oxygen (passive semi-hydro) | Medium | High | Plant Science | Maintain an air gap; slotted insert + perforated floor + capillary media; no fully-submerged root mass | [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md), verified [WI-QA-07](../plan/work-items/05-validation-qa/WI-QA-07-grow-trial.md) |
| R11 | Reservoir stagnation (no circulation) | Low | Medium | Plant Science + Docs | Inert nutrients; opaque tank; documented refill/clean cadence; avoid organic sludge | [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md), [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |
| R12 | USB-C power insufficient for the 50–80 W LED | Medium | High | Electronics | Validate USB-C PD negotiation + cable/connector rating + driver input; **reserve a DC-barrel fallback** if PD is inadequate (mechanical reserves connector space only) | Electronics (power budget / schematic); flagged in [ECO-003](ECO-003-v1-redesign.md) §3 |
| R13 | Pillar joints wobble (two-pillar structure under the cantilevered light + plant load) | Medium | Medium | Mechanical | Pillar sockets (30 mm) + underside M4 into inserts + anti-rotation dowel + block set screws; validate with a load test (physics sim deferred) | [WI-ME-01](../plan/work-items/04-mechanical/WI-ME-01-assembly-cad.md), [WI-ME-08](../plan/work-items/04-mechanical/WI-ME-08-tolerance-alpha-build.md) |
| R14 | Effective media (~5 L raised insert) too small vs the old 10 L pot → reduced vigour/yield | Medium | Medium | Plant Science + Mechanical + Docs | Documented trade-off; size insert as large as the low base allows; set realistic yield expectations; grow-trial-gated | [WI-ME-02](../plan/work-items/04-mechanical/WI-ME-02-pot-reservoir-fit.md), [WI-PL-01](../plan/work-items/01-plant-science/WI-PL-01-lifecycle-profile.md), verified [WI-QA-07](../plan/work-items/05-validation-qa/WI-QA-07-grow-trial.md) |

---

## Change log

| Date | Change | Reference |
|---|---|---|
| 2026-06-14 | **ECO-003 (V1 redesign):** two-pillar form; **electronics moved to the top block**; **pump removed → passive self-watering**; reservoir 4 L→6 L; pot→integrated insert; status LEDs 5→4; power→USB-C; wet/dry separation now top/bottom. Retires/changes R3, R4, R5, R8, S2, S5, S6, S7, S14, S18, S20; adds R9–R14. Maintainer (Project & Repo) approved. | [ECO-003](ECO-003-v1-redesign.md) |
| 2026-06-13 | Register seeded with §22 engineering risks (R1–R7) and §17 safety risks (S1–S22). | WI-PS-05 |
| 2026-06-13 | Design review (spec §23): pump current-sense made required (DR-04) and battery-backed RTC added (DR-05); pre-order modeling + bench-characterization + surrogate-shakedown gates added (DR-01/02/03). | spec §23 |
| 2026-06-13 | Grow-trial decision: run **n=2 parallel units** (DR-03) — no single-unit-trial risk accepted. | spec §23 DR-03, WI-QA-07/10 |
| 2026-06-13 | Recorded R8: EE-08 bring-up/HIL (in-hardware pump fail-off + leak lockout proofs) is blocked on the unbuilt WI-FW-07 safety state machine. | R8, WI-EE-08, WI-FW-07 |

> To change a locked requirement or reopen a non-goal, add a row here describing the change, its risk
> impact, and the approving track before the change takes effect.
