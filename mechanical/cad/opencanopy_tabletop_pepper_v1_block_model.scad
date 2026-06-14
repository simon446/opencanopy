// OpenCanopy Tabletop Pepper — v1 AESTHETIC PRODUCT MODEL
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// Compact Scandinavian tabletop appliance. SAME functional architecture as v0:
//   - electronics + reservoir in the BASE; wet | sealed wall | dry (side by side)
//   - open-frame / non-enclosed; no screen, no controls; 4 status LEDs only
// New product form (silhouette-first; screws/wiring deferred):
//   - two continuous rounded SIDE-FRAME ARCHES (left + right) replace the 4 posts
//   - slim TOP LIGHT BRIDGE (not a thick top cage)
//   - slimmer footed BASE, rounded front corners, recessed rear service bay
//   - raised WOOD-LOOK SHELF with a recessed circular POT WELL (integrated pot)
//   - small PILL status diffuser (4 tiny LED dots) instead of a big front panel
// Material grouping: white shell / wood accent / dark recessed service / (reservoir
// translucent in cutaway only).
//
// Per-part export:  openscad -D 'part="pot"' --render -o pot.stl this_file.scad
// Origin = front-left-bottom. X=width(0..480) Y=depth(0..320) Z=height(0..680). mm

// ----------------------------- parameters --------------------------------
env_w = 480; env_d = 320; env_h = 680;
$fn = 96;

base_h    = 120;           // visible base height (100-120)
foot_h    = 14;  foot_r = 16;  foot_inset = 40;
wall_t    = 10;
side_t    = 26;            // side-frame visual thickness in X (22-28)
fw        = 26;            // arch member width (Y/Z)
arch_rr   = 26;            // arch outer corner radius (16-28)
rr_base   = 20;            // base edge radius
bridge_h  = 52;            // top light bridge height (45-60)
bridge_y0 = 44; bridge_d = 80;   // bridge front position + depth
iso_x     = 315; iso_t = 12;     // sealed wet|dry wall

shelf_w = 360; shelf_d = 250; shelf_h = 30; well_d = 244; well_depth = 22;
floor_top = foot_h + wall_t;
deck_bot  = base_h - 12;

// rounded box (hull of 8 corner spheres)
module rbox(size, r) {
    r = min(r, size[0]/2, size[1]/2, size[2]/2);
    hull() for (x=[r,size[0]-r], y=[r,size[1]-r], z=[r,size[2]-r]) translate([x,y,z]) sphere(r=r,$fn=40);
}
// horizontal stadium/pill solid of length L (X), width w (Y), thickness t (Z)
module pill(L, w, t) {
    hull() for (x=[w/2, L-w/2]) translate([x, w/2, 0]) cylinder(h=t, d=w);
}

// ------------------------------- modules ----------------------------------

// 4 feet
module feet() {
    for (x=[foot_inset, env_w-foot_inset], y=[foot_inset, env_d-foot_inset])
        translate([x,y,0]) cylinder(h=foot_h, r1=foot_r, r2=foot_r-3);
}

// one inverted-U side arch (legs front+back, rounded top), thin in X
module side_arch(x0) {
    ysp = env_d - 12; hsp = env_h - base_h;
    difference() {
        translate([x0, 6, base_h]) rbox([side_t, ysp, hsp], arch_rr);
        translate([x0-1, 6+fw, base_h+fw]) rbox([side_t+2, ysp-2*fw, hsp-2*fw], max(3, arch_rr-10)); // opening
        translate([x0-1, 6+fw, base_h-1]) cube([side_t+2, ysp-2*fw, fw+2]);                            // open bottom -> arch
    }
}
module side_frames() { side_arch(0); side_arch(env_w - side_t); }

// slim top light bridge connecting the two arches at the front-top
module light_bridge() {
    translate([side_t-6, bridge_y0, env_h - bridge_h]) rbox([env_w - 2*side_t + 12, bridge_d, bridge_h], 12);
}

// LED grow bar under the bridge (downlight)
module led_bar() {
    led_l = 360; led_w = 70; led_h = 20;
    translate([240 - led_l/2, bridge_y0 + bridge_d/2 - led_w/2, env_h - bridge_h - led_h]) rbox([led_l, led_w, led_h], 4);
}

