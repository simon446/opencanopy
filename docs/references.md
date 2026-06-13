# OpenCanopy V1 — References & Research Basis

**Owner:** Documentation track (WI-DOC-07) + Plant Science · **Spec refs:** §2.1, §2.2, §2.3 · **Status:** Seeded

This page surfaces the research basis behind OpenCanopy's design decisions, transcribed from the
engineering spec §2 (now in-repo at `plan/tabletop_pepper_grower_v1_spec_v1_1.md`). It is the source
list the requirements, scope, and risk docs cite. The Documentation and Plant Science tracks own
expanding it (e.g. adding sources as the plant profile, light, and nutrient work items land).

> The design values are grounded in extension/research guidance, not marketing claims — in particular,
> light is specified in **DLI / PPFD**, never lumens.

## Core plant facts the design relies on (§2.1)

- Peppers are **warm-season** crops; fruit set drops at temperature extremes (roughly >90 °F / 32 °C
  days, or nights too cold/warm). [R4, R5]
- **Consistent moisture** aids flower/fruit retention and reduces blossom-end rot; **waterlogging
  harms roots**. [R5, R8, R15, R16]
- **Container plants dry quickly** and need frequent but not excessive watering. [R7, R8]
- Best around **media pH 6.0–6.8**. [R4, R10]
- For grow lighting, **DLI and PPFD** are the engineering quantities, not lumens. [R1, R2, R3]
- Treat fruiting peppers as **high-light**: target **DLI ≈ 20–25 mol·m⁻²·day⁻¹** and the ability to
  deliver **≥ ~400 µmol·m⁻²·s⁻¹ PPFD** across the useful canopy, heat permitting. [R3, R14]
- Prefer a **broad-spectrum white** horticultural LED (adequate blue + red) over a "blurple-only"
  fixture for a living-space product. [R2, R14]

## Sources (§2.2)

| ID | Source | Used for |
|---|---|---|
| R1 | Virginia Tech Extension — [Calculating and Using Daily Light Integral (DLI)](https://pubs.ext.vt.edu/SPES/spes-720/spes-720.html) | DLI concept; PPFD/DLI relationship; 400–700 nm PAR framing |
| R2 | University of Missouri Extension — [CEA: Understanding Grow Lights](https://extension.missouri.edu/publications/g6987) | CEA grow-light selection; LED considerations; system-level light planning |
| R3 | Oklahoma State University Extension — [LED Grow Lights for Plant Production](https://extension.okstate.edu/fact-sheets/led-grow-lights-for-plant-production) | PPFD and DLI ranges; high-light crop framing |
| R4 | Oklahoma State University Extension — [Pepper Production](https://extension.okstate.edu/fact-sheets/pepper-production) | Pepper temperature; fruit-set constraints; soil pH |
| R5 | Michigan State University Extension — [How to Grow Peppers, Part 2](https://www.canr.msu.edu/resources/how_to_grow_peppers_part_2) | Day/night temperatures; moisture; blossom drop |
| R6 | University of Minnesota Extension — [Growing peppers](https://extension.umn.edu/vegetables/growing-peppers) | Seed starting; germination warmth; watering practice |
| R7 | University of Minnesota Extension — [Fertilizing and watering container plants](https://extension.umn.edu/managing-soil-and-nutrients/fertilizing-and-watering-container-plants) | Container moisture behavior; fertilizer caution |
| R8 | Illinois Extension — [Watering \| Container Gardens](https://extension.illinois.edu/container-gardens/watering) | Container watering; avoiding waterlogging and full dry-out |
| R9 | Oregon State University Extension — [Grow your own peppers](https://extension.oregonstate.edu/catalog/ec-1227-grow-your-own-peppers) | Container size; drainage; container pepper guidance |
| R10 | Ohio State University Ohioline — [Growing Peppers in the Home Garden](https://ohioline.osu.edu/factsheet/hyg-1618) | pH; drip/soaker watering; wet-leaf disease avoidance |
| R11 | Oklahoma State University Extension — [EC and pH Guide for Hydroponics](https://extension.okstate.edu/fact-sheets/electrical-conductivity-and-ph-guide-for-hydroponics) | Hydroponic pH/EC reference values for peppers |
| R12 | University of Missouri Extension — [Hydroponic Nutrient Solutions](https://extension.missouri.edu/publications/g6984) | Nutrient solution management; pH/EC monitoring limits |
| R13 | Michigan State University Extension — [Why pay attention to VPD and not RH?](https://www.canr.msu.edu/news/why_should_greenhouse_growers_pay_attention_to_vapor_pressure_deficit_and_n) | VPD logic; humidity disease/transpiration framing |
| R14 | Liu et al. — [Effects of LED Light Quality on Pepper Seedlings, *Agronomy* 2022](https://www.mdpi.com/2073-4395/12/10/2269) | Seedling response to blue/UV-A/far-red light quality |
| R15 | Michigan State University Extension — [Blossom end rot: a perennial problem](https://www.canr.msu.edu/news/blossom_end_rot_understanding_a_perennial_problem) | Blossom-end rot, calcium transport, inconsistent water |
| R16 | Ohio State University — [Blossom End Rot of Tomatoes and Peppers](https://u.osu.edu/gofarmohio/2022/07/15/blossom-end-rot-of-tomatoes-and-peppers/) | Blossom-end rot from dry/wet soil swings |
| R17 | PuckerButt Pepper Company — [Carolina Reaper growing guide](https://puckerbuttpeppercompany.com/blogs/insights/the-complete-guide-to-growing-your-very-first-carolina-reaper-pepper-plant) | Carolina Reaper seed-starting and temperature context |

## Important limitation (§2.3)

This is a **small indoor device plan, not a commercial greenhouse crop plan**:

- The DLI target is ambitious and is only approachable in a tabletop footprint if the **canopy area is
  constrained**, the LED has a **real PPFD map**, and the plant is **pruned/trained**.
- The compact geometry trades some maximum yield for table fit and visual acceptability — acceptable
  for V1, whose goal is **credible indoor fruiting, not commercial yield**.
- The device is **not enclosed** and does not heat/cool the room; it manages LED heat, airflow,
  watering, and warning states only. A 22–25 °C room is adequate (germination may benefit from a
  removable heat mat).

---

*Full context and additional design rationale live in the engineering spec §2 and §5 (`plan/`). The
data the BOM/light must satisfy is in spec §16.3; see also [`product-requirements.md`](product-requirements.md),
[`scope.md`](scope.md), and [`risk-register.md`](risk-register.md).*
