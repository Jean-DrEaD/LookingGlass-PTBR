//! Ponto unico de acesso ao backend grafico. Trocar aqui = trocar plataforma.
pub use macroquad as mq;
pub mod prelude {
    pub use macroquad::prelude::*;
}
