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

// one side-frame arch (thin in X); + foot tenons that plug into base sockets
module side_arch(x0) {
    difference() {
        translate([x0, arch_y0, base_h]) rotate([90,0,90]) linear_extrude(side_t) arch2d();
        // conduit bore up the BACK leg of the RIGHT arch only
        if (x0 > env_w/2)
            translate([x0+side_t/2, arch_y1-arch_fw/2, base_h-1]) cylinder(h=arch_h*0.9, d=conduit_id);
    }
    // foot tenons (front + back legs) extending DOWN into base sockets
    for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2])
        translate([x0+side_t/2, yc, base_h-socket_depth]) cube([side_t-8, arch_fw-10, socket_depth+2], center=false)
            ; // (centered below)
}
// tenons modelled centered:
module arch_tenons(x0) {
    for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2])
        translate([x0+side_t/2, yc, base_h-socket_depth+1]) translate([-(side_t-10)/2,-(arch_fw-12)/2,0])
            cube([side_t-10, arch_fw-12, socket_depth]);
}
module left_arch()  { side_arch(0); arch_tenons(0); }
module right_arch() { side_arch(env_w-side_t); arch_tenons(env_w-side_t); }
module side_frames() { left_arch(); right_arch(); }

// slim top light bridge, CENTERED on Y=160; with bridge tongues into arch-top sockets
module light_bridge() {
    rcube([env_w - 2*side_t + 16, bridge_d, bridge_h], 10, 5)
        ; // placed in assembly via translate
}
module bridge_placed() { translate([side_t-8, bridge_y0, env_h-bridge_h]) light_bridge(); }

// LED grow bar — optical centerline at (led_cx, led_cy), under the bridge
module led_bar() {
    led_l=300; led_w=64; led_h=18;
    translate([led_cx-led_l/2, led_cy-led_w/2, env_h-bridge_h-led_h]) rcube([led_l, led_w, led_h], 6, 3);
}

// base shell: selective radii, OPEN back + rear service bay, arch foot sockets, M4 access
module base_shell() {
    difference() {
        translate([0,0,foot_h]) rcube([env_w, env_d, base_h-foot_h], base_rv, base_re);
        translate([wall_t, wall_t, floor_top]) cube([env_w-2*wall_t, env_d, deck_bot-floor_top]); // cavity, open back
        translate([70, env_d-30, foot_h+12]) cube([env_w-140, 31, base_h-foot_h-24]);             // rear service bay
        translate([led_cx-48,-1,52]) rotate([-90,0,0]) pill_2d_extrude(96,18,wall_t+2);            // status slot
        translate([200,160,deck_bot-1]) cylinder(h=20,d=26);                                       // deck drain (wet side)
        // arch foot SOCKETS (receive the tenons) + dowel holes + underside M4 access
        for (x0=[0, env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) {
            translate([x0+side_t/2-(side_t-10)/2, yc-(arch_fw-12)/2, deck_bot-socket_depth])
                cube([side_t-10, arch_fw-12, socket_depth+1]);                                      // socket
            for (dx=[-7,7]) translate([x0+side_t/2+dx, yc, base_h-socket_depth-6]) cylinder(h=socket_depth+8,d=dowel_d); // dowels
            translate([x0+side_t/2, yc, foot_h-1]) cylinder(h=base_h, d=m4);                        // M4 shank up to insert
            translate([x0+side_t/2, yc, -1]) cylinder(h=foot_h+1, d=m4_head);                       // underside counterbore (driver access)
        }
    }
}
module pill_2d_extrude(L,w,t) { linear_extrude(t) hull() for(x=[w/2,L-w/2]) translate([x,w/2]) circle(d=w); }

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
// reservoir — WET (left), kept clear of the corner arch feet (Y) and the deck (Z)
module reservoir_placeholder() { translate([24,156,floor_top+2]) rcube([300,124,76],6,4); }
// electronics — DRY (right), placed in the mid-depth band to clear the corner feet
module pcb()         { translate([iso_x+iso_t/2+16, 98, floor_top+12]) cube([90,116,8]); }
module led_driver()  { translate([iso_x+iso_t/2+16, 222, floor_top+6]) rcube([100,56,34],3,3); }
module power_input() { translate([iso_x+iso_t/2+16, 36, floor_top+6]) rcube([86,54,30],3,3); }
module electronics_bay() { pcb(); led_driver(); power_input(); }

module status_diffuser() {
    translate([led_cx-48, 0.5, 52]) { pill_2d_extrude(96,18,5); for(i=[0:3]) translate([20+i*18.7,9,5]) cylinder(h=2,d=7); }
}

// joint hardware (validation/exploded views)
module dowels() {
    for (x0=[0,env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) for (dx=[-7,7])
        translate([x0+side_t/2+dx, yc, base_h-socket_depth-2]) cylinder(h=socket_depth+8, d=dowel_d-0.2);
}
module screws() {
    for (x0=[0,env_w-side_t]) for (yc=[arch_y0+arch_fw/2, arch_y1-arch_fw/2]) {
        translate([x0+side_t/2, yc, 2]) cylinder(h=base_h-socket_depth-2, d=m4-0.6);   // M4 shank
        translate([x0+side_t/2, yc, 1]) cylinder(h=5, d=m4_head-1);                     // head
    }
}
// cable path (debug): base dry bay -> drip loop -> right-rear arch -> bridge -> LED
module cable_path() {
    d=7;
    pts_z = base_h*0.4;
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
