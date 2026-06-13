// OpenCanopy Tabletop Pepper v1 — v0 parametric BLOCK MODEL
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// Built from docs/cad_brief_for_claude.md, with the CURRENT (latest) architecture
// decision overriding the brief's electronics placement:
//
//   ELECTRONICS LIVE IN THE BASE, *BESIDE* THE WATER RESERVOIR, ISOLATED BY AN
//   ADDITIONAL VERTICAL WALL (side-by-side wet | wall | dry, both in the base).
//
// This keeps water and electronics horizontally separated by a solid wall rather than
// stacked, so a leak/overflow cannot reach the dry compartment. Simple solids only;
// no screws, wiring, fillets, labels, or plant geometry in v0. The base cabinet is
// drawn translucent so the two compartments + the isolating wall are visible.
//
// Coordinate system: origin = front-left-bottom of the envelope.
//   X = width (0..480), Y = depth front->back (0..320), Z = height (0..680). mm.

// ----------------------------- parameters --------------------------------
env_w = 480;
env_d = 320;
env_h = 680;

$fn = 64;

side_t     = 40;                 // corner-post / side beam thickness
top_beam_h = 50;                 // top ring beam height
base_h     = 100;                // base cabinet height
wall_t     = 10;                 // base outer wall thickness
iso_x      = 320;                // X of the additional isolating wall (wet | dry)
iso_t      = 12;                 // isolating wall thickness
clear_w    = env_w - 2 * side_t;

// preview colours ([r,g,b] opaque; [r,g,b,a] translucent in preview)
col_frame = [0.94, 0.94, 0.92];
col_base  = [0.94, 0.94, 0.92, 0.16];   // translucent cabinet (reveals compartments)
col_wood  = [0.80, 0.62, 0.38];
col_pot   = [0.74, 0.52, 0.36];
col_water = [0.40, 0.70, 0.85];
col_pcb   = [0.20, 0.45, 0.25];
col_drv   = [0.30, 0.30, 0.32];
col_pwr   = [0.45, 0.45, 0.48];
col_led   = [0.96, 0.93, 0.70];
col_fan   = [0.55, 0.55, 0.58];
col_diff  = [0.50, 0.85, 0.60];
col_cable = [0.85, 0.75, 0.25];

// ------------------------------- modules ----------------------------------

// 1. open structural cage — four corner posts + top perimeter ring (open front+sides)
module outer_frame() {
    post = side_t;
    post_top = env_h - top_beam_h;
    color(col_frame) {
        for (x = [0, env_w - post], y = [0, env_d - post])
            translate([x, y, base_h]) cube([post, post, post_top - base_h]);
        translate([0, 0, post_top]) {
            cube([env_w, post, top_beam_h]);
            translate([0, env_d - post, 0]) cube([env_w, post, top_beam_h]);
            cube([post, env_d, top_beam_h]);
            translate([env_w - post, 0, 0]) cube([post, env_d, top_beam_h]);
        }
        // thin rear support panel (back only; front + sides stay open)
        translate([post, env_d - 16, base_h]) cube([clear_w, 16, post_top - base_h]);
    }
}

// 2. fixed horticultural LED bar (downlight) under the top ring
module top_light_module() {
    led_l = 390; led_w = 50; led_h = 20; cz = 620;
    color(col_led)
        translate([240 - led_l / 2, 152 - led_w / 2, cz - led_h / 2]) cube([led_l, led_w, led_h]);
    color(col_wood)
        translate([60, 0, env_h - top_beam_h]) cube([env_w - 120, 5, top_beam_h - 6]);
}

// 3. base cabinet with TWO isolated compartments (wet | iso wall | dry)
module bottom_base() {
    // base floor plate + the ADDITIONAL isolating wall — always shown
    color([0.88, 0.88, 0.86]) cube([env_w, env_d, 8]);
    color(col_frame)
        translate([iso_x - iso_t / 2, wall_t, 0]) cube([iso_t, env_d - 2 * wall_t, base_h]);
    if (show_top) {
        // translucent cabinet body + front wood accent (exterior look)
        color(col_base) translate([0, 0, 8]) cube([env_w, env_d, base_h - 8]);
        color(col_wood) translate([60, 0, 18]) cube([env_w - 120, 5, base_h - 36]);
    }
}

// 4. removable 8-10 L pot (tapered), on the base deck
module pot_placeholder() {
    top_d = 280; bot_d = 235; pot_h = 235;
    color(col_pot) translate([240, 150, base_h]) cylinder(h = pot_h, d1 = bot_d, d2 = top_d);
}

// 5. reservoir — WET compartment (left of the isolating wall)
module reservoir_placeholder() {
    rw = iso_x - iso_t / 2 - wall_t - 12;   // fit the wet compartment width
    rd = 150; rh = 80;
    color(col_water) translate([wall_t + 6, 150, 14]) cube([rw, rd, rh]);  // ~3.4 L
}

// 6. electronics — DRY compartment (right of the isolating wall, BESIDE the water)
module electronics_bay() {
    ex = iso_x + iso_t / 2 + 6;             // just right of the isolating wall
    // PCB lies flat; LED driver + power supply beside it (placeholders)
    color(col_pcb) translate([ex, 90, 24])  cube([90, 120, 8]);    // controller PCB
    color(col_drv) translate([ex, 222, 16]) cube([100, 60, 32]);   // LED driver
    color(col_pwr) translate([ex, 24, 16])  cube([90, 56, 30]);    // power input
}

// 7. quiet fan, upper rear-right (square plate + circular bore)
module fan_mount() {
    fs = 60; ft = 15; bore = 50; cx = 390; cy = 300; cz = 500;
    color(col_fan)
        translate([cx, cy, cz]) rotate([90, 0, 0])
            difference() {
                translate([-fs / 2, -fs / 2, -ft / 2]) cube([fs, fs, ft]);
                cylinder(h = ft + 2, d = bore, center = true);
            }
}

// 8. front LED status diffuser (4 LEDs behind a frosted strip; no screen/controls)
module status_led_diffuser() {
    dw = 120; dh = 16; dd = 6; cx = 240; cz = 55;
    color(col_diff) translate([cx - dw / 2, 0, cz - dh / 2]) cube([dw, dd, dh]);
    for (i = [0:3])
        color([0.9, 0.9, 0.9])
            translate([cx - dw / 2 + 18 + i * (dw - 36) / 3, dd, cz])
                rotate([90, 0, 0]) cylinder(h = 2, d = 8, center = true);
}

// 9. rear-right cable channel (base dry compartment -> LED, low-voltage; drip loops)
module cable_channel() {
    cw = 30; cd = 30;
    x = env_w - side_t - cw;
    y = env_d - cd - 2;
    color(col_cable) translate([x, y, base_h]) cube([cw, cd, 600 - base_h]);
}

// render-only flag: set false (e.g. `openscad -D show_top=false`) to hide the upper
// parts and reveal the base internals (reservoir | isolating wall | electronics).
show_top = true;

// 10. full assembly
module assembly() {
    bottom_base();
    reservoir_placeholder();
    electronics_bay();
    status_led_diffuser();
    if (show_top) {
        outer_frame();
        top_light_module();
        pot_placeholder();
        fan_mount();
        cable_channel();
    }
}

assembly();
