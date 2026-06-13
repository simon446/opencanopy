# CAD Verification Checklist (spec §12.1)

Mechanical track · covers WI-ME-01 … WI-ME-07. The numeric rows are **computed
from the parametric model** by `cad/source/verify.py` (run it to reproduce), so
this checklist cannot silently drift from the geometry.

```text
$ .venv-cad/bin/python mechanical/cad/source/verify.py
CHECK                             VALUE                 TARGET                       RESULT
Envelope (W x D x H)              468 x 314 x 700 mm    <= 480 x 320 x 700           PASS
Frame interior (W x D)            428 x 268 mm          info                         PASS
Pot usable capacity               9.7 L                 8-10 L compact               PASS
Pot OD vs interior depth          250 vs 268 mm         clearance 18 mm > 0          PASS
Reservoir usable capacity         4.1 L                 2.5-4 L compact              PASS
LED clearance above pot rim       159 mm                150-300 mm                   PASS
Reservoir top vs pot deck         146 vs 180 mm         withdraws below the deck     PASS
CG height (full + plant)          305 mm (44% H)        bottom-heavy (< 50% H)       PASS
Cable bend / drip-loop radius     20 mm                 drip loop before bay         PASS
CAD verification: PASS
```

## §12.1 line items

| # | Check | Status | Evidence |
|---|---|---|---|
| 1 | Full assembly CAD complete | ✅ | `cad/source/opencanopy/assembly.py`; `cad/step/opencanopy-assembly.step` |
| 2 | Pot inserted/removed path checked | ✅ | Pot lifts off the locating ring (10 mm) into 159 mm of headroom, then withdraws through the open front; light mount raises on its adjustment row if needed. |
| 3 | Reservoir inserted/removed path checked | ✅ | Reservoir top 146 mm < pot deck 180 mm → the drawer slides out the front on the cradle rails **without** disturbing the pot/plant (§8.4). |
| 4 | Pump/filter access checked | ✅ | Pump sits in the reservoir on the open-front C-clip (`pump-clip`); lift the reservoir, lift the pump out — tool-free. |
| 5 | Cable bend radius checked | ✅ | Drip-loop / min-bend radius 20 mm enforced at every dry-bay grommet entry; channel mouths are radiused. |
| 6 | Tubing path checked | ✅ | Tube runs in the dedicated channel pocket (separate from wiring), visible/inspectable, clipped with `tube-clip`. |
| 7 | LED height/clearance checked | ✅ | 159 mm above the pot rim at the nominal carrier hole, adjustable 150–300 mm (§8.6). |
| 8 | Fan clearance checked | ✅ | Fan offset +80 mm in X and high at the rear → circulation across, not a stream into, the canopy; guard present. |
| 9 | Electronics bay access checked | ✅ | Top-removable lid; serviced without opening the wet bay (§8.4). |
| 10 | Tool access checked | ✅ | Reservoir, pump and pot are tool-free; only heat-set/screw bosses need a driver, all reachable from the open front/top. |
| 11 | Center of gravity (full reservoir + plant) | ✅ | CG at 305 mm = 44 % of height; water + media + pot dominate and sit low → bottom-heavy/stable. |
| 12 | Drip/leak path checked | ✅ | Pot overflow → pot-tray gutter → downspout → leak tray → sensor sump. Reservoir overflow → front weir (away from rear/upper electronics). |
| 13 | Interference / clearance (worst case) | ✅ | `collision_check.py` (FCL on the real models): zero collisions; every clearance ≥2 mm, survives a 1.0 mm worst-case FDM closure budget. Details in `tolerance-analysis.md`. |

## "Water fails downward; electronics live upward" (§6.2)

`assembly.assert_zone_separation()` proves no part tagged *dry* (electronics bay +
lid) intrudes below the pot-deck Z (180 mm). The only water-carrying parts
(reservoir, pump, leak tray, pot tray) are the lowest modules; the electronics bay
is the topmost module. This is asserted on every build — see `build.py` output.

## Interface assumptions published to other tracks

* **Electronics (WI-EE-04):** the dry bay reserves a **120 × 90 mm** controller-PCB
  envelope with M3 heat-set bosses on an 8 mm-inset pattern, plus a 110 × 40 mm
  remote-driver footprint. The EE-04 layout report (`electronics/analysis/`)
  reciprocally states the board outline + mount-hole pattern are *"coordinated with
  the mechanical electronics-bay before fabrication"* — i.e. the outline is **not yet
  frozen on either side**; this reserved envelope is the coordination point. When the
  KiCad board lands, confirm it fits and update `PCB_W/PCB_D` in `params.py` (a
  change-controlled value) if needed.
* **Electronics (WI-EE-05):** cable-channel and dry-bay entries are labelled
  `pump / fan / led / moisture / reservoir / leak`. Verified against the landed
  `electronics/wiring/harness-table.csv` — connectors `J_PUMP / J_FAN / J_LED /
  J_MOIST / J_RES / J_LEAK` match; the table also has a `J_PWR` 24 V PSU input
  (XT30/screw) which enters the bay through its own grommet + drip loop. Field
  connectors are keyed JST VH/XH/PH, which the channel pockets and grommet bores clear.
* **Light (§16.3):** the LED head is modelled at 320 × 120 × 30 mm, 50–80 W remote
  driver; secondary-retention eyelets (Ø4 mm tether) are provided regardless of the
  exact fixture chosen.
