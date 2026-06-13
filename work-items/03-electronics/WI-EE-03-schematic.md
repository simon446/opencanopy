# WI-EE-03 — Schematic & protection

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-01, M4-04 |
| Depends on | WI-EE-01, WI-EE-02 |
| Spec refs | §7.9, §7.10, §11.1, §17.1 |
| Status | Not started |

## Objective

Capture the full controller schematic in KiCad, including all protection circuits and the
fail-safe pump drive.

## Deliverables

- [ ] KiCad schematic covering MCU, all sensor buses, fan PWM/tach, pump driver, LED dim interface,
      status LED board connector, and expansion headers (camera/PAR/load-cell/pH/EC, unpopulated).
- [ ] Protection: input fuse, reverse-polarity protection, TVS, flyback on inductive loads (§17.1).
- [ ] Pump MOSFET with **gate pull-down** so pump fails OFF on MCU reset/crash (§9.6, §11.4).
- [ ] Logic-level MOSFETs at 3.3 V gate, or gate driver (§7.10).
- [ ] Schematic passes ERC; design review per §11.1 completed.

## Acceptance criteria

- ERC clean; review checklist (§11.1) signed off.
- Fuse, reverse-polarity, TVS, and pump-fail-off present and labeled.
