use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if let Ok(color) = ResistorColor::from_int(value) {
        let s = match color {
            ResistorColor::Black => "Black",
            ResistorColor::Blue => "Blue",
            ResistorColor::Brown => "Brown",
            ResistorColor::Green => "Green",
            ResistorColor::Grey => "Grey",
            ResistorColor::Orange => "Orange",
            ResistorColor::Red => "Red",
            ResistorColor::Violet => "Violet",
            ResistorColor::White => "White",
            ResistorColor::Yellow => "Yellow",
        };
        return String::from(s);
    }
    String::from("value out of range")
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
