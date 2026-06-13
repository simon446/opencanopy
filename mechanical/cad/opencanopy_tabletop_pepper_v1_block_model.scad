// OpenCanopy Tabletop Pepper v1 — v0 parametric BLOCK MODEL
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// Source of truth for the v0 geometry (docs/cad_brief_for_claude.md). Architecture:
// open-frame tabletop unit — base cabinet + four corner posts + top hood, OPEN at the
// front, sides AND back. ELECTRONICS LIVE IN THE BASE, BESIDE the water reservoir,
// isolated by an additional vertical wall (side-by-side: wet | wall | dry).
//
// Per-part export for rendering / collision checking: set the `part` variable, e.g.
//   openscad -D 'part="pot"' --render -o pot.stl this_file.scad
// `part="all"` (default) renders the whole assembly.
//
// Coordinate system: origin = front-left-bottom of the envelope.
//   X = width (0..480), Y = depth front->back (0..320), Z = height (0..680). mm.

// ----------------------------- parameters --------------------------------
env_w = 480;
env_d = 320;
env_h = 680;

$fn = 120;                 // smooth cylinders

post     = 40;             // corner-post cross-section
top_h    = 50;            // top hood ring height
base_h   = 110;           // base cabinet height
wall_t   = 10;            // base wall thickness
deck_t   = 10;            // base top deck thickness
iso_x    = 315;           // X of the additional isolating wall (wet | dry)
iso_t    = 12;            // isolating wall thickness
eps      = 0.4;           // small inset to avoid coincident-face z-fighting

post_top = env_h - top_h; // posts run from base top to the hood ring

// ------------------------------- modules ----------------------------------

// open structural cage: four corner posts + top perimeter ring (open front/sides/back)
module outer_frame() {
    for (x = [0, env_w - post], y = [0, env_d - post])
        translate([x, y, base_h]) cube([post, post, post_top - base_h]);
    translate([0, 0, post_top]) {                 // top ring (hollow centre)
        cube([env_w, post, top_h]);
        translate([0, env_d - post, 0]) cube([env_w, post, top_h]);
        cube([post, env_d, top_h]);
        translate([env_w - post, 0, 0]) cube([post, env_d, top_h]);
    }
}

// fixed full-spectrum LED bar (downlight) slung under the top ring
module top_light_module() {
    led_l = 380; led_w = 90; led_h = 22;
    translate([240 - led_l/2, 160 - led_w/2, post_top - led_h - 6])
        cube([led_l, led_w, led_h]);
}

// base cabinet: floor + front + two side walls + top deck. OPEN BACK (rear service)
// and split into wet (left) / dry (right) by the isolating wall (separate module).
module base_cabinet() {
    // floor
    cube([env_w, env_d, wall_t]);
    // front wall (with a status-diffuser cutout) and two full-depth side walls
    difference() {
        union() {
            translate([0, 0, wall_t]) cube([env_w, wall_t, base_h - wall_t - deck_t]);     // front
            translate([0, 0, wall_t]) cube([wall_t, env_d, base_h - wall_t - deck_t]);      // left
            translate([env_w - wall_t, 0, wall_t]) cube([wall_t, env_d, base_h - wall_t - deck_t]); // right
        }
        translate([240 - 66, -1, 38]) cube([132, wall_t + 2, 20]);                          // status slot
    }
    // top deck (closes the top so the pot has a seat) with a drain hole over the WET side
    difference() {
        translate([0, 0, base_h - deck_t]) cube([env_w, env_d, deck_t]);
        translate([200, 150, base_h - deck_t - 1]) cylinder(h = deck_t + 2, d = 26);         // drain -> reservoir
    }
}

// the ADDITIONAL isolating wall between the wet (left) and dry (right) compartments
module iso_wall() {
    translate([iso_x - iso_t/2, wall_t + eps, wall_t + eps])
        cube([iso_t, env_d - 2*(wall_t + eps), base_h - wall_t - deck_t - 2*eps]);
}

// removable ~10 L pot — HOLLOW (open top, walls, central drain hole), on the deck
module pot_placeholder() {
    top_d = 280; bot_d = 235; pot_h = 230; pwall = 9; pfloor = 14;
    translate([240, 150, base_h]) difference() {
        cylinder(h = pot_h, d1 = bot_d, d2 = top_d);
        translate([0, 0, pfloor])                                   // hollow interior
            cylinder(h = pot_h, d1 = bot_d - 2*pwall, d2 = top_d - 2*pwall);
        translate([0, 0, -1]) cylinder(h = pfloor + 2, d = 22);     // drain hole
    }
}

// reservoir — WET compartment (left of the isolating wall)
module reservoir_placeholder() {
    rw = iso_x - iso_t/2 - wall_t - 16; rd = 150; rh = 80;
    translate([wall_t + 8, 150, wall_t + 4]) cube([rw, rd, rh]);     // ~3.3 L
}

// electronics — DRY compartment (right of the isolating wall), well clear of the walls
module pcb()          { translate([iso_x + iso_t/2 + 14, 96, wall_t + 12]) cube([90, 120, 8]); }
module led_driver()   { translate([iso_x + iso_t/2 + 14, 226, wall_t + 6]) cube([100, 60, 34]); }
module power_input()  { translate([iso_x + iso_t/2 + 14, 26, wall_t + 6]) cube([90, 56, 30]); }
module electronics_bay() { pcb(); led_driver(); power_input(); }

// front LED status diffuser (4 LEDs behind a frosted strip; no screen/controls)
module status_led_diffuser() {
    dw = 120; dh = 16; dd = 6;
    translate([240 - dw/2, eps, 48 - dh/2]) cube([dw, dd, dh]);
}

// rear-right cable channel (base dry compartment -> LED), routed on the rear-right post
module cable_channel() {
    cw = 26; cd = 26;
    translate([env_w - post - cw, env_d - post - cd, base_h]) cube([cw, cd, post_top - base_h]);
}

// ------------------------------- assembly ---------------------------------
module assembly() {
    color([0.93,0.93,0.91]) outer_frame();
    color([0.93,0.93,0.91]) base_cabinet();
    color([0.85,0.85,0.88]) iso_wall();
    color([0.96,0.93,0.55]) top_light_module();
    color([0.78,0.50,0.34]) pot_placeholder();
    color([0.30,0.62,0.85]) reservoir_placeholder();
    color([0.15,0.45,0.25]) pcb();
    color([0.28,0.28,0.30]) led_driver();
    color([0.45,0.45,0.48]) power_input();
    color([0.35,0.80,0.55]) status_led_diffuser();
    color([0.85,0.72,0.25]) cable_channel();
}

// ------------------------- per-part dispatch ------------------------------
part = "all";
if      (part == "all")        assembly();
else if (part == "frame")      outer_frame();
else if (part == "base")       base_cabinet();
else if (part == "iso_wall")   iso_wall();
else if (part == "led_bar")    top_light_module();
else if (part == "pot")        pot_placeholder();
else if (part == "reservoir")  reservoir_placeholder();
else if (part == "pcb")        pcb();
else if (part == "driver")     led_driver();
else if (part == "power")      power_input();
else if (part == "status")     status_led_diffuser();
else if (part == "cable")      cable_channel();
