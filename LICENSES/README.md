# Licensing

OpenCanopy is multi-licensed **by asset type** — firmware, hardware/mechanical, and documentation
each carry a different open-source license, as recommended in spec §14.2 and locked in §20. Do **not**
collapse these into a single root `LICENSE`; the licenses differ deliberately.

## License files

| File | License | SPDX identifier |
|---|---|---|
| [`firmware-license.txt`](firmware-license.txt) | Apache License 2.0 | `Apache-2.0` |
| [`hardware-license.txt`](hardware-license.txt) | CERN Open Hardware Licence v2 — Strongly Reciprocal | `CERN-OHL-S-2.0` |
| [`docs-license.txt`](docs-license.txt) | Creative Commons Attribution 4.0 International | `CC-BY-4.0` |

## Subtree → license map

This is the authoritative mapping. Every tracked subtree falls under exactly one license.

| Path / asset | License | SPDX |
|---|---|---|
| `firmware/` (all source, tests, sim, HIL) | Apache-2.0 | `Apache-2.0` |
| `scripts/` (project tooling) | Apache-2.0 | `Apache-2.0` |
| `electronics/` (PCB: KiCad source, Gerbers, BOM, wiring) | CERN-OHL-S v2 | `CERN-OHL-S-2.0` |
| `mechanical/` (CAD source, STEP, STL, drawings) | CERN-OHL-S v2 | `CERN-OHL-S-2.0` |
| `docs/` (all documentation) | CC BY 4.0 | `CC-BY-4.0` |
| `README.md`, `CONTRIBUTING.md`, top-level prose | CC BY 4.0 | `CC-BY-4.0` |
| `validation/` test plans, reports, measurement data | CC BY 4.0 | `CC-BY-4.0` |
| `validation/photos/`, captured `validation/logs/` | CC BY 4.0 — **see condition below** | `CC-BY-4.0` |

### Why these three (and why CERN-OHL-S for hardware)

- **Firmware → Apache-2.0:** permissive, with an explicit patent grant; standard for embedded
  firmware that others may vendor into their own builds.
- **Hardware + mechanical → CERN-OHL-S v2 (Strongly Reciprocal):** the project lead's locked default
  (§20). Strong reciprocity keeps derived board/enclosure designs open. The project lead may override
  to `CERN-OHL-P` (permissive) or `CERN-OHL-W` (weakly reciprocal) per asset — record any such
  override here and in the risk register before it takes effect.
- **Documentation → CC BY 4.0:** attribution-only reuse of the written guides.

## Photos and logs — privacy condition

`validation/photos/` and any captured device/operator logs in `validation/logs/` are released under
**CC BY 4.0 only after personally identifying information has been removed** (faces, reflections,
people in frame, home/network identifiers, location metadata, EXIF GPS, account names, IPs/MACs).
Until that scrub is done, a given photo or log is **not** licensed for redistribution — strip the PII
first, then it falls under CC BY 4.0 like the rest of `validation/`.

## Applying licenses in new files

- Source files (`firmware/`, `scripts/`): add an SPDX header, e.g. `// SPDX-License-Identifier: Apache-2.0`.
- Hardware/mechanical exports where a header is impractical: the subtree mapping above governs.
- Docs: covered by the `docs/` mapping; no per-file header required.

This file is the REUSE-style mapping note required by WI-PS-02. If the directory layout grows, update
the table above so every subtree still resolves to exactly one license.
