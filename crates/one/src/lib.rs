mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::setup_scaffolding!("one");
