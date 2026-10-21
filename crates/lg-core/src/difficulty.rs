//! Nível de dificuldade (compartilhado entre core e frontends).
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use crate::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Nightmare,
}

impl Difficulty {
    pub const ALL: [Self; 4] = [Self::Easy, Self::Normal, Self::Hard, Self::Nightmare];
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "0" | "easy" => Ok(Self::Easy),
            "1" | "normal" | "medium" => Ok(Self::Normal),
            "2" | "hard" => Ok(Self::Hard),
            "3" | "nightmare" => Ok(Self::Nightmare),
            _ => anyhow::bail!("Choose Easy, Normal, Hard or Nightmare"),
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Nightmare => "Nightmare",
        }
    }
    pub fn incoming(self) -> f32 {
        [0.75, 1., 1.4, 1.75][self as usize]
    }
    pub fn outgoing(self) -> f32 {
        [1.25, 1., 0.7, 0.55][self as usize]
    }
    pub fn duration(self, seconds: f32) -> f32 {
        floor_f32(seconds * [1., 1., 0.8, 0.6][self as usize])
    }
    /// Original inhibit bits; Hard and Nightmare share the hard spawn set.
    pub fn allows(self, flags: u32) -> bool {
        flags & (1 << [8, 9, 10, 10][self as usize]) == 0
    }
    pub fn regen(self) -> (f32, f32) {
        // Original fgamex86 recovery: +0.3/+0.1 at whole-second time checks.
        // Integrate by simulation time so rendering frequency cannot change it.
        match self {
            Self::Easy => (0.3, 100.),
            Self::Normal => (0.1, 10.),
            _ => (0., 0.),
        }
    }
}

#[inline]
fn floor_f32(v: f32) -> f32 {
    let t = v as i64 as f32;
    if t > v { t - 1. } else { t }
}
