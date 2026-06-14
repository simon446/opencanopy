// OpenCanopy Tabletop Pepper — v1 product model (two-pillar Scandinavian architecture)
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// MAJOR REDESIGN (supersedes the arched-frame model). Architecture:
//   - one low integrated BASE (the product body): a 6 L passive self-watering reservoir +
//     an integrated grow well. The base is a SINGLE WET ZONE; there is NO electronics bay
//     in the base (electronics moved to the top — see below).
//   - TWO vertical wooden cylindrical PILLARS, centred on the base depth, rising from dry
//     structural bosses.
//   - one horizontal top LED BLOCK spanning the pillars; below it, ONE centered LED PANEL (8×6
//     emitters across one board, WI-PL-06) on a finned passive HEATSINK, single central mount.
//     The small 1.6 mm controller+driver PCB is ENCAPSULATED inside the block
//     (in an internal bay on standoff bosses), with a USB-C port through the rear face.
//   - a removable RAISED GROW INSERT (slotted/perforated, semi-hydro) for ONE pepper plant.
//   - passive self-watering: reservoir below + wicking. NO pump, NO fan, no screen/controls,
//     4 status LEDs only.
//
// Wet/dry separation is now TOP (electronics) vs BOTTOM (water) — not an in-base wall. Only
// sealed low-voltage sensor leads + status-LED light pipes touch the base, entering through a
// grommet at the right pillar; power (USB-C) enters at the TOP. Pillars and the grow module
// share Y (=160, base centre) so a THIN block puts the LED directly over the plant. The base
// stays low (135 mm) because the grow insert is a RAISED planter rising above the top.
//
// Per-part export:  openscad -D 'part="base"' --render -o base.stl this_file.scad
// Origin = front-left-bottom. X=width(0..480) Y=depth(0..320) Z=height(0..680). mm

// ----------------------------- parameters --------------------------------
env_w = 480; env_d = 320; env_h = 680;
$fn = 64;

// base (the product body — single wet zone)
foot_h  = 12;
base_h  = 135;                 // base top plane (visible base from foot top = 123 mm, <=130)
wall_t  = 12; floor_t = 10; top_t = 8;
base_rv = 16; base_re = 4;     // selective radii: vertical corners R16, top/bottom R4
floor_z = foot_h + floor_t;    // 22  inner floor
ceil_z  = base_h - top_t;      // 127 underside of base top

// grow module / LED optical center (centred on the base)
gm_x = 240; gm_y = 160;

// grow well opening (top of base) — rounded rectangle
well_w = 270; well_d = 190; well_r = 8;

// removable raised grow insert (semi-hydro basket)
ins_w = 244; ins_d = 180; ins_wall = 6; ins_r = 10;
ins_bot_z = 110;               // slotted bottom rests at the reservoir top (wick contact)
ins_top_z = 250;               // rises to 250 -> ~115 mm raised planter above base top

// passive reservoir (wet) — sits on the inner floor BETWEEN the two pillar bosses
res_x0 = 90; res_x1 = 390; res_y0 = 40; res_y1 = 290; res_z0 = floor_z; res_z1 = ins_bot_z - 0.5; // 0.5 mm wick gap

// wooden pillars (rise from dry structural bosses in the base; centred on base depth)
pil_d = 28; pil_r = 14; pil_x0 = 64; pil_x1 = env_w - 64; pil_y = gm_y;
boss_d = 42;                   // dry structural boss around each pillar foot
socket_d = 30;                 // pillar seats 30 mm into base and into block
pil_bot_z = base_h - socket_d; // 105
pil_top_z = env_h - 48 + socket_d;  // 30 mm up into the block (= 662)

// top LED block (mostly rectangular; only the pillar-hole ends are rounded)
blk_l = 440; blk_d = 60; blk_h = 48;
blk_z = env_h - blk_h;         // 632
blk_x0 = (env_w - blk_l)/2;    // 20
blk_y0 = gm_y - blk_d/2;       // 130
blk_end_r = 16;                // end-cap radius (around pillar holes)
blk_edge_r = 2;                // long edges

