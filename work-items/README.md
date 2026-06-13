# Work Items

This directory decomposes the [V1 Engineering Specification](../tabletop_pepper_grower_v1_spec_v1_1.md)
into **self-contained work items**, grouped by the specialist who owns each one.

Each work item is a single markdown file that can be picked up independently. It states its
objective, the spec sections it implements, concrete deliverables, acceptance criteria, and
dependencies on other work items. The goal: a specialist (or an agent) can open one file and
start working without re-reading the full 2000-line spec.

## How to read a work item

Every file follows the same header:

| Field | Meaning |
|---|---|
| Track | Owning specialist discipline |
| Milestone | Mapping back to spec §15 milestone (M0–M7), where applicable |
| Depends on | Work items that must land first |
| Spec refs | Sections of the spec this item implements |

Statuses are tracked inline (`Not started` / `In progress` / `Blocked` / `Done`). Treat the
checkbox lists as the definition of done.

## Tracks

| Track | Folder | Owner profile | Summary |
|---|---|---|---|
| Project & Repo | [`00-project-setup/`](00-project-setup/) | Project lead / DevOps | Repo skeleton, licensing, scope lock, risk register, CI |
| Plant Science | [`01-plant-science/`](01-plant-science/) | Horticulturist / CEA specialist | Lifecycle profile, light/DLI, watering, VPD, nutrients |
| Firmware | [`02-firmware/`](02-firmware/) | Embedded software engineer | Control logic, simulator, safety state machine, logging |
| Electronics | [`03-electronics/`](03-electronics/) | EE / PCB designer | Breadboard PoC, schematic, PCB, harness, bring-up, HIL |
| Mechanical | [`04-mechanical/`](04-mechanical/) | Mechanical / CAD engineer | Frame, bays, mounts, routing, print tolerances |
| Validation & QA | [`05-validation-qa/`](05-validation-qa/) | Test / QA engineer | Dry/wet runs, thermal, acoustic, fault injection, grow trial |
| Documentation | [`06-documentation/`](06-documentation/) | Technical writer | README, safety, assembly, calibration, grow guide |

## Suggested sequencing

The tracks are intentionally parallelizable, but there is a critical path:

```
00 Project setup ──► 01 Plant science ──► 02 Firmware (sim) ──┐
                                                              ├─► 05 Validation ──► Grow trial
03 Electronics (PoC ──► schematic ──► PCB ──► bring-up) ───────┤
04 Mechanical (CAD ──► alpha build) ──────────────────────────┘
06 Documentation runs continuously, finalized before release.
```

- **Plant science** must lock setpoints before firmware control logic is meaningful.
- **Firmware simulation** (`02-firmware`) can be fully validated before any PCB exists.
- **Electronics PoC** can run in parallel with plant science; the custom PCB waits on PoC results.
- **Validation** integrates the outputs of firmware + electronics + mechanical.
- **Grow trial** is the final gate and depends on a fully integrated unit.

## Release gate

V1 is tagged only when the acceptance criteria in spec §21 are all met. The
[validation track](05-validation-qa/) owns proving each one.
