#[derive(Debug, Clone, Copy, Default)]
pub struct PowerStatus {
    pub ac_online: bool,
    pub battery_percent: u8,
}

pub fn query_power_status() -> PowerStatus {
    PowerStatus {
        ac_online: true,
        battery_percent: 255,
    }
}

pub fn get_win_accent_color() -> String {
    "#00F5FF".to_string()
}

pub fn is_dark_mode() -> bool {
    true
}
