mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

use one::One;
uniffi::ffi_converter_forward!(One, one::UniFfiTag, crate::UniFfiTag);

uniffi::setup_scaffolding!("two");
