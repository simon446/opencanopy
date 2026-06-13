# Fastening & assembly strategy

How the parts join, and why. This addresses the "how is it held together?" question for
the open-frame unit (base cabinet + corner posts + hood + internal panels).

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
| Post ↔ base deck (×4) | 2× M3 heat-set inserts in each post foot; screws up through the deck. Modelled as the `screws` part. |
| Post ↔ hood (×4) | 2× M3 inserts in each post top; screws down from the hood. |
| Base cabinet panels (front/sides/floor/deck) | M3 inserts + screws at the edges; **dowel pins** locate the seams. |
| Isolating wall | keyed (slot) into the floor + deck and screwed — a positive water/dry barrier, not just glued. |
| Electronics (PCB, driver, PSU) | M3 heat-set standoffs on the dry-shelf floor. |
| LED bar ↔ hood | M3 bracket screws + a **secondary-retention tether** so it can't fall (§7.2). |
| Reservoir, pump | **no fasteners** — drop-in / lift-out from the open back for tool-free service (§8.4). |
| Cables | routed **through the hollow posts** (no external channel); drip loop before the dry compartment (§8.5). |
| Feet (×4) | M3 insert or screw-in rubber feet on the base underside. |

The block model (`mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad`) shows
the joint **screws** as a labelled part and the **hollow posts**; precise boss/insert
geometry is added when the panels are split for printing (the next CAD step).
