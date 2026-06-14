<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# mechanical/stl — ⚠️ superseded (build123d arched model)

> **These STLs are from the SUPERSEDED design** — the build123d arched-frame model with a
> **pump**, **fan**, separate **pot**, in-base **electronics dry bay**, and **cable/tube channel**.
> They do **not** reflect the current V1, which was redesigned by
> [ECO-003](../../docs/ECO-003-v1-redesign.md) to a **two-pillar** form with **electronics in the top
> block** and **passive self-watering (no pump, no fan)**.

The authoritative current geometry is the OpenSCAD source
`mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad` (per-part STLs are exported by
`mechanical/cad/render_block.py` into the gitignored `mechanical/cad/exports/parts/`, all manifold;
geometry audited by `mechanical/cad/audit.py`).

**Stale here (do not print for V1):** `frame.stl`, `fan-mount.stl`, `pump-clip.stl`,
`electronics-dry-bay*.stl`, `pot*.stl`, `cable-channel.stl`, `light-mount.stl` (arched-era),
`leak-tray.stl`, etc., under `assembly/` and `printable/`.

**Still valid:** the **tolerance coupons** under `prototypes/` are generic fit coupons (heat-set,
screw boss, snap-fit, slot/rail) — most carry over; new coupons (pillar↔base socket, PCB standoff,
block body/lid seam) are added per [WI-ME-08](../../plan/work-items/04-mechanical/WI-ME-08-tolerance-alpha-build.md).

**Follow-up (later CAD pass):** regenerate the printable part STLs from the OpenSCAD model and
replace the superseded files here. Until then, treat everything except `prototypes/` as historical.
