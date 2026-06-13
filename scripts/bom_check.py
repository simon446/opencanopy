#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""
bom_check.py — Validate the OpenCanopy bill of materials against the spec constraints.

Primary gate (spec §16.3): a row identified as a grow light MUST carry real engineering data, not
marketing claims. A light is REJECTED if it is missing any of:

    - power_w              actual power draw (watts)
    - ppf_or_ppfd          PPF (µmol/s) or a PPFD map reference
    - dimming              dimming method (e.g. PWM, 0-10V)
    - spectrum             horticultural full-spectrum claim / spectral data
    - thermal_mounting     thermal mounting / heatsinking information

...and it is REJECTED if the spectrum/marketing fields advertise ONLY lumens, "equivalent watts",
or a vague "red/blue plant lamp" claim with no horticultural spectrum data.

`electrical_cert` is checked as "if available" (spec §16.3) — a warning, not a failure.

Usage:
    bom_check.py [BOM_CSV]        validate a BOM file (default: electronics/bom/bom.csv)
    bom_check.py --selftest       run built-in pass/fail fixtures (used by CI)
    bom_check.py --strict ...     also fail if the BOM contains no grow-light row at all

Exit code 0 = pass, 1 = fail. A missing BOM file is a pass-with-notice (the BOM is built later),
unless --strict is given.
"""
from __future__ import annotations

import argparse
import csv
import io
import sys
from pathlib import Path

DEFAULT_BOM = Path("electronics/bom/bom.csv")

# Fields a grow-light row must provide (spec §16.3). electrical_cert is "if available" -> warn only.
REQUIRED_LIGHT_FIELDS = ["power_w", "ppf_or_ppfd", "dimming", "spectrum", "thermal_mounting"]
ADVISORY_LIGHT_FIELDS = ["electrical_cert"]

# A row is treated as a grow light if its category/type/description matches one of these.
LIGHT_MARKERS = ("grow light", "grow-light", "growlight", "led light", "horticultural", "grow led")

# Marketing-only spectrum claims that are NOT acceptable on their own (spec §16.3).
FORBIDDEN_ONLY = ("lumen", "equivalent watt", "equiv watt", "red/blue", "red blue", "plant lamp")
# Evidence that a real horticultural spectrum is described.
SPECTRUM_OK = ("full-spectrum", "full spectrum", "spd", "spectral", "nm", "kelvin", "horticultural", "par")


def _is_light(row: dict[str, str]) -> bool:
    hay = " ".join(
        (row.get(k, "") or "") for k in ("category", "type", "subsystem", "description", "item", "name")
    ).lower()
    return any(m in hay for m in LIGHT_MARKERS)


def _norm(row: dict[str, str]) -> dict[str, str]:
    return {(k or "").strip().lower(): (v or "").strip() for k, v in row.items()}


def check_light_row(row: dict[str, str]) -> tuple[list[str], list[str]]:
    """Return (errors, warnings) for a single grow-light row."""
    r = _norm(row)
    errors: list[str] = []
    warnings: list[str] = []

    for field in REQUIRED_LIGHT_FIELDS:
        if not r.get(field):
            errors.append(f"missing required field '{field}'")

    spectrum = r.get("spectrum", "").lower()
    if spectrum:
        has_forbidden = any(f in spectrum for f in FORBIDDEN_ONLY)
        has_real = any(s in spectrum for s in SPECTRUM_OK)
        if has_forbidden and not has_real:
            errors.append(
                f"spectrum claim '{r.get('spectrum')}' is marketing-only "
                "(lumens / equivalent-watts / vague red-blue) with no horticultural spectrum data"
            )

    power = r.get("power_w", "")
    if power:
        try:
            if float(str(power).lower().replace("w", "").strip()) <= 0:
                errors.append(f"power_w '{power}' must be a positive number of watts")
        except ValueError:
            errors.append(f"power_w '{power}' is not a number (actual draw required, not 'equivalent')")

    for field in ADVISORY_LIGHT_FIELDS:
        if not r.get(field):
            warnings.append(f"no '{field}' provided (acceptable per §16.3 'if available', but prefer to include)")

    return errors, warnings


def validate_rows(rows: list[dict[str, str]], *, strict: bool) -> tuple[bool, list[str]]:
    msgs: list[str] = []
    light_rows = [row for row in rows if _is_light(row)]

    if not light_rows:
        msg = "no grow-light row found in BOM"
        if strict:
            msgs.append(f"ERROR: {msg} (a final BOM must include a compliant grow light, §16.3)")
            return False, msgs
        msgs.append(f"NOTICE: {msg} — skipping light check (BOM still in progress)")
        return True, msgs

    ok = True
    for i, row in enumerate(light_rows, 1):
        label = (row.get("item") or row.get("name") or row.get("description") or f"light#{i}").strip()
        errors, warnings = check_light_row(row)
        for w in warnings:
            msgs.append(f"WARN  [{label}]: {w}")
        for e in errors:
            ok = False
            msgs.append(f"ERROR [{label}]: {e}")
        if not errors:
            msgs.append(f"OK    [{label}]: grow light satisfies §16.3 required fields")
    return ok, msgs


def validate_csv_text(text: str, *, strict: bool) -> tuple[bool, list[str]]:
    rows = list(csv.DictReader(io.StringIO(text)))
    if not rows:
        return (not strict), ["NOTICE: BOM is empty"] if not strict else ["ERROR: BOM is empty"]
    return validate_rows(rows, strict=strict)


# --- Self-test fixtures (exercised in CI so the gate is proven even before a real BOM exists) ----

_GOOD = (
    "item,category,power_w,ppf_or_ppfd,dimming,spectrum,thermal_mounting,electrical_cert\n"
    "Compact grow light,grow light,60,150 PPFD @300mm (see ppfd-measurements/),PWM,"
    "full-spectrum white 3500K horticultural,aluminum heatsink + M3 standoffs,CE/UL listed\n"
)
_BAD_MISSING = (
    "item,category,power_w,ppf_or_ppfd,dimming,spectrum,thermal_mounting\n"
    "Mystery LED,grow light,60,,PWM,full-spectrum,heatsink\n"  # missing ppf_or_ppfd
)
_BAD_MARKETING = (
    "item,category,power_w,ppf_or_ppfd,dimming,spectrum,thermal_mounting\n"
    "Cheap lamp,grow light,150W equivalent,n/a,none,'red/blue plant lamp 2000 lumens',none\n"
)


def selftest() -> int:
    cases = [
        ("good light passes", _GOOD, False, True),
        ("missing PPF/PPFD fails", _BAD_MISSING, False, False),
        ("marketing-only claim fails", _BAD_MARKETING, False, False),
    ]
    failures = 0
    for name, text, strict, expect_ok in cases:
        ok, msgs = validate_csv_text(text, strict=strict)
        status = "PASS" if ok == expect_ok else "FAIL"
        if ok != expect_ok:
            failures += 1
        print(f"[selftest] {status}: {name} (expected ok={expect_ok}, got ok={ok})")
        for m in msgs:
            print(f"           {m}")
    print(f"[selftest] {'ALL PASSED' if failures == 0 else f'{failures} FAILED'}")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description="Validate the OpenCanopy BOM (spec §16.3 grow-light gate).")
    ap.add_argument("bom", nargs="?", default=str(DEFAULT_BOM), help="path to bom.csv")
    ap.add_argument("--selftest", action="store_true", help="run built-in pass/fail fixtures and exit")
    ap.add_argument("--strict", action="store_true", help="fail if no compliant grow-light row exists")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    path = Path(args.bom)
    if not path.exists():
        print(f"NOTICE: {path} not found — nothing to check yet (BOM is produced by the Electronics track).")
        return 1 if args.strict else 0

    ok, msgs = validate_csv_text(path.read_text(encoding="utf-8"), strict=args.strict)
    for m in msgs:
        print(m)
    print("BOM check:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
