#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""
stl_check.py — Lightweight STL manifold / watertightness check for CI (spec §10.5, §12.2).

A mesh is considered manifold + watertight here when every undirected edge is shared by exactly two
triangles. This is a fast, dependency-free sanity gate (it does not replace a full mesh-repair tool),
intended to catch obviously broken/leaky STLs before they reach a printer.

Supports both ASCII and binary STL. Vertices are quantized before comparison to absorb float noise.

Usage:
    stl_check.py FILE.stl [FILE2.stl ...]   check specific files
    stl_check.py --dir mechanical/stl       check every *.stl under a directory (recursive)

Exit 0 = all checked meshes are manifold (or nothing to check); 1 = at least one is non-manifold.
"""
from __future__ import annotations

import argparse
import struct
import sys
from collections import defaultdict
from pathlib import Path

QUANT = 1e-5  # 0.01 µm; collapses near-coincident vertices from float export noise.


def _key(x: float, y: float, z: float) -> tuple[int, int, int]:
    return (round(x / QUANT), round(y / QUANT), round(z / QUANT))


def _parse_binary(data: bytes) -> list[tuple]:
    (n,) = struct.unpack_from("<I", data, 80)
    tris = []
    off = 84
    for _ in range(n):
        vs = struct.unpack_from("<12fH", data, off)
        off += 50
        tris.append((_key(*vs[3:6]), _key(*vs[6:9]), _key(*vs[9:12])))
    return tris


def _parse_ascii(text: str) -> list[tuple]:
    verts = []
    tris = []
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("vertex"):
            _, x, y, z = line.split()[:4]
            verts.append(_key(float(x), float(y), float(z)))
            if len(verts) == 3:
                tris.append(tuple(verts))
                verts = []
    return tris


def load_triangles(path: Path) -> list[tuple]:
    data = path.read_bytes()
    # Binary STLs are 84 + 50*n bytes; ASCII starts with "solid" and contains "facet".
    is_ascii = data[:5].lower() == b"solid" and b"facet" in data[:1024].lower()
    if is_ascii:
        return _parse_ascii(data.decode("utf-8", "replace"))
    return _parse_binary(data)


def check_file(path: Path) -> tuple[bool, str]:
    try:
        tris = load_triangles(path)
    except Exception as e:  # noqa: BLE001 - report any parse failure as non-manifold
        return False, f"FAIL  {path}: could not parse STL ({e})"
    if not tris:
        return False, f"FAIL  {path}: no triangles found"

    edges: dict[tuple, int] = defaultdict(int)
    for a, b, c in tris:
        for u, v in ((a, b), (b, c), (c, a)):
            edges[(u, v) if u <= v else (v, u)] += 1

    bad = sum(1 for count in edges.values() if count != 2)
    if bad:
        return False, f"FAIL  {path}: {bad} non-manifold edge(s) of {len(edges)} ({len(tris)} tris)"
    return True, f"OK    {path}: manifold, {len(tris)} tris, {len(edges)} edges"


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description="STL manifold/watertightness check.")
    ap.add_argument("files", nargs="*", help="STL files to check")
    ap.add_argument("--dir", help="recursively check every *.stl under this directory")
    args = ap.parse_args(argv)

    paths: list[Path] = [Path(f) for f in args.files]
    if args.dir:
        paths += sorted(Path(args.dir).rglob("*.stl"))

    if not paths:
        print("NOTICE: no STL files to check yet (mechanical STLs are produced by the Mechanical track).")
        return 0

    all_ok = True
    for p in paths:
        ok, msg = check_file(p)
        all_ok = all_ok and ok
        print(msg)
    print("STL check:", "PASS" if all_ok else "FAIL")
    return 0 if all_ok else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
