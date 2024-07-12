mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::ffi_converter_forward!(
    one::One,
    one::UniFfiTag,
    crate::UniFfiTag
);

uniffi::include_scaffolding!("two");
