use avUtilTypes::AVColorTransferCharacteristic;
use avutil_wasmedge;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TransferCharacteristic {
    Reserved0 = 0,
    BT709 = 1,
    Unspecified = 2,
    Reserved = 3,
    GAMMA22 = 4,
    GAMMA28 = 5,
    SMPTE170M = 6,
    SMPTE240M = 7,
    Linear = 8,
    Log = 9,
    LogSqrt = 10,
    IEC61966_2_4 = 11,
    BT1361_ECG = 12,
    IEC61966_2_1 = 13,
    BT2020_10 = 14,
    BT2020_12 = 15,
    SMPTE2084 = 16,
    SMPTE428 = 17,
    ARIB_STD_B67 = 18,
}

impl TransferCharacteristic {
    pub fn name(&self) -> Option<String> {
        if *self == TransferCharacteristic::Unspecified {
            return None;
        }
        unsafe {

            let transfer_id = (*self).into();
            let len = avutil_wasmedge::av_color_transfer_name_length(transfer_id) as usize;
            let name = vec![0u8;len];
            avutil_wasmedge::av_color_transfer_name(transfer_id,name.as_ptr(),len);
            Some(String::from_utf8_unchecked(name))
        }
    }
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        match value {
            value if value == 0  => TransferCharacteristic::Reserved0,
            value if value == 1  => TransferCharacteristic::BT709,
            value if value == 2  => TransferCharacteristic::Unspecified,
            value if value == 3 => TransferCharacteristic::Reserved,
            value if value == 4 => TransferCharacteristic::GAMMA22,
            value if value == 5 => TransferCharacteristic::GAMMA28,
            value if value == 6 => TransferCharacteristic::SMPTE170M,
            value if value == 7 => TransferCharacteristic::SMPTE240M,
            value if value == 8 => TransferCharacteristic::Linear,
            value if value == 9 => TransferCharacteristic::Log,
            value if value == 10 => TransferCharacteristic::LogSqrt,
            value if value == 11 => TransferCharacteristic::IEC61966_2_4,
            value if value == 12 => TransferCharacteristic::BT1361_ECG,
            value if value == 13 => TransferCharacteristic::IEC61966_2_1,
            value if value == 14 => TransferCharacteristic::BT2020_10,
            value if value == 15 => TransferCharacteristic::BT2020_12,
            value if value == 16 => TransferCharacteristic::SMPTE2084,
            value if value == 17 => TransferCharacteristic::SMPTE428,
            value if value == 18 => TransferCharacteristic::ARIB_STD_B67,
            value if value == 19 => TransferCharacteristic::Reserved0,
            _ => TransferCharacteristic::Reserved0
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        value as AVColorTransferCharacteristic
    }
}
