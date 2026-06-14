# Fastening & assembly strategy

How the parts join, and why. This addresses the "how is it held together?" question for
the open-frame unit (base cabinet + two side-frame arches + top light bridge + panels).

## Why it can't be a single print

The unit is **480 × 320 × 680 mm**. Consumer FDM beds are ~220–350 mm, so a one-piece
print is impossible on normal hardware; a 500 mm+ printer is rare and expensive, the
print would run days with high failure risk, and you'd lose all service access. So the
unit is **printed (and/or cut) in modular panels and bolted together** — the standard
approach for large printed enclosures/frames.

## Primary method — heat-set inserts + machine screws

The de-facto standard for quality printed assemblies (used by Voron, Prusa enclosures,
most printed-enclosure designs) and what V1 uses:

- **M3 brass heat-set threaded inserts** melted into bosses on one part, with **M3
  socket-head cap screws** through clearance holes (Ø3.4 mm) on the mating part. Strong,
  repeatable, and **re-serviceable** (unlike glue or one-shot snap-fits).
- **Dowel pins** (3–4 mm) at panel seams for alignment and to carry shear, so the screws
  aren't the only thing locating the parts.
- **Stainless steel** screws anywhere near water (§8.3 / §16.2).

Tolerances for the bosses/holes are the ones the tolerance coupons validate
(heat-set bore, screw boss — see `mechanical/fit-tests.md`).

### Alternatives (and when)

| Method | Use | Notes |
|---|---|---|
| Heat-set insert + M3 screw | **all structural + serviceable joints** | primary |
| Captive (trap) hex nut + screw | where a heat-set tool isn't available | print a nut pocket |
| Dowel pin (+ light glue) | alignment at seams | not load-bearing alone |
| CA / epoxy glue | permanent decorative seams only | **not** for serviceable/structural joints |
| Snap-fit | tool-free covers (e.g. clip lids) | wears with cycles; not for primary structure |

Glue-only / plug-and-glue is avoided for the frame because the unit must stay
**serviceable** (remove reservoir, pump, electronics, LED — §8.4).

## Per-joint scheme

| Joint | Fastening |
|---|---|
| Side-frame arch ↔ base (×4 feet) | each arch foot **tenons 26 mm into a base socket** + **2 dowel pins** (alignment/shear) + **1 hidden M4** from the underside into a heat-set insert in the foot. The base underside is counterbored for straight screwdriver access. |
| Side-frame arch ↔ light bridge | **bridge tongues** into the arch-top sockets + dowel pins + accessible **M3/M4** screws from the underside/rear. |
| Base panels (front/sides/floor/deck) | M3 inserts + screws at the edges; **dowel pins** locate the seams. |
| Sealed wet/dry wall | keyed (slot) into the floor + deck and screwed — a positive water/dry barrier, not just glued. |
| Electronics (PCB, driver, PSU) | M3 heat-set standoffs on the dry-compartment floor. |
| LED bar ↔ bridge | M3 bracket screws + a **secondary-retention tether** so it can't fall (§7.2). |
| Reservoir, pump | **no fasteners** — drop-in / lift-out from the open rear service bay (§8.4). |
| Cables | routed **inside the hollow side-frame arches** (no external channel); drip loop before the dry compartment (§8.5). |
| Wood shelf | located by the recessed pot well + screwed/clipped to the base deck. |
| Feet (×4) | screw-in rubber feet on the base underside. |

The block model (`mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad`) shows
the joint **screws** as a labelled part and the **hollow posts**; precise boss/insert
geometry is added when the panels are split for printing (the next CAD step).