// grow LED PANEL + heatsink — ONE centered board (emitters spread across its face, 8×6 grid,
// per PL-06 fixture C) on a finned passive heatsink, single central mount under the block.
// Sized to fit between the pillar inner faces (≤ ~324 mm) — panel meets uniformity at the
// 150 mm target clearance, where a strip/bar needed ≥200–225 mm (WI-PL-06).
pan_w = 300; pan_d = 210; pan_t = 3;            // panel board (X × Y × thickness)
emit_z = 600;                                   // emitter plane (faces down); board = emit_z..+pan_t
emit_nx = 8; emit_ny = 6;                        // emitter grid across the panel face
hs_base = 6; hs_fin_h = 22; hs_fin_t = 3; hs_fin_pitch = 18;   // finned heatsink (on the board's back)

// encapsulated PCB bay inside the block right end (around the right pillar)
bay_x0 = 384; bay_x1 = 454; bay_z0 = 634; bay_z1 = 648;     // internal void (2 mm skin below)
pcb_x0 = 388; pcb_x1 = 450; pcb_t = 1.6; pcb_z = 638;        // 1.6 mm board on standoff bosses
pcb_my = [gm_y-16, gm_y+16]; pcb_mx = [394, 444];           // 4 mounting-hole / boss positions

// joints / hardware
m4 = 4.5; m4_head = 8; dowel_d = 4; setscrew = 4.5; m25 = 2.7;
pil_clear = 0.6;               // pillar-to-socket diametral clearance

// selective-radius box: vertical-corner radius rv, top/bottom edge radius re
module rcube(size, rv, re) {
    re = min(re, rv, size[2]/2 - 0.01); rv = max(rv, re);
    minkowski() {
        translate([re, re, re]) linear_extrude(max(0.2, size[2] - 2*re))
            offset(r=rv-re) offset(delta=-(rv-re))
                square([max(0.2,size[0]-2*re), max(0.2,size[1]-2*re)]);
        sphere(r=re, $fn=18);
    }
}
// horizontal pill on the front face (L along X, ht along Z, depth along Y), centred at x
module front_pill(L, ht, depth) { hull() for (x=[-L/2+ht/2, L/2-ht/2]) translate([x,0,0]) rotate([-90,0,0]) cylinder(h=depth, d=ht); }
// rounded-rectangle prism centred on (cx,cy), corner radius r, from z0 height h
module rrect(cx, cy, w, d, r, z0, h) { translate([cx,cy,z0]) linear_extrude(h) offset(r=r) offset(delta=-r) square([w,d], center=true); }

// ------------------------------- modules ----------------------------------
module feet() { for (x=[46, env_w-46], y=[46, env_d-46]) translate([x,y,0]) cylinder(h=foot_h, r1=17, r2=15); }

// BASE SHELL — low single-wet-zone body; cavity, grow well, two DRY pillar bosses, status
// slot, fill port, deck drain, underside pillar joints. All hardware in clearance holes.
module base_shell() {
    difference() {
        union() {
            difference() {
                translate([0,0,foot_h]) rcube([env_w, env_d, base_h-foot_h], base_rv, base_re);
                translate([wall_t, wall_t, floor_z]) cube([env_w-2*wall_t, env_d-2*wall_t, ceil_z-floor_z]); // wet cavity
                rrect(gm_x, gm_y, well_w, well_d, well_r, ceil_z-1, top_t+2);          // grow well opening
                translate([gm_x, 42, ceil_z-1]) cylinder(h=top_t+2, d=30);             // fill port (front apron)
                translate([gm_x, -1, 70]) front_pill(54, 9, wall_t+2);                 // status slot
                translate([gm_x, gm_y, ceil_z-1]) cylinder(h=top_t+2, d=20);           // deck drain (wet)
            }
            for (xc=[pil_x0, pil_x1]) translate([xc, pil_y, floor_z]) cylinder(h=base_h-floor_z, d=boss_d); // dry pillar bosses
        }
        for (xc=[pil_x0, pil_x1]) {
            translate([xc, pil_y, pil_bot_z]) cylinder(h=socket_d+2, d=pil_d+pil_clear);   // socket (from top)
            translate([xc, pil_y, -1]) cylinder(h=base_h+2, d=m4);                          // M4 shank from underside
            translate([xc, pil_y, -1]) cylinder(h=foot_h+4, d=m4_head);                     // underside counterbore
            translate([xc, pil_y+10, pil_bot_z-1]) cylinder(h=socket_d+3, d=dowel_d+0.4);   // anti-rotation dowel
        }
        translate([pil_x1, pil_y+boss_d/2-3, ceil_z-1]) cylinder(h=top_t+2, d=8);           // sealed sensor-lead grommet
    }
}

