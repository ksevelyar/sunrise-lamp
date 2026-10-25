include!(concat!(env!("OUT_DIR"), "/configuration.rs"));

#[derive(Clone, Copy, Debug, defmt::Format, Eq, PartialEq)]
pub enum LampState {
    Day,
    Night,
}

pub fn determine_lamp_state(local_hour: u8, local_minute: u8) -> LampState {
    let current_minutes = u16::from(local_hour) * 60 + u16::from(local_minute);

    if (SUNRISE_MINUTES..SUNSET_MINUTES).contains(&current_minutes) {
        LampState::Day
    } else {
        LampState::Night
    }
}
