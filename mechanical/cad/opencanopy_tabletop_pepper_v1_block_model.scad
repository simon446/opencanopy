// OpenCanopy Tabletop Pepper — v1 product model (selective radii, centered LED, joints)
// SPDX-License-Identifier: CERN-OHL-S-2.0
//
// Unchanged architecture: electronics + reservoir in the BASE, side by side, separated
// by a SEALED vertical wall (wet | wall | dry); open-frame; no fan; no screen/controls;
// 4 status LEDs only.
//
// This revision: flatter/appliance look via SELECTIVE edge radii (not uniform rounding);
// LED optical centerline centered on the pot at X=240, Y=160; defined TAB-AND-SOCKET +
// DOWEL joints with M4/M3 screws and screwdriver access; a real internal CABLE CONDUIT
// (base dry bay -> right-rear arch -> bridge -> LED, ID 10 mm).
//
// Per-part export:  openscad -D 'part="base"' --render -o base.stl this_file.scad
// Origin = front-left-bottom. X=width(0..480) Y=depth(0..320) Z=height(0..680). mm

// ----------------------------- parameters --------------------------------
env_w = 480; env_d = 320; env_h = 680;
$fn = 64;

foot_h   = 14;
base_h   = 120;            // visible base height
wall_t   = 12;
base_rv  = 15;  base_re = 6;        // base vertical-corner / top-bottom radii
shelf_w  = 360; shelf_d = 250; shelf_h = 30; shelf_r = 8; well_d = 244; well_depth = 22;

side_t   = 26;             // side-frame arch thickness (X)
arch_fw  = 32;             // arch member width
arch_or  = 18;             // arch outer corner radius (14-22)
arch_ir  = 9;              // arch opening corner radius
arch_y0  = 8; arch_y1 = env_d - 8;          // arch depth span
arch_h   = env_h - base_h;

bridge_h = 52; bridge_y0 = 116; bridge_d = 88;   // centered on Y=160
led_cx = 240; led_cy = 160;                       // LED optical centerline (= pot center)
led_l = 300; led_w = 64; led_h = 18;
led_z = env_h - bridge_h - 10;   // LED nests 8 mm UP into the bridge recess; emitter 10 mm proud

iso_x = 340; iso_t = 14;
floor_top = foot_h + wall_t; deck_bot = base_h - 12;

socket_depth = 26; tenon = 18; dowel_d = 4; m4 = 4.5; m4_head = 8;   // joints
conduit_id = 10;                                                     // cable conduit

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

// 2D inverted-U arch profile (x=depth, y=height); rounded outer corners, open bottom
module arch2d() {
    ad = arch_y1 - arch_y0; fw = arch_fw;
    difference() {
        offset(r=arch_or) offset(delta=-arch_or) square([ad, arch_h]);
        translate([fw, -1]) offset(r=arch_ir) offset(delta=-arch_ir) square([ad-2*fw, arch_h-fw+1]);
    }
}

// ------------------------------- modules ----------------------------------
module feet() {
    for (x=[44, env_w-44], y=[44, env_d-44]) translate([x,y,0]) cylinder(h=foot_h, r1=16, r2=14);
}

// horizontal pill on the front face (L along X, ht along Z, depth along Y), centred
module front_pill(L, ht, depth) { hull() for (x=[-L/2+ht/2, L/2-ht/2]) translate([x,0,0]) rotate([-90,0,0]) cylinder(h=depth, d=ht); }

// one side-frame arch (thin in X) + foot tenons into base sockets. Dowel + foot-screw
// CLEARANCE HOLES are cut through the tenons so the hardware sits in clearance, not solid.
TEN_W = side_t-10; TEN_D = arch_fw-12;
module arch_frame(x0) { translate([x0, arch_y0, base_h]) rotate([90,0,90]) linear_extrude(side_t) arch2d(); }
module arch_tenons(x0) {
    for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2])
        translate([x0+side_t/2-TEN_W/2, yc-TEN_D/2, base_h-socket_depth]) cube([TEN_W, TEN_D, socket_depth]);
}
module one_arch(x0) {
    difference() {
        union() { arch_frame(x0); arch_tenons(x0); }
        if (x0 > env_w/2)                                            // conduit up the right-rear leg
            translate([x0+side_t/2, arch_y1-arch_fw/2, base_h-1]) cylinder(h=arch_h*0.9, d=conduit_id);
        for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) {            // dowel + foot-screw clearance
            for (dx=[-7,7]) translate([x0+side_t/2+dx, yc, base_h-socket_depth-1]) cylinder(h=socket_depth+2, d=dowel_d+0.4);
            translate([x0+side_t/2, yc, base_h-socket_depth-1]) cylinder(h=socket_depth+2, d=m4);
        }
    }
}
module left_arch()  { one_arch(0); }
module right_arch() { one_arch(env_w-side_t); }
module side_frames() { left_arch(); right_arch(); }

