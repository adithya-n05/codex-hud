#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityColor {
    Green,
    Yellow,
    Red,
}

pub fn color_for_percent(percent: u8) -> SeverityColor {
    if percent >= 85 {
        SeverityColor::Red
    } else if percent >= 70 {
        SeverityColor::Yellow
    } else {
        SeverityColor::Green
    }
}

pub fn format_percent_label(percent: u8) -> String {
    format!("{percent}%")
}
