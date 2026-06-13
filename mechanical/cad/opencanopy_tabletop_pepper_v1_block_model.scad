// OpenCanopy Tabletop Pepper v1 — parametric block model (rounded, footed, fastened)
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// Architecture: open-frame tabletop unit — base cabinet + four rounded corner posts +
// top hood, OPEN front/sides/back. ELECTRONICS LIVE IN THE BASE, BESIDE the water
// reservoir, isolated by an additional vertical wall (wet | wall | dry).
//
// This revision adds, per review: rounded edges (furniture look), 4 feet, cable routing
// through HOLLOW posts (no dedicated channel), and a fastening scheme (M3 heat-set
// inserts + socket screws at the post joints; dowels for alignment; printed in panels
// because 480 mm exceeds consumer beds — see mechanical/fastening.md).
//
// Per-part export:  openscad -D 'part="pot"' --render -o pot.stl this_file.scad
//   parts: frame base iso_wall led_bar pot reservoir pcb driver power status feet screws
//
// Origin = front-left-bottom of envelope. X=width(0..480) Y=depth(0..320) Z=height(0..680). mm

// ----------------------------- parameters --------------------------------
env_w = 480; env_d = 320; env_h = 680;
$fn = 96;

foot_h   = 16;  foot_r = 17;  foot_inset = 36;   // 4 feet
wall_t   = 10;  deck_t = 12;                       // base shell
base_h   = 124;                                    // base top deck height
post     = 40;  post_wall = 9;  rr_post = 6;       // corner posts (rounded, hollow)
post_bore = post - 2*post_wall;                    // 22 mm cable bore through posts
top_h    = 50;  rr_hood = 6;                        // top hood ring
rr_base  = 12;                                      // base cabinet edge radius
iso_x    = 315; iso_t = 12;                         // additional isolating wall
floor_top  = foot_h + wall_t;                       // 26
deck_bot   = base_h - deck_t;                       // 112
post_bot   = base_h;                                // posts start on the base
post_top_z = env_h - top_h;                         // 630, posts meet the hood
screw_r  = 3;                                       // M3 socket-head representation

// rounded box of `size`, corner radius r (hull of 8 corner spheres)
module rbox(size, r) {
    hull() for (x = [r, size[0]-r], y = [r, size[1]-r], z = [r, size[2]-r])
        translate([x, y, z]) sphere(r = r, $fn = 40);
}

// ------------------------------- modules ----------------------------------

// 4 feet — slightly tapered pucks; the unit stands on these, not flat on the counter
module feet() {
    for (x = [foot_inset, env_w - foot_inset], y = [foot_inset, env_d - foot_inset])
        translate([x, y, 0]) cylinder(h = foot_h, r1 = foot_r, r2 = foot_r - 3);
}

// open cage: four rounded HOLLOW posts (cables run inside) + rounded top hood ring
module outer_frame() {
    ph = post_top_z - post_bot;
    for (x = [0, env_w - post], y = [0, env_d - post])
        translate([x, y, post_bot]) difference() {
            rbox([post, post, ph], rr_post);
            translate([post/2, post/2, -1]) cylinder(h = ph + 2, d = post_bore);  // cable bore
        }
    // top hood ring (rounded beams), hollow centre
    translate([0, 0, post_top_z]) {
        rbox([env_w, post, top_h], rr_hood);
        translate([0, env_d - post, 0]) rbox([env_w, post, top_h], rr_hood);
        rbox([post, env_d, top_h], rr_hood);
        translate([env_w - post, 0, 0]) rbox([post, env_d, top_h], rr_hood);
    }
}

// base cabinet: rounded shell, OPEN back + open top interior, closed deck on top.
module base_cabinet() {
    difference() {
        translate([0, 0, foot_h]) rbox([env_w, env_d, base_h - foot_h], rr_base);
        // single interior cavity (iso wall added separately); OPEN at the back (Y+)
        translate([wall_t, wall_t, floor_top])
            cube([env_w - 2*wall_t, env_d, deck_bot - floor_top]);
        // status-diffuser slot in the front wall
        translate([240 - 66, -1, 44]) cube([132, wall_t + 2, 20]);
        // deck drain hole over the WET side -> reservoir
        translate([200, 150, deck_bot - 1]) cylinder(h = deck_t + 2, d = 26);
    }
}

