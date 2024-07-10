#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct One(bool);

uniffi::custom_newtype!(One, bool);

#[uniffi::export]
pub fn new_one_default() -> One {
    One::default()
}

#[uniffi::export]
pub fn new_one(value: bool) -> One {
    One(value)
}
