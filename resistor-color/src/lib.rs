use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntoEnumIterator, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}


pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    if value > 9 {
        return "value out of range".to_string()
    }
    format!("{:?}", ResistorColor::from_int(value).unwrap())
}

pub fn colors() -> Vec<ResistorColor> {
    let mut unsorted: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    unsorted.sort_by(
        |&a, &b| color_to_value(a).cmp(&color_to_value(b))
    );
    unsorted
}
