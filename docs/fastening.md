# Fastening & assembly strategy

How the parts join, and why — for the two-pillar unit (**base + two wooden pillars + top
LED block + removable grow insert**). See [ECO-003](ECO-003-v1-redesign.md) for the redesign.

## Why it can't be a single print

The unit is **480 × 320 × 680 mm**. Consumer FDM beds are ~220–350 mm, so a one-piece print
is impossible. The unit is built from a few modular parts that bolt together: the **base** (may
itself be split for the bed, or printed on a 350 mm+ bed), **two pillars**, and the **top LED
block** — which **prints in two parts (body + bottom lid)** so the encapsulated controller PCB
can be installed. This is the standard approach for large printed assemblies and keeps the unit
**serviceable**.

## Primary method — heat-set inserts + machine screws + dowels

The de-facto standard for quality printed assemblies (Voron, Prusa enclosures), and what V1 uses:

- **Brass heat-set threaded inserts** + **machine screws** through clearance holes on the mating
  part: strong, repeatable, **re-serviceable** (unlike glue or one-shot snap-fits).
- **Dowel pins** (Ø4 mm) for alignment and to **carry shear**, so screws aren't the only locator.
- **Stainless steel** screws anywhere near water (§8.3 / §16.2).

## Per-joint scheme (two-pillar architecture)

| Joint | Fastening |
|---|---|
| **Pillar ↔ base** (×2) | each pillar **seats 30 mm into a base socket** in a dry structural boss + **1 hidden M4** from the **underside** into a threaded insert in the pillar bottom + **1 anti-rotation dowel** (Ø4). The base underside is **counterbored** for straight-screwdriver access. |
| **Pillar ↔ top block** (×2) | each pillar top **seats 30 mm into a socket** in the block + a **rear set screw** grips the pillar (the set-screw access is on the block rear face). |
| **Top block body ↔ bottom lid** | the block prints in two parts to **encapsulate the PCB**; joined with **M3 heat-set inserts + screws** along the underside seam. |
| **Controller PCB ↔ block** | the **1.6 mm board** mounts on **4 standoff bosses** inside the block bay with **M2.5 screws**; the **USB-C** port exits through the block **rear face**. The board is fully enclosed (not exposed). |
| **LED bar ↔ block** | **M4 screws** up into block inserts (heads below the bar); a **secondary-retention tether** so the bar can't fall (§7.2). |
| **Grow insert** | **drop-in / lift-out, no fasteners** — lifts straight out of the well for cleaning and root inspection (§8.4). |
| **Reservoir** | **integral to the base** (the base *is* the wet zone). Service is **fill from the top port** + **insert lift-out** to access/clean the reservoir; no fasteners. |
| **Feet (×4)** | screw-in rubber feet on the base underside. |
| **Cabling** | low-voltage **sensor leads run up the rear flat of the right pillar** through a **sealed grommet** into the base; **USB-C power enters at the top block**. Drip/strain relief at the base entry. No cable crosses the open water. |

## Materials note (pillars)

The pillars are the visible structural + decorative element. Options (ECO-003 Open Q5): real
**wood dowels** with a threaded insert or cross-pinned foot; **printed faux-wood** with heat-set
inserts; or a wood/composite. The socket + M4 + dowel joint above works for any of these — only
the insert install method differs (drilled+epoxied insert in real wood vs heat-set in printed).

## Alternatives (and when)

| Method | Use | Notes |
|---|---|---|
| Heat-set insert + screw | **all structural + serviceable joints** | primary |
| Captive hex nut + screw | where a heat-set tool isn't available | print a nut pocket |
| Dowel pin | alignment + shear at seams | not load-bearing alone |
| Set screw (grub) | pillar ↔ block grip | accessible from the block rear |
| CA / epoxy | wood-insert install; permanent decorative seams only | **not** for serviceable structural joints |
| Snap-fit | tool-free covers | wears with cycles; not for primary structure |

The block model (`mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad`) shows the
**dowels** and **screws** as labelled parts and the pillar sockets/bosses; precise boss/insert
geometry and the block body/lid split are finalised when parts are prepared for printing.
