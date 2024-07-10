use one::One;
use std::sync::Arc;

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

    #[uniffi::constructor]
    pub fn new(one: One, two: Two) -> Self {
        Self { one, two }
    }
}

impl From<BetaObject> for BetaRecord {
    fn from(value: BetaObject) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}
impl From<BetaRecord> for BetaObject {
    fn from(value: BetaRecord) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}

#[uniffi::export]
pub fn new_record(one: One, two: Two) -> BetaRecord {
    BetaRecord { one, two }
}

#[uniffi::export]
pub fn new_record_default() -> BetaRecord {
    BetaRecord::default()
}

#[uniffi::export]
pub fn record_ref_record(value: &BetaRecord) -> BetaRecord {
    value.clone()
}

#[uniffi::export]
pub fn record_record(value: BetaRecord) -> BetaRecord {
    value
}

#[uniffi::export]
pub fn object_ref_object(value: &BetaObject) -> BetaObject {
    value.clone()
}

#[uniffi::export]
pub fn object_object(value: Arc<BetaObject>) -> Arc<BetaObject> {
    value
}

#[uniffi::export]
pub fn record_object(value: BetaRecord) -> BetaObject {
    value.into()
}

#[uniffi::export]
pub fn object_record(value: Arc<BetaObject>) -> BetaRecord {
    let raw = Arc::<BetaObject>::into_raw(value);
    let inner = unsafe { raw.as_ref() }.unwrap().clone();
    BetaRecord::from(inner)
}
