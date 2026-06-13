# Build Ignition — Specialist Run Sequence

This document drives the OpenCanopy build. Each specialist is launched with a single **ignition
command** — one `claude` session that owns one discipline track and works every work item in that
track (under `work-items/`) in dependency order.

> [!IMPORTANT]
> **Run these in sequence, top to bottom. Do not run two ignition commands at the same time**
> unless a step is explicitly tagged **`CONCURRENCY OK`** below. The tracks share a dependency
> chain (plant targets → firmware logic → hardware bring-up → validation → grow trial); running
> them out of order or in parallel will produce work against unfinished upstream decisions.

Run each command, let that session finish its track, confirm its work items are checked off, then
start the next. Each session is scoped to read the whole plan but **only modify its own track's
deliverables**.

## Prerequisites

- Claude Code CLI installed and authenticated (`claude --version`).
- **Run every command from the project root (`opencanopy/`), not from `plan/`.** The paths below
  (`plan/work-items/...`) are relative to the project root, because that is where each session
  builds the actual project.
- `plan/` present on disk (it is gitignored by the project repo but read by every session).

---

## Step 1 — Project & Repo specialist  *(must run first; blocks everything)*

```bash
claude "You are the Project & Repo specialist for OpenCanopy. Read plan/work-items/README.md, then work the track in plan/work-items/00-project-setup/ in dependency order (WI-PS-01 first). Implement every deliverable, check off the boxes in each work-item file as you complete them, and set its Status to Done. Build the repo skeleton, licenses, issue/PR templates, locked requirements + scope docs, risk register, and CI. Stay strictly within the 00-project-setup track."
```

## Step 2 — Plant Science specialist

```bash
claude "You are the Plant Science / horticulture specialist for OpenCanopy. Read plan/work-items/README.md, then work plan/work-items/01-plant-science/ in dependency order (WI-PL-01 first). Produce the hot-pepper lifecycle profile, DLI/light targets + calculator, watering model, VPD/climate model, and nutrient guidance — all traceable to the cited sources R1–R17 in the spec. Check off deliverables and set Status as you go. This track is the single source of truth for plant targets; do not touch other tracks."
```

> **`CONCURRENCY OK`:** Steps 2 (Plant Science), 4 (Electronics — PoC stage only), and 5 (Mechanical
> — CAD stage only) each depend only on Step 1 and may be run concurrently **if** you intend to run
> several sessions at once. If in doubt, keep them sequential.

## Step 3 — Firmware specialist  *(needs Step 2 outputs)*

```bash
claude "You are the Embedded Firmware specialist for OpenCanopy. Read plan/work-items/README.md and the plant-science outputs, then work plan/work-items/02-firmware/ in dependency order (WI-FW-01 → WI-FW-02 → controllers → simulator). Build the host-testable control library, HAL + mocks, plant-profile/light/irrigation/climate/safety/LED-status modules, the simulator with all 11 required scenarios, logging, and calibration storage. Every control rule must have a passing host unit test. Check off deliverables and set Status. Firmware only — do not modify hardware or mechanical files."
```

## Step 4 — Electronics specialist

```bash
claude "You are the Electronics / PCB specialist for OpenCanopy. Read plan/work-items/README.md, then work plan/work-items/03-electronics/ in dependency order: breadboard PoC and power budget first, then schematic, PCB layout, harness, trace/thermal report, fabrication package, and finally board bring-up + HIL (which depends on the firmware safety state machine from Step 3). Check off deliverables and set Status. Electronics track only."
```

## Step 5 — Mechanical specialist

```bash
claude "You are the Mechanical / CAD specialist for OpenCanopy. Read plan/work-items/README.md, then work plan/work-items/04-mechanical/ in dependency order: full assembly CAD first, then pot/reservoir fit, dry electronics bay, wet bay, light mount, fan mount, cable/tube routing, and finally print-tolerance coupons + alpha build. Honor the locked envelope and the 'water fails downward, electronics live upward' rule. Check off deliverables and set Status. Mechanical track only."
```

## Step 6 — Validation & QA specialist  *(needs Steps 3, 4, 5 integrated)*

```bash
claude "You are the Validation & QA specialist for OpenCanopy. Read plan/work-items/README.md, then work plan/work-items/05-validation-qa/ in dependency order: dry electrical run, wet run + water-path matrix, thermal, acoustic, fault injection, calibration-guide validation, electrical/water safety sign-off, and finally the live grow trial. Treat safety (WI-QA-08) as a hard gate before the grow trial. Record evidence under validation/. Check off deliverables and set Status. Validation track only."
```

## Step 7 — Documentation specialist  *(finalize last)*

```bash
claude "You are the Documentation / technical-writing specialist for OpenCanopy. Read plan/work-items/README.md, then work plan/work-items/06-documentation/ in dependency order. Produce the README, safety guide, assembly + flashing guide, calibration/operation guide, LED status legend + troubleshooting, maintenance + grow guide, and references/BOM/validation reports. Pull facts from the completed track outputs; do not invent values. Check off deliverables and set Status."
```

> **`CONCURRENCY OK`:** Step 7 (Documentation) can be run continuously alongside later steps, but its
> deliverables can only be *finalized* once the tracks they document are complete.

---

## Tracking progress

Each work-item file carries a `Status` field and a deliverables checklist — those are the live
progress markers. Plan-side changes (status updates, refinements) are committed in this `plan/` repo;
implementation lands in the project repo.

## Release gate

V1 is tagged only when the acceptance criteria in `tabletop_pepper_grower_v1_spec_v1_1.md` §21 are
all met, as proven by the Validation & QA track.
