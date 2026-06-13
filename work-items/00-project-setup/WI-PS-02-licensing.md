# WI-PS-02 — Licensing

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | M0-02 |
| Depends on | WI-PS-01 |
| Spec refs | §14.2, §20 (license row) |
| Status | Not started |

## Objective

Apply explicit, per-asset-type open-source licenses so the project is legally reusable.

## Deliverables

- [ ] `LICENSES/firmware-license.txt` — Apache-2.0 (preferred) or MIT.
- [ ] `LICENSES/hardware-license.txt` — CERN-OHL-S (strong reciprocal default for PCB + mechanical).
- [ ] `LICENSES/docs-license.txt` — CC BY 4.0.
- [ ] `REUSE`-style or README note mapping each subtree to its license.
- [ ] Photos/logs note: CC BY 4.0 only after personal info is removed.

## Acceptance criteria

- Every asset type from the spec §14.2 table has an explicit license.
- License choice for hardware/mechanical is CERN-OHL-S unless the project lead overrides.

## Notes

Firmware vs hardware vs docs licenses differ deliberately — do not collapse them into one root LICENSE.
