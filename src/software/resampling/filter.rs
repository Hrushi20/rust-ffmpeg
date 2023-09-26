use super::types::SwrFilterType;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Filter {
    Cubic = 1,
    BlackmanNuttall = 2,
    Kaiser = 3,
}

impl From<SwrFilterType> for Filter {
    fn from(value: u32) -> Filter {
        match value {
            i if i == Filter::Cubic as u32 => Filter::Cubic,
            i if i == Filter::BlackmanNuttall as u32 => Filter::BlackmanNuttall,
            i if i == Filter::Kaiser as u32 => Filter::Kaiser,
            _ => Filter::Cubic
        }
    }
}

impl From<Filter> for SwrFilterType {
    fn from(value: Filter) -> u32 {
        value as u32
    }
}
