include!(concat!(env!("OUT_DIR"), "/configuration.rs"));

pub fn determine_brightness_percentage(local_hour: u8, local_minute: u8) -> u8 {
    let current_minutes = u16::from(local_hour) * 60 + u16::from(local_minute);
    let mut current_schedule_point_index = 0;

    while current_schedule_point_index + 1 < SCHEDULE_POINTS.len()
        && SCHEDULE_POINTS[current_schedule_point_index + 1].0 <= current_minutes
    {
        current_schedule_point_index += 1;
    }

    let (segment_start_minutes, segment_start_brightness_percent) =
        SCHEDULE_POINTS[current_schedule_point_index];
    let (segment_end_minutes, segment_end_brightness_percent) = SCHEDULE_POINTS
        .get(current_schedule_point_index + 1)
        .copied()
        .unwrap_or((24 * 60, SCHEDULE_POINTS[0].1));
    let elapsed_segment_minutes = current_minutes - segment_start_minutes;
    let segment_duration_minutes = segment_end_minutes - segment_start_minutes;
    let brightness_difference =
        i32::from(segment_end_brightness_percent) - i32::from(segment_start_brightness_percent);
    let brightness_change = brightness_difference * i32::from(elapsed_segment_minutes)
        / i32::from(segment_duration_minutes);

    (i32::from(segment_start_brightness_percent) + brightness_change) as u8
}
