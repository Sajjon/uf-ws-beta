use one::One;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Two(bool);

uniffi::custom_newtype!(Two, bool);

#[uniffi::export]
pub fn new_two_default() -> Two {
    Two::default()
}

#[uniffi::export]
pub fn new_two(value: bool) -> Two {
    Two(value)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct BetaRecord {
    one: One,
    two: Two,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct BetaObject {
    one: One,
    two: Two,
}

#[uniffi::export]
impl BetaObject {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }
}

#[uniffi::export]
pub fn new_beta_record_default() -> BetaRecord {
    BetaRecord::default()
}

#[uniffi::export]
pub fn beta_record_to_object(record: BetaRecord) -> BetaObject {
    BetaObject {
        one: record.one,
        two: record.two,
    }
}
