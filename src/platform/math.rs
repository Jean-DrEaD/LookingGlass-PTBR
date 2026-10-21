//! Matematica portavel (glam). Igual em todos os backends.
pub use glam::{
    Vec2, Vec3, Vec4, IVec3, Quat, Mat3, Mat4, EulerRot,
    vec2, vec3, vec4, ivec3,
    Vec2Swizzles, Vec3Swizzles, Vec4Swizzles,
};

// TODO(xbox): Rect proprio.
#[cfg(feature = "backend-macroquad")]
pub use macroquad::math::Rect;
