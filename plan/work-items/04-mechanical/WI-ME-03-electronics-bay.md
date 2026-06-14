# WI-ME-03 — Electronics dry bay

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-03 |
| Depends on | WI-ME-01 |
| Spec refs | §6.2, §8.4, §17.1 |
| Status | **Superseded by [ECO-003](../../../docs/ECO-003-v1-redesign.md)** — electronics moved into the top LED block |

> **🔄 Superseded by [ECO-003](../../../docs/ECO-003-v1-redesign.md).** There is no separate electronics
> bay in V1. The electronics are now a **small 1.6 mm controller + driver PCB encapsulated inside the
> top LED block** (on standoff bosses, USB-C through the block rear face). With the pump removed, power
> distribution shrinks to USB-C + the LED driver. **New mechanical PCB budget published to
> [WI-EE-04](../03-electronics/WI-EE-04-pcb-layout.md): ≈ 62 × 44 mm, 1.6 mm, 4 mounting holes**, fitting
> the block bay (was 120 × 90 mm) — the board must shrink (ECO-003 hand-off). The deliverables below are
> the superseded separate-bay design, kept for history.

## Objective

*(Superseded.)* Originally: design a separate dry electronics bay. Now: the top LED block **is** the
electronics enclosure — see [WI-ME-01](WI-ME-01-assembly-cad.md) and the block in the CAD model.

## Deliverables

- [x] CAD for the dry bay housing controller PCB, LED driver, power distribution, status wiring
      (`electronics_bay.build_dry_bay()` + `build_dry_bay_lid()`), topmost module of the stack.
- [x] Heat-set standoffs on a 4-point pattern (no board flex); top-removable lid → serviced **without**
      opening the wet bay (§8.4).
- [x] Splash protection (drop-lip lid); cable entries are raised grommet collars + drip-loop hook posts
      so water cannot track in (§8.5, §17.1).
- [x] Fits a reserved **120 × 90 mm** controller-PCB envelope + 110 × 40 mm driver footprint, published
      to [WI-EE-04](../03-electronics/WI-EE-04-pcb-layout.md) as the mechanical budget (see note).

## Acceptance criteria

- Dry service bay isolated from wet zone (spec §15.6 M5-03). ✅ Bay floor at z 608 mm; zone-separation
  assertion proves no overlap with the wet band (≤180 mm).
- Electronics accessible without touching the reservoir/pump. ✅ Top lid only.

## Notes

WI-EE-04 (PCB outline) is not yet frozen — its layout report reciprocally states the board outline +
mount-hole pattern are *"coordinated with the mechanical electronics-bay before fabrication"*. So the
bay is designed to a generous reserved board envelope (`PCB_W`/`PCB_D` in `params.py`) which is the
agreed coordination point, published back to the Electronics track via `cad-verification-checklist.md`.
When the KiCad board lands, confirm fit and update those change-controlled params if needed, then
re-run `build.py`.
