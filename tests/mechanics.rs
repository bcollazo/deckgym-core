extern crate deckgym as deckgym_lib;

mod support;

pub mod deckgym {
    pub use super::deckgym_lib::*;

    pub mod test_support {
        pub use crate::support::*;
    }
}

#[path = "mechanics/mod.rs"]
mod mechanics;
