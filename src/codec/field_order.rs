use avCodecType::AVFieldOrder;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FieldOrder {
    Unknown = 0,
    Progressive = 1,
    TT = 2,
    BB = 3,
    TB = 4,
    BT = 5,
}

impl From<AVFieldOrder> for FieldOrder {
    fn from(value: AVFieldOrder) -> Self {
        match value {
            value if value == 0 => FieldOrder::Unknown,
            value if value == 1 => FieldOrder::Progressive,
            value if value == 2 => FieldOrder::TT,
            value if value == 3 => FieldOrder::BB,
            value if value == 4 => FieldOrder::TB,
            value if value == 5 => FieldOrder::BT,
            _ => FieldOrder::Unknown
        }
    }
}

impl From<FieldOrder> for AVFieldOrder {
    fn from(value: FieldOrder) -> AVFieldOrder {
        value as AVFieldOrder
    }
}
