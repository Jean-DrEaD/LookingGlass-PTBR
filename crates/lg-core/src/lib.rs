//! Lógica do jogo independente de plataforma (desktop, Android, Xbox).
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

/// Tipos que o prelude do std traria automaticamente.
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use alloc::{borrow::ToOwned, boxed::Box, format, string::{String, ToString}, vec, vec::Vec};
}
pub mod time;
pub mod entity;
pub mod weapon_rules;
pub mod school2_quest;
pub mod i18n;
pub mod event;
pub mod model_detail;
pub mod bsp;
pub mod difficulty;
pub mod visibility;
pub mod levels_wchess1_waves;
