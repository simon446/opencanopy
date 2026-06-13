# Tolerance & interference analysis (spec §12.1, §12.2)

This retires the **dimensional fit risk by simulation** instead of by trial-and-error
printing: the goal is that with the clearances as designed, fit is a non-issue for
any realistic FDM printer.

## Method — FCL on the actual models

`cad/source/collision_check.py` runs the **real geometry** (the same parametric parts
that produce the released STEP/STL, tessellated in their assembly-placed positions)
through **FCL** — the Flexible Collision Library (via `python-fcl`), the same
narrow-phase collision engine used in robotics (MoveIt). No hand-rolled geometry math.

For every pair of the 13 physical bodies it computes the exact minimum gap (and
penetration depth on contact), then applies a **configurable manufacturing error
margin** to ask the worst-case question:

> If every printed surface grew by `growth` and every part shifted by `position`,
> would a clearance fit close up?

```text
closure budget = (growth_A + growth_B) + (position_A + position_B)
a clearance pair survives  <=>  measured_gap >= closure_budget
```

Defaults model a well-tuned desktop FDM machine: `growth = 0.30 mm` per surface
(over-extrusion + elephant-foot), `position = 0.20 mm` (placement/warp) →
**1.0 mm closure budget**. Fastened/resting joints (lid-on-rim, drawer-on-rails,
bolted mounts) are expected to touch and are excluded from the clearance test.

```bash
.venv-cad/bin/python mechanical/cad/source/collision_check.py            # typical
.venv-cad/bin/python mechanical/cad/source/collision_check.py --growth 0.5 --position 0.3
```

## Result — robustness across printer error bands

| Error band (per surface) | Closure budget | Result |
|---|---:|---|
| Nominal (perfect print) | 0.0 mm | **PASS** — no real collisions |
| Typical FDM (0.30 / 0.20) | 1.0 mm | **PASS** |
| Harsh (0.50 / 0.30) | 1.6 mm | **PASS** |
| Extreme (0.80 / 0.30) | 2.2 mm | FAIL — 4 pairs by 0.2 mm |

Every clearance fit has **≥ 2 mm** gap. The design only begins to interfere once the
combined manufacturing error exceeds **~2 mm** — roughly **4× typical FDM**. The
limiting (tightest, 2 mm) interfaces are: dry-bay↔frame, dry-bay↔light-mount,
lid↔frame, reservoir↔status-diffuser.

## Interferences found and fixed

Running the sim on the first model surfaced **8 real interferences** that eyeballing
the CAD had missed; all are now resolved (the model asserts clean at nominal):

| Pair | Was | Cause | Fix |
|---|---|---|---|
| reservoir ↔ leak-tray | −5 cm³ | drawer sat 3 mm into the cradle rails | rest it on the rails |
| lid ↔ dry-bay rim | −11 cm³ | no lid rebate | walls stop one lid-thickness short |
| cable-channel ↔ pot-tray | 24 mm | channel base drove through the tray | raise channel base above the tray rim |
| dry-bay ↔ frame | 8 mm | back wall poked past the rear top rail (off-by-`EXTRUSION/2`) | correct `BAY_BACK_Y` to clear the rail |
| lid ↔ frame | — | same root cause | (same fix) |
| reservoir ↔ frame | 11 mm | bottom **front** rail blocked the drawer | open the frame front (drawer + pot access) |
| light-mount ↔ dry-bay | 5 mm | arm/eyelet `align` bug raised the carrier into the bay floor | anchor carrier to local z 0–12 |
| fan-mount (unattached) + ↔ pot | — / 2 mm | fan had no frame member to bolt to; once attached it clipped the pot | add a rear cross-rail at fan height, raise fan above the pot rim |

## Snap-fit strain (material check, not collision)

FCL answers geometry, not whether a flexing part cracks. The one functional snap is
the tube clip (compliant silicone tube, forgiving) and the snap-fit coupon. The
coupon is sized so the deflection-to-assemble stays within PETG's allowable bending
strain, using the standard constant-section cantilever relation:

```text
eps_max = 3 * y * t / (2 * L^2)
        = 3 * 1.2 * 2.5 / (2 * 18^2)  = 1.4 %      (PETG allowable ~ 2-3 %)
```

(The first coupon draft had a 4 mm undercut → 5.6 % → would have whitened/cracked;
the analysis caught it and it was reduced to a 1.2 mm undercut.)

## What simulation does *not* cover

These remain genuine physical confirmations (material/process, not geometry), but the
material selection already provides the margin:

- **40 °C warp (§12.2):** addressed by material choice — PETG/ASA (Tg ≈ 80 °C / >100 °C)
  vs the 40 °C requirement; PLA is excluded from heat/humidity zones (`print-settings.md`).
- **Layer-adhesion / real cracking:** the snap strain is within allowable, but actual
  layer bonding is print-quality-dependent — the snap coupon confirms it in one print.
- **Surface finish / burrs near tubing:** a visual/tactile check at assembly.

## Bottom line

The dimensional-fit and interference risk is **retired by simulation**: ≥2 mm
clearance everywhere, validated on the real models with a worst-case FDM margin and
margin to spare. The tolerance coupons (`fit-tests.md`) are therefore a **process-band
confirmation** (does *this* printer fall within the assumed 0.3/0.2 mm band?) rather
than a fit discovery — one coupon print confirms the whole assembly, not each joint.