// the ADDITIONAL isolating wall between wet (left) and dry (right) compartments
module iso_wall() {
    translate([iso_x - iso_t/2, wall_t, floor_top])
        cube([iso_t, env_d - wall_t, deck_bot - floor_top]);
}

// fixed full-spectrum LED grow bar (downlight) under the hood
module top_light_module() {
    led_l = 380; led_w = 90; led_h = 22;
    translate([240 - led_l/2, 160 - led_w/2, post_top_z - led_h - 6]) rbox([led_l, led_w, led_h], 4);
}

// removable ~10 L pot — hollow (open top, walls, drain hole), on the deck
module pot_placeholder() {
    top_d = 280; bot_d = 235; pot_h = 230; pwall = 9; pfloor = 14;
    translate([240, 150, base_h]) difference() {
        cylinder(h = pot_h, d1 = bot_d, d2 = top_d);
        translate([0, 0, pfloor]) cylinder(h = pot_h, d1 = bot_d - 2*pwall, d2 = top_d - 2*pwall);
        translate([0, 0, -1]) cylinder(h = pfloor + 2, d = 22);
    }
}

// reservoir — WET compartment (left of the isolating wall)
module reservoir_placeholder() {
    rw = iso_x - iso_t/2 - wall_t - 16; rd = 150; rh = 80;
    translate([wall_t + 8, 150, floor_top + 4]) rbox([rw, rd, rh], 6);
}

// electronics — DRY compartment (right of the isolating wall), clear of the walls
module pcb()         { translate([iso_x + iso_t/2 + 14, 96, floor_top + 12]) cube([90, 120, 8]); }
module led_driver()  { translate([iso_x + iso_t/2 + 14, 226, floor_top + 6]) rbox([100, 60, 34], 3); }
module power_input() { translate([iso_x + iso_t/2 + 14, 26, floor_top + 6]) rbox([90, 56, 30], 3); }
module electronics_bay() { pcb(); led_driver(); power_input(); }

// front LED status diffuser (4 LEDs behind a frosted strip; no screen/controls)
module status_led_diffuser() {
    dw = 120; dh = 16; dd = 6;
    translate([240 - dw/2, 0.4, 54 - dh/2]) rbox([dw, dd, dh], 2);
}

// fastening: M3 socket-head screws at the 8 post joints (post<->base, post<->hood).
// Heat-set brass inserts in the mating bosses; dowel pins align panels. See fastening.md.
module screws() {
    module cap() { cylinder(h = 4, d = 2*screw_r); translate([0,0,-8]) cylinder(h = 8, d = 3.4); }
    for (x = [post/2, env_w - post/2], y = [post/2, env_d - post/2]) {
        translate([x, y, base_h + 2]) cap();                 // post -> base (screws up)
        translate([x, y, post_top_z - 2]) rotate([180,0,0]) cap(); // post -> hood (screws down)
    }
}

// ------------------------------- assembly ---------------------------------
module assembly() {
    color([0.62,0.62,0.64]) outer_frame();
    color([0.90,0.89,0.85]) base_cabinet();
    color([0.55,0.65,0.80]) iso_wall();
    color([0.95,0.88,0.35]) top_light_module();
    color([0.80,0.50,0.34]) pot_placeholder();
    color([0.25,0.60,0.88]) reservoir_placeholder();
    color([0.15,0.55,0.30]) pcb();
    color([0.28,0.28,0.33]) led_driver();
    color([0.55,0.55,0.60]) power_input();
    color([0.30,0.82,0.55]) status_led_diffuser();
    color([0.50,0.50,0.52]) feet();
    color([0.20,0.20,0.22]) screws();
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
else if (part == "feet")       feet();
else if (part == "screws")     screws();