// WOODEN PILLAR — cylinder with tiny end chamfers; bottom into base socket (M4 insert pocket +
// dowel clearance), top into block. A rear flat keys the cable clip.
module pillar(xc) {
    translate([xc, pil_y, pil_bot_z]) difference() {
        union() {
            cylinder(h=pil_top_z-pil_bot_z, d=pil_d);
            cylinder(h=1.2, d1=pil_d-2, d2=pil_d);
            translate([0,0,pil_top_z-pil_bot_z-1.2]) cylinder(h=1.2, d1=pil_d, d2=pil_d-2);
        }
        translate([-pil_d/2, pil_d/2-1.5, base_h-pil_bot_z+8]) cube([pil_d, 2, pil_top_z-base_h-20]); // rear cable flat
        translate([0, 0, -1]) cylinder(h=26, d=6);                       // M4 insert pocket (bottom)
        translate([0, 10, -1]) cylinder(h=socket_d+1, d=dowel_d+0.4);    // anti-rotation dowel clearance
    }
}
module pillar_left()  { pillar(pil_x0); }
module pillar_right() { pillar(pil_x1); }
module pillars() { pillar_left(); pillar_right(); }

// TOP LED BLOCK — mostly rectangular; rounded only at the pillar-hole ends. Pillar sockets,
// rear set screws, an underside LED recess, and an INTERNAL PCB bay (right end, around the
// right pillar) with 4 standoff bosses + a USB-C port through the rear face. The block prints
// in two parts (body + bottom lid) so the encapsulated board can be installed.
module light_block() {
    difference() {
        union() {
            difference() {
                translate([blk_x0, blk_y0, blk_z]) rcube([blk_l, blk_d, blk_h], blk_end_r, blk_edge_r);
                for (xc=[pil_x0, pil_x1]) {
                    translate([xc, pil_y, blk_z-1]) cylinder(h=socket_d+1, d=pil_d+pil_clear);                 // pillar socket
                    translate([xc, blk_y0+blk_d+1, blk_z+blk_h/2]) rotate([90,0,0]) cylinder(h=blk_d/2+2, d=setscrew); // rear set screw
                }
                translate([gm_x, gm_y, blk_z-1]) cylinder(h=14, d=m4+5);                                      // grow-panel central mount boss/insert
                translate([bay_x0, gm_y-24, bay_z0]) cube([bay_x1-bay_x0, 48, bay_z1-bay_z0]);                 // PCB bay (internal void)
                translate([430, blk_y0+blk_d-8, pcb_z+1]) cube([20, 10, 8]);                                   // USB-C port (rear face)
            }
            for (mx=pcb_mx) for (my=pcb_my) translate([mx, my, bay_z0]) cylinder(h=pcb_z-bay_z0, d=6);         // PCB standoff bosses
        }
        for (mx=pcb_mx) for (my=pcb_my) translate([mx, my, bay_z0-1]) cylinder(h=pcb_z-bay_z0+2, d=m25-0.3);   // boss screw pilots
    }
}

// grow LED PANEL — one centered board with an 8×6 emitter grid spread across the down-facing
// face (PL-06 fixture C). Optical centre = grow-module centre (gm_x, gm_y). Single central mount.
module led_panel() {
    difference() {
        translate([gm_x-pan_w/2, gm_y-pan_d/2, emit_z]) rcube([pan_w, pan_d, pan_t], 6, 1);   // board
        translate([gm_x, gm_y, emit_z-1]) cylinder(h=pan_t+2, d=m4+0.6);                       // central mount clearance
    }
    for (i=[0:emit_nx-1]) for (j=[0:emit_ny-1])                                                // emitters (face down)
        translate([gm_x + (i-(emit_nx-1)/2)*34, gm_y + (j-(emit_ny-1)/2)*32, emit_z-1.2]) cube([6,6,1.4], center=true);
}

