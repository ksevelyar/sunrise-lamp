length = 80;
depth = 40.1;
saw_blade_height = 35;

height = depth + saw_blade_height;
wall = 8;
bottom = wall / 2;
overcut = 5;
blade_cut_width = 0.8;

clamp_extension = 20;

difference() {
  union() {
    cube([depth + wall * 2, length + wall * 2, height + bottom]);
    translate([-clamp_extension, 0, 0]) cube([depth + wall * 2 + clamp_extension * 2, length + wall * 2, bottom]);
  }
  translate([wall, -overcut, bottom]) cube([depth, length + wall * 2 + overcut * 2, height + overcut]);

  translate([-overcut, (length - blade_cut_width) / 2 + wall, bottom]) cube([depth + wall * 2 + overcut * 2, blade_cut_width, height + overcut]);

  for (angle = [-45, 45]) {
    cut_length = 2 * (depth + length + wall * 4 + overcut);
    translate([(depth + wall * 2) / 2, (length + wall * 2) / 2, bottom])
      rotate([0, 0, angle])
        translate([-cut_length / 2, -blade_cut_width / 2, 0])
          cube([cut_length, blade_cut_width, height + overcut]);
  }
}
