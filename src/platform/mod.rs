//! Ponto unico de acesso ao backend. Trocar aqui = trocar plataforma.
#[cfg(feature = "backend-macroquad")]
mod macroquad_backend;
#[cfg(feature = "backend-macroquad")]
pub use macroquad_backend::*;

#[cfg(feature = "backend-xbox")]
mod xbox;
#[cfg(feature = "backend-xbox")]
pub use xbox::*;

#[cfg(not(any(feature = "backend-macroquad", feature = "backend-xbox")))]
compile_error!("Ative um backend: backend-macroquad ou backend-xbox");
#[cfg(all(feature = "backend-macroquad", feature = "backend-xbox"))]
compile_error!("Ative apenas UM backend");

pub mod math;

/// Converte Rect portavel para o `source` do backend.
#[cfg(feature = "backend-macroquad")]
pub fn src_rect(r: impl Into<macroquad::math::Rect>) -> Option<macroquad::math::Rect> { Some(r.into()) }