// passive finned heatsink on the panel's back; base bonded to the board, fins rise to the block.
module heatsink() {
    hs0 = emit_z + pan_t;
    difference() {
        union() {
            translate([gm_x-pan_w/2, gm_y-pan_d/2, hs0]) rcube([pan_w, pan_d, hs_base], 6, 1);  // base plate
            for (x = [-(pan_w/2-14) : hs_fin_pitch : (pan_w/2-14)])
                translate([gm_x+x-hs_fin_t/2, gm_y-pan_d/2+6, hs0+hs_base]) cube([hs_fin_t, pan_d-12, hs_fin_h]); // fins (along Y)
        }
        translate([gm_x, gm_y, hs0-1]) cylinder(h=hs_base+hs_fin_h+2, d=m4+0.6);                  // central mount clearance
    }
}

// ---- encapsulated controller + driver PCB (1.6 mm) — inside the block bay, on the bosses ----
module controller_pcb() {
    difference() {
        translate([pcb_x0, gm_y-22, pcb_z]) cube([pcb_x1-pcb_x0, 44, pcb_t]);
        translate([pil_x1, gm_y, pcb_z-1]) cylinder(h=pcb_t+2, d=pil_d+4);             // right pillar passes through
        for (mx=pcb_mx) for (my=pcb_my) translate([mx, my, pcb_z-1]) cylinder(h=pcb_t+2, d=m25); // 4 mounting holes
    }
}
// USB-C input jack — on the PCB, poking out the rear port (the only thing visible from outside)
module usb_c() { translate([432, gm_y+16, pcb_z+pcb_t]) cube([16, blk_y0+blk_d-(gm_y+16), 7]); }
module top_electronics() { controller_pcb(); usb_c(); }

// REMOVABLE GROW INSERT — raised rounded-rect basket; slotted lower walls + perforated floor
// for capillary/wick contact with the reservoir. Lifts straight up out of the well.
module grow_insert() {
    h = ins_top_z - ins_bot_z;
    difference() {
        rrect(gm_x, gm_y, ins_w, ins_d, ins_r, ins_bot_z, h);
        rrect(gm_x, gm_y, ins_w-2*ins_wall, ins_d-2*ins_wall, max(1,ins_r-ins_wall), ins_bot_z+ins_wall, h);
        for (i=[-2:2], j=[-1:1]) translate([gm_x+i*40, gm_y+j*45, ins_bot_z-1]) cylinder(h=ins_wall+2, d=12);   // perforated floor
        for (i=[-2:2]) for (sy=[-1,1]) translate([gm_x+i*40, gm_y+sy*ins_d/2, ins_bot_z+10]) cube([14, ins_wall+4, 26], center=true); // slots F/B
        for (j=[-1:1]) for (sx=[-1,1]) translate([gm_x+sx*ins_w/2, gm_y+j*45, ins_bot_z+10]) cube([ins_wall+4, 14, 26], center=true);  // slots sides
    }
}

// passive reservoir water volume (placeholder) — on the inner floor, between the pillar bosses
module reservoir() { translate([res_x0, res_y0, res_z0]) rcube([res_x1-res_x0, res_y1-res_y0, res_z1-res_z0], 6, 3); }

// WS2812B-2020 addressable LED — true datasheet outline (2.0 × 2.0 × 0.8 mm body + emitter
// lens), matching the electrical team's vendor model `electronics/pcb/3d-models/
// WS2812B-2020_C965555.step` (the disjoint vendor tessellation isn't CGAL-importable in
// OpenSCAD 2021.01, so we use the datasheet outline per that model's README; see vendor/README.md).
// Centred at origin, emitter on +Z.
module ws2812b_2020() {
    translate([-1, -1, 0]) cube([2.0, 2.0, 0.8]);          // package body
    translate([0, 0, 0.8]) cylinder(h=0.2, d=1.5);          // emitter lens/window
}