// slim top light bridge, CENTERED on Y=160; carries a LED recess in its underside so
// the LED bar nests up into it (captured, not floating) + bridge tongues into arches
module light_bridge() {
    difference() {
        translate([side_t, bridge_y0, env_h-bridge_h]) rcube([env_w-2*side_t, bridge_d, bridge_h], 10, 5); // abuts arch inner faces
        translate([led_cx-(led_l+8)/2, led_cy-(led_w+8)/2, env_h-bridge_h-1]) cube([led_l+8, led_w+8, 13]); // LED recess
        for (sx=[-1,1]) translate([led_cx+sx*(led_l/2-26), led_cy, env_h-bridge_h-1]) cylinder(h=bridge_h+2, d=m4+0.4); // LED screw clearance
    }
}
module bridge_placed() { light_bridge(); }

// LED grow bar — optical centerline at (led_cx, led_cy); top nests into the bridge recess
module led_bar() {
    difference() {
        translate([led_cx-led_l/2, led_cy-led_w/2, led_z]) rcube([led_l, led_w, led_h], 6, 3);
        for (sx=[-1,1]) translate([led_cx+sx*(led_l/2-26), led_cy, led_z-1]) cylinder(h=led_h+2, d=m4+0.4); // screw clearance
    }
}

// base shell: selective radii, OPEN back + rear service bay, arch foot sockets, M4 access
module base_shell() {
    difference() {
        translate([0,0,foot_h]) rcube([env_w, env_d, base_h-foot_h], base_rv, base_re);
        translate([wall_t, wall_t, floor_top]) cube([env_w-2*wall_t, env_d, deck_bot-floor_top]); // cavity, open back
        translate([70, env_d-30, foot_h+12]) cube([env_w-140, 31, base_h-foot_h-24]);             // rear service bay
        translate([led_cx, -1, 56]) front_pill(97, 19, wall_t+2);                                  // status slot (0.5 mm fit)
        translate([200,160,deck_bot-1]) cylinder(h=20,d=26);                                       // deck drain (wet side)
        // arch foot SOCKETS (z-aligned to tenons, +1 mm clearance) + dowel/M4 holes + access
        for (x0=[0, env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) {
            translate([x0+side_t/2-(TEN_W+1)/2, yc-(TEN_D+1)/2, base_h-socket_depth-1])
                cube([TEN_W+1, TEN_D+1, socket_depth+2]);                                           // socket
            for (dx=[-7,7]) translate([x0+side_t/2+dx, yc, foot_h-1]) cylinder(h=base_h, d=dowel_d+0.4); // dowel holes
            translate([x0+side_t/2, yc, foot_h-1]) cylinder(h=base_h, d=m4);                        // M4 shank to insert
            translate([x0+side_t/2, yc, -1]) cylinder(h=foot_h+2, d=m4_head);                       // underside counterbore (access)
        }
    }
}

// raised wood-look shelf with recessed pot well
module wood_shelf() {
    difference() {
        translate([led_cx-shelf_w/2, led_cy-shelf_d/2, base_h]) rcube([shelf_w, shelf_d, shelf_h], shelf_r, 4);
        translate([led_cx,led_cy, base_h+shelf_h-well_depth]) cylinder(h=well_depth+1, d=well_d);
    }
}

// ~9.5 L hollow pot, seated in the well, centered at (240,160)
module pot_placeholder() {
    top_d=270; bot_d=232; pot_h=215; pwall=9; pfloor=14;
    z0 = base_h + shelf_h - well_depth;
    translate([led_cx,led_cy,z0]) difference() {
        cylinder(h=pot_h,d1=bot_d,d2=top_d);
        translate([0,0,pfloor]) cylinder(h=pot_h,d1=bot_d-2*pwall,d2=top_d-2*pwall);
        translate([0,0,-1]) cylinder(h=pfloor+2,d=22);
    }
}

module iso_wall() { translate([iso_x-iso_t/2, wall_t, floor_top]) cube([iso_t, env_d-wall_t, deck_bot-floor_top]); }
// reservoir + electronics SEATED on the base inner floor (floor_top) — not floating
module reservoir_placeholder() { translate([24,156,floor_top]) rcube([300,124,76],6,4); }
module pcb()         { translate([iso_x+iso_t/2+16, 98, floor_top]) cube([90,116,8]); }
module led_driver()  { translate([iso_x+iso_t/2+16, 222, floor_top]) rcube([100,56,34],3,3); }
module power_input() { translate([iso_x+iso_t/2+16, 36, floor_top]) rcube([86,54,30],3,3); }
module electronics_bay() { pcb(); led_driver(); power_input(); }

// status pill on the FRONT face (matches the front slot) + 4 LED dots behind
module status_diffuser() {
    translate([led_cx, -1, 56]) front_pill(96, 18, 6);
    for (i=[0:3]) translate([led_cx-30+i*20, 4, 56]) rotate([-90,0,0]) cylinder(h=3, d=7);
}

// joint hardware (validation/exploded views)
module dowels() {
    for (x0=[0,env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) for (dx=[-7,7])
        translate([x0+side_t/2+dx, yc, base_h-socket_depth]) cylinder(h=socket_depth-2, d=dowel_d-0.2);
}
module screws() {
    for (x0=[0,env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) {
        translate([x0+side_t/2, yc, 2]) cylinder(h=base_h-10, d=m4-0.6);                // M4 shank up into the foot insert
        translate([x0+side_t/2, yc, 1]) cylinder(h=5, d=m4_head-1);                     // head (underside)
    }
    // LED-bar mount screws (up through the LED into the bridge) — head just below the LED
    for (sx=[-1,1]) translate([led_cx+sx*(led_l/2-26), led_cy, led_z]) {
        cylinder(h=env_h-bridge_h-led_z+12, d=m4-0.8);    // shank into bridge insert
        translate([0,0,-5]) cylinder(h=5, d=m4_head-1.5);  // head below the LED
    }
}
// cable path (debug): base dry bay -> drip loop -> right-rear arch -> bridge -> LED
module cable_path() {
    d=6;
    translate([iso_x+iso_t/2+40, 60, floor_top+20]) sphere(d=12);                         // strain relief / drip loop
    translate([env_w-side_t/2, arch_y1-arch_fw/2, base_h]) cylinder(h=arch_h*0.86, d=d);   // up right-rear arch
    translate([env_w-side_t/2, arch_y1-arch_fw/2, env_h-bridge_h-4]) rotate([0,90,0]) cylinder(h=0.1,d=d);
}

// ------------------------------- assembly ---------------------------------
WHITE=[0.90,0.90,0.89]; WOOD=[0.78,0.60,0.36]; DARK=[0.24,0.24,0.27];
module assembly() {
    color(WHITE) side_frames();
    color(WHITE) base_shell();
    color(WHITE) bridge_placed();
    color(WOOD)  wood_shelf();
    color([0.97,0.92,0.5]) led_bar();
    color([0.80,0.55,0.42]) pot_placeholder();
    color([0.30,0.62,0.88]) reservoir_placeholder();
    color(DARK) electronics_bay();
    color([0.86,0.88,0.92]) iso_wall();
    color([0.30,0.82,0.55]) status_diffuser();
    color([0.55,0.55,0.58]) feet();
    color([0.6,0.6,0.62]) dowels();
    color([0.2,0.2,0.22]) screws();
}

// ------------------------- per-part dispatch ------------------------------
part = "all";
if      (part=="all")        assembly();
else if (part=="left_arch")  left_arch();
else if (part=="right_arch") right_arch();
else if (part=="side_frames") side_frames();
else if (part=="base")       base_shell();
else if (part=="bridge")     bridge_placed();
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
else if (part=="dowels")     dowels();
else if (part=="screws")     screws();
else if (part=="cable")      cable_path();
