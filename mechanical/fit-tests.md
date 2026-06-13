# Fit tests — tolerance coupons (spec §12.2)

Print the coupon set in `stl/prototypes/` **before** committing the large parts,
then record results here and fold the winning clearances back into
`cad/source/opencanopy/params.py`. Coupons are graded (a few values bracketing the
nominal) so you can pick what fits your printer/material.

> **Note:** the *assembly* fit risk is already retired by simulation — see
> [`tolerance-analysis.md`](tolerance-analysis.md). Every clearance fit has ≥2 mm gap
> and survives a worst-case FDM error band (validated with FCL on the real models).
> These coupons therefore **confirm the process band** (does *this* printer fall
> within the assumed 0.3 mm growth / 0.2 mm position?) rather than discover fits — one
> coupon print validates the whole assembly.

## Coupon set (the seven §12.2 coupons)

| Coupon (STL) | Tests | Graded values |
|---|---|---|
| `coupon-snap-fit` | cantilever flex without cracking | single 3.0 mm finger + hook |
| `coupon-screw-boss` | M3 self-tapping pilot | pilot Ø 2.5 / 2.7 / 2.9 mm |
| `coupon-heatset-insert` | M3 brass heat-set bore | bore Ø 3.8 / 4.0 / 4.2 mm |
| `coupon-tube-clip` | 8 mm tube snap grip | gap 0.2 / 0.4 / 0.6 mm |
| `coupon-diffuser-slot` | 6 mm diffuser strip slot | slot 6.0 / 6.2 / 6.4 mm |
| `coupon-cable-channel-clip` | retaining-lip snap on a wire bundle | single (lip = channel) |
| `coupon-reservoir-rail` | sliding rail/slider | `CLEAR_SLIDE` = 0.30 mm |

## Acceptance (§12.2)

- [ ] No cracking (snap-fit flexes and returns; no whitening/fracture).
- [ ] No excessive force (parts assemble by hand, no tools/hammering).
- [ ] No sharp edges near tubing (tube clip and channel mouths deburred/radiused).
- [ ] No loose fan/light mounts (mount bosses hold M3 without spin-out).
- [ ] Parts survive 40 °C for 4 h without warping (heat-soak the coupon set).

## Results log

Fill one block per print run. Status stays **PENDING** until the physical alpha
print is made — these are the design-side coupons; the build is owned jointly with
the Validation track (WI-QA cluster) and the alpha build (WI-ME-08).

```text
Run:            <date>
Printer:        <model>
Material:       <PETG/ASA, brand, colour>
Nozzle/layer:   0.4 mm / 0.20 mm
Bed/hotend:     <°C / °C>

Coupon              Best value      Pass?   Notes
snap-fit            -               -       flex/crack check
screw-boss          pilot 2.? mm    -       which pilot held best
heatset-insert      bore 4.? mm     -       insert flush & square?
tube-clip           gap 0.? mm      -       grips 8 mm tube, removable
diffuser-slot       slot 6.? mm     -       strip slides, no rattle
cable-channel-clip  -               -       bundle retained, releasable
reservoir-rail      0.30 mm         -       slides freely, no slop
40 °C heat-soak     -               -       any warp/deform?
```

> **Status:** PENDING physical print. Coupons are modelled, exported, and
> manifold-clean (CI `stl_check`). Once printed, record the chosen clearances above
> and update `params.py`, then `python mechanical/cad/source/build.py` to propagate.
