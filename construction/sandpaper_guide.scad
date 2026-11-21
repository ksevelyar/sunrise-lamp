length = 120;
depth = 40.1;

height = depth;
wall = 4;
overcut = 5;
blade_cut_width = 0.8;

difference() {
  cube([depth + wall * 2, length + wall * 2, height + wall * 2]);
  translate([wall, -wall - overcut, wall]) cube([depth, length + wall * 2 + overcut * 2, height]);

  translate([0, length - 40, -overcut])
    rotate([0, 0, 45])
      cube([depth * 2 + wall * 2 + overcut * 2, 40, height + wall * 2 + overcut * 2]);

  translate([wall, 0, wall]) cube([0.4, 0100, height + overcut]);
}