// base shell: slim, rounded, OPEN back + recessed rear service bay, closed deck
module base_shell() {
    difference() {
        translate([0,0,foot_h]) rbox([env_w, env_d, base_h-foot_h], rr_base);
        // interior cavity (wet+dry), open at the back
        translate([wall_t, wall_t, floor_top]) cube([env_w-2*wall_t, env_d, deck_bot-floor_top]);
        // recessed rear service bay (dark inset on the back)
        translate([70, env_d-30, foot_h+12]) cube([env_w-140, 31, base_h-foot_h-24]);
        // pill status slot on the front
        translate([240-48, -1, 50]) pill_slot();
        // deck drain hole over the wet side -> reservoir
        translate([200,150,deck_bot-1]) cylinder(h=20, d=26);
    }
}
module pill_slot() { rotate([-90,0,0]) translate([0,0,0]) pill(96, 18, wall_t+2); }

// raised wood-look shelf with a recessed circular pot well
module wood_shelf() {
    difference() {
        translate([240-shelf_w/2, 160-shelf_d/2, base_h]) rbox([shelf_w, shelf_d, shelf_h], 16);
        translate([240,160, base_h+shelf_h-well_depth]) cylinder(h=well_depth+1, d=well_d);
    }
}

// removable ~9.5 L pot — hollow, seated in the well
module pot_placeholder() {
    top_d=270; bot_d=232; pot_h=215; pwall=9; pfloor=14;
    z0 = base_h + shelf_h - well_depth;
    translate([240,160,z0]) difference() {
        cylinder(h=pot_h, d1=bot_d, d2=top_d);
        translate([0,0,pfloor]) cylinder(h=pot_h, d1=bot_d-2*pwall, d2=top_d-2*pwall);
        translate([0,0,-1]) cylinder(h=pfloor+2, d=22);
    }
}

// sealed isolating wall between wet (left) and dry (right)
module iso_wall() { translate([iso_x-iso_t/2, wall_t, floor_top]) cube([iso_t, env_d-wall_t, deck_bot-floor_top]); }

// reservoir — WET compartment (left)
module reservoir_placeholder() {
    rw = iso_x-iso_t/2-wall_t-16; rd=150; rh=70;
    translate([wall_t+8, 150, floor_top+4]) rbox([rw, rd, rh], 6);
}

// electronics — DRY compartment (right), dark recessed service parts
module pcb()         { translate([iso_x+iso_t/2+14, 96, floor_top+12]) cube([90,120,8]); }
module led_driver()  { translate([iso_x+iso_t/2+14, 226, floor_top+6]) rbox([100,60,34],3); }
module power_input() { translate([iso_x+iso_t/2+14, 26, floor_top+6]) rbox([90,56,30],3); }
module electronics_bay() { pcb(); led_driver(); power_input(); }

// small pill status diffuser (4 tiny LED dots) on the front face
module status_diffuser() {
    translate([240-48, 0.5, 50]) {
        pill(96, 18, 5);
        for (i=[0:3]) translate([20 + i*18.7, 9, 5]) cylinder(h=2, d=7);
    }
}

// ------------------------------- assembly ---------------------------------
WHITE=[0.93,0.93,0.91]; WOOD=[0.80,0.62,0.38]; DARK=[0.26,0.26,0.29];
module assembly() {
    color(WHITE)            side_frames();
    color(WHITE)            base_shell();
    color(WHITE)            light_bridge();
    color(WOOD)             wood_shelf();
    color([0.96,0.90,0.45]) led_bar();
    color([0.82,0.55,0.42]) pot_placeholder();
    color([0.30,0.62,0.88]) reservoir_placeholder();
    color(DARK)             pcb();
    color(DARK)             led_driver();
    color(DARK)             power_input();
    color([0.88,0.88,0.90]) iso_wall();
    color([0.30,0.82,0.55]) status_diffuser();
    color([0.55,0.55,0.58]) feet();
}

// ------------------------- per-part dispatch ------------------------------
part = "all";
if      (part=="all")        assembly();
else if (part=="side_frames") side_frames();
else if (part=="base")       base_shell();
else if (part=="bridge")     light_bridge();
else if (part=="shelf")      wood_shelf();
else if (part=="led_bar")    led_bar();
else if (part=="pot")        pot_placeholder();
else if (part=="reservoir")  reservoir_placeholder();
else if (part=="pcb")        pcb();
else if (part=="driver")     led_driver();
else if (part=="power")      power_input();
else if (part=="iso_wall")   iso_wall();
else if (part=="status")     status_diffuser();
else if (part=="feet")       feet();
