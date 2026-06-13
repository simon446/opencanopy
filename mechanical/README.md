# mechanical/

Frame, bays, mounts, and printable parts for OpenCanopy. Owned by the **Mechanical** track
(spec §8, §12).

## Layout

- `cad/` — design source.
  - `source/` — native CAD source files.
  - `step/` — neutral STEP exports for interoperability.
- `stl/` — printable geometry.
  - `printable/` — release-quality, manifold STLs intended for end users.
  - `prototypes/` — work-in-progress / experimental prints.
- `drawings/` — dimensioned drawings and assembly references.
- `print-settings.md` — recommended material and slicer settings (PETG/ASA/ABS per §16.2).
- `fit-tests.md` — tolerance and fit-test results for printed parts.

## Key constraints

Open-frame appliance within the locked envelope (see `docs/product-requirements.md`): electronics in
an isolated upper dry bay, water in the bottom wet bay, removable pot and reservoir, drip tray, fan
guard, and stainless fasteners near water (spec §8, §16.2, §17).