// 4 status LEDs (WS2812B-2020) behind a front pill diffuser (no screen/controls), facing
// forward through the diffuser. (Electronics PCB2 still carries 5 — ECO-003 reduces it to 4.)
module status_diffuser() {
    translate([gm_x, -1, 70]) front_pill(52, 8, 6);
    for (i=[0:3]) translate([gm_x-21+i*14, 6.0, 70]) rotate([90,0,0]) ws2812b_2020();
}

// fill-port plug/cap (sits in the port, proud lip)
module fill_cap() { translate([gm_x, 42, ceil_z]) cylinder(h=base_h-ceil_z+3, d=29.4); }

// joint hardware (validation/exploded views)
module dowels() { for (xc=[pil_x0,pil_x1]) translate([xc, pil_y+10, pil_bot_z]) cylinder(h=socket_d-2, d=dowel_d-0.2); }
module screws() {
    for (xc=[pil_x0,pil_x1]) {                                          // pillar-to-base M4 from underside
        translate([xc, pil_y, 2]) cylinder(h=base_h-16, d=m4-0.6);
        translate([xc, pil_y, 1]) cylinder(h=5, d=m4_head-1);
        translate([xc, blk_y0+blk_d-2, blk_z+blk_h/2]) rotate([90,0,0]) cylinder(h=14, d=setscrew-0.8); // block set screw
    }
    translate([gm_x, gm_y, emit_z-1]) cylinder(h=blk_z-emit_z+8, d=m4-0.6);   // grow-panel central mount screw (up into the block)
    translate([gm_x, gm_y, emit_z-4]) cylinder(h=5, d=m4_head-1);             // head below the panel
    for (mx=pcb_mx) for (my=pcb_my) translate([mx, my, bay_z0]) cylinder(h=pcb_z-bay_z0+4, d=m25-0.6); // PCB mount screws
}
// cabling: sensor leads up the rear of the right pillar (base -> bay) + USB-C tail out the rear
module cable_path() {
    cy = pil_y + pil_d/2 + 3.5;
    translate([pil_x1, cy, base_h]) cylinder(h=bay_z0-base_h, d=6);     // sensor bundle up rear of R pillar to bay
    translate([pil_x1, cy, base_h+4]) sphere(d=6);                      // strain relief at base
    translate([440, blk_y0+blk_d+2, pcb_z+4]) rotate([-90,0,0]) cylinder(h=16, d=6); // USB-C tail out the rear port
}

// ------------------------------- assembly ---------------------------------
WHITE=[0.90,0.90,0.89]; WOOD=[0.78,0.60,0.36]; DARK=[0.24,0.24,0.27]; BASKET=[0.36,0.40,0.37];
module assembly() {
    color(WHITE)  base_shell();
    color(WOOD)   pillars();
    color(WHITE)  light_block();
    color([0.72,0.74,0.78]) heatsink();
    color([0.97,0.92,0.5]) led_panel();
    color([0.15,0.7,0.3]) controller_pcb();
    color([0.2,0.2,0.22]) usb_c();
    color(BASKET) grow_insert();
    color([0.30,0.62,0.88]) reservoir();
    color([0.30,0.82,0.55]) status_diffuser();
    color(WHITE)  fill_cap();
    color([0.55,0.55,0.58]) feet();
    color([0.6,0.6,0.62]) dowels();
    color([0.2,0.2,0.22]) screws();
}

// ------------------------- per-part dispatch ------------------------------
part = "all";
if      (part=="all")          assembly();
else if (part=="base")         base_shell();
else if (part=="pillar_left")  pillar_left();
else if (part=="pillar_right") pillar_right();
else if (part=="pillars")      pillars();
else if (part=="light_block")  light_block();
else if (part=="led_panel")    led_panel();
else if (part=="heatsink")     heatsink();
else if (part=="pcb")          controller_pcb();
else if (part=="usb_c")        usb_c();
else if (part=="grow_insert")  grow_insert();
else if (part=="reservoir")    reservoir();
else if (part=="status")       status_diffuser();
else if (part=="fill_cap")     fill_cap();
else if (part=="feet")         feet();
else if (part=="dowels")       dowels();
else if (part=="screws")       screws();
else if (part=="cable")        cable_path();
