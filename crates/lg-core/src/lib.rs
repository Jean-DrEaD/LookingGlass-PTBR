//! Lógica do jogo independente de plataforma (desktop, Android, Xbox).
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub mod time;
pub mod entity;
pub mod weapon_rules;
pub mod school2_quest;
