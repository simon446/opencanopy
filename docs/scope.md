# OpenCanopy V1 — Scope & Non-Goals (Locked)

**Status:** Locked (M0-05) · **Owner:** Project & Repo track · **Spec refs:** §4.1, §4.2, §4.3, §20

This document fixes what V1 **is** and — just as importantly — what it is **not**. Excluded features
are out of scope for V1 by decision, not by omission. Each exclusion below names where it may return:
an [§4.3 optional expansion header](#expansion-headers-provisioned-not-in-scope) and/or the V2 roadmap
(spec §19).

> **Change control:** Adding any non-goal back into V1 scope requires a [`risk-register.md`](risk-register.md)
> entry (scope creep is tracked risk #7) and Project & Repo sign-off.

**Why this scope (research basis).** V1 is deliberately *a small indoor device, not a commercial
greenhouse crop plan* (spec §2.3): the compact geometry trades maximum yield for table fit, and the
goal is **credible indoor fruiting, not commercial yield**. That framing is what justifies excluding
active climate control (a 22–25 °C room is adequate for hot peppers) and deferring pH/EC dosing,
camera/AI, and multi-plant support — none are needed to prove the core grow loop. The excluded items
return only through change control or the V2 roadmap (§19). Sources and the full rationale:
[`references.md`](references.md), spec §2.

---

## In scope for V1 (§4.1)

A zero-config, LED-status, offline-first tabletop grower for **one** hot-pepper plant:

- One plant; fixed hot-pepper profile.
- Broad-spectrum horticultural LED with dimming and an automated light schedule.
- Soil/substrate moisture sensing + automated pump watering.
- Reservoir-level sensing.
- Temperature/humidity sensing with **computed VPD**.
- Circulation fan control.
- Leak/flood safety sensor with pump lockout.
- 5-LED status interface.
- Local, deterministic firmware control (offline-first).
- Open-source firmware, PCB + wiring, and mechanical design.
- Simulation tests, hardware verification, and a grow-validation protocol.

The locked physical/hardware decisions for the above live in
[`product-requirements.md`](product-requirements.md).

## Out of scope for V1 (§4.2)

These are **explicitly excluded** from V1. None is a stretch goal to be added late; each is deferred to
the named expansion path.

| Excluded from V1 | Why excluded for V1 | Where it may return |
|---|---|---|
| Camera plant-health analysis | Adds optics, compute, and tuning unrelated to proving the core grow loop | §4.3 *Camera module* header; V2 §19.1 |
| Cloud AI plant diagnosis | Breaks offline-first; adds service + privacy surface | V2 §19.1 (camera/AI module) |
| Plant-species auto-ID | V1 is a fixed hot-pepper profile by design | Not roadmapped for V1/V2 core |
| Automatic pH dosing | Needs pH sensor + dosing pumps + reservoir chemistry | §4.3 *pH sensor* header; V2 §19.3 |
| Automatic EC / nutrient dosing | Needs EC sensor + dosing hardware + calibration | §4.3 *EC sensor* header; V2 §19.3 |
| Multi-plant support | Envelope, light map, and irrigation are sized for **one** plant | Not in V1/V2 core |
| Commercial yield optimization | V1 is a DIY appliance, not certified production (§17.3) | Out of project scope |
| Voice-assistant dependency | Must work with no app/assistant (offline-first) | Optional telemetry only |
| Required mobile app | UX must be usable with **no app and no normal controls** (§3.5) | Optional telemetry only |
| AC mains inside the grow unit | Safety: external certified PSU only (§17.1) | Never — permanent exclusion |
| On-device display (LCD/e-ink) | LED status is the V1 interface (§3.4–§3.5) | §4.3 *E-ink status module*; V2 §19.2 |
| Sealed enclosure / climate control (heater/humidifier/refrigeration) | Open-frame default; room 22–25 °C assumed (§3.6) | Not in V1/V2 core |

## Expansion headers provisioned, not in scope

V1 hardware **may** include connectors/pads so future modules can attach without a board respin, but
V1 must **not** be blocked on any of them and none is functional in V1:

- Camera module
- PAR / light sensor
- Load cell under pot
- EC sensor
- pH sensor
- CO₂ sensor
- E-ink status module
- External telemetry module

Provisioning a header does **not** bring its feature into V1 scope — the feature remains excluded per
the table above until a future version implements it.

## Scope-creep guardrail

Scope creep (AI / app / display / enclosure) is logged as the project's risk #7 in
[`risk-register.md`](risk-register.md). The default answer to "can we add X to V1?" for any item in the
out-of-scope table is **no** — reopen it only through change control, after the core grow loop is proven.
