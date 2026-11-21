$fn = 128;

length = 40;
wall = 3;
depth = 40.1;
height = depth + wall * 2;

overcut = 5;
blade_cut_width = 0.8;
drill_hole_diameter = 3;

module subtract_drill_hole(drill_hole_position) {
  translate([drill_hole_position[0], drill_hole_position[1], -overcut])
    cylinder(h=wall + overcut * 2, d=drill_hole_diameter);
}

module subtract_miter_joint_drill_holes(miter_joint_origin) {
  subtract_drill_hole([miter_joint_origin[0] + depth / 3, miter_joint_origin[1] + depth * 2 / 3]);
  subtract_drill_hole([miter_joint_origin[0] + depth * 2 / 3, miter_joint_origin[1] + depth / 3]);
}

module subtract_miter_joint_observation_window(miter_joint_origin) {
  miter_observation_window_width = 7;
  miter_observation_window_end_clearance = wall;
  miter_observation_window_half_width = miter_observation_window_width / 2;
  miter_observation_window_start_distance = sqrt(2) * miter_observation_window_end_clearance + miter_observation_window_half_width;
  miter_observation_window_length = depth * sqrt(2) - 2 * sqrt(2) * miter_observation_window_end_clearance - miter_observation_window_width;

  translate([miter_joint_origin[0], miter_joint_origin[1], -overcut])
    rotate([0, 0, 45])
      translate([miter_observation_window_start_distance, -miter_observation_window_half_width, 0])
        cube([miter_observation_window_length, miter_observation_window_width, wall + overcut * 2]);
}

module add_drill_bushing(drill_hole_position) {
  drill_bushing_height = 30;
  drill_bushing_outer_diameter = drill_hole_diameter + wall * 3;

  difference() {
    translate([drill_hole_position[0], drill_hole_position[1], wall])
      cylinder(h=drill_bushing_height, d=drill_bushing_outer_diameter);
    translate([drill_hole_position[0], drill_hole_position[1], wall - overcut])
      cylinder(h=drill_bushing_height + overcut * 2, d=drill_hole_diameter);
  }
}

module corner() {
  difference() {
    cube([(depth + wall) * 2, (depth + wall) * 2, height]);
    translate([wall, wall, wall]) cube([depth * 3, depth, height]);
    translate([wall, wall, wall]) cube([depth, depth * 3, height]);
    translate([wall * 2 + depth, wall * 2 + depth, -overcut]) cube([depth + overcut, depth + overcut, height + overcut * 2]);
  }
}

module cover() {
  union() {
    difference() {
      union() {
        cube([depth * 2, depth, wall]);
        cube([depth, depth * 2, wall]);
      }

      subtract_miter_joint_observation_window([0, 0]);
      subtract_miter_joint_drill_holes([0, 0]);
    }

    add_drill_bushing([depth / 3, depth * 2 / 3]);
    add_drill_bushing([depth * 2 / 3, depth / 3]);
  }
}

difference() {
  corner();
  subtract_miter_joint_observation_window([wall, wall]);
  subtract_miter_joint_drill_holes([wall, wall]);
}

// translate([(depth + wall) * 2 + wall, 0, 0]) cover();
