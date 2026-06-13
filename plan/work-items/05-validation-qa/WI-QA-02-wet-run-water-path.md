# WI-QA-02 — Wet run & water-path verification

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-02, M6-03 |
| Depends on | WI-QA-01, WI-ME-08 |
| Spec refs | §13.3, §12.3 |
| Status | Not started |

## Objective

Run 7+ days with water, pump, and media (no plant) and execute the water-path test matrix to prove no
water reaches electronics and dosing is repeatable.

## Deliverables

- [ ] 7-day wet run log; pump dose repeatability within ±25%.
- [ ] Water-path matrix (§12.3): reservoir fill, reservoir removal, 100-cycle pump run, tube-disconnect
      simulation, intentional overflow, leak-sensor add-water test.
- [ ] Confirm leak sensor + reservoir-low lockout actually inhibit the pump.

## Acceptance criteria

- §13.3 wet-run conditions met: no leaks, no water in dry bay, repeatable doses, easy pump/filter service.
- 100 pump cycles with no leaks/clogs (spec §15.7 M6-03).
