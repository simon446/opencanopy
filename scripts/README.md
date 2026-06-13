# scripts/

Project tooling and calculators for OpenCanopy (spec §14.1). Plain Python, no heavy dependencies, so
they run locally and in CI.

| Script | Purpose | Used by |
|---|---|---|
| `bom_check.py` | Validate `electronics/bom/bom.csv`; fail if a grow light lacks the §16.3 required fields. | CI (WI-PS-06), Electronics |
| `dli_calculator.py` | Convert PPFD ↔ DLI for a photoperiod; estimate delivered PPFD from fixture PPF. **Delivered (`--selftest` in CI).** | Plant Science (WI-PL-02) |
| `pump_calibration.py` | Derive pump on-time ↔ delivered-volume calibration. | Firmware / Validation |
| `moisture_calibration.py` | Map raw capacitive readings to dry/wet calibration points. | Firmware / Validation |
| `log_parser.py` | Parse exported device logs into analyzable form. | Validation |

`bom_check.py` ships with the project-setup track; `dli_calculator.py` ships with Plant Science
(WI-PL-02). The remaining scripts are stubbed by their owning tracks. CI invokes `bom_check.py` and
`dli_calculator.py --selftest` on every PR.

> A reproducible **photometric delivery model** (the §23 DR-01 pre-order PPFD/DLI gate, WI-PL-06) lives
> with the validation track at
> [`validation/ppfd-measurements/model/`](../validation/ppfd-measurements/model/), not here.
