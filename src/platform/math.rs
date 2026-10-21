//! Matematica portavel (glam). Igual em todos os backends.
pub use glam::{
    Vec2, Vec3, Vec4, IVec3, Quat, Mat3, Mat4, EulerRot,
    vec2, vec3, vec4, ivec3,
    Vec2Swizzles, Vec3Swizzles, Vec4Swizzles,
};

/// Retangulo portavel (API compativel com o Rect do backend).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 }

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self { Self { x, y, w, h } }
    pub fn point(&self) -> Vec2 { vec2(self.x, self.y) }
    pub fn size(&self) -> Vec2 { vec2(self.w, self.h) }
    pub fn center(&self) -> Vec2 { vec2(self.x + self.w * 0.5, self.y + self.h * 0.5) }
    pub fn left(&self) -> f32 { self.x }
    pub fn right(&self) -> f32 { self.x + self.w }
    pub fn top(&self) -> f32 { self.y }
    pub fn bottom(&self) -> f32 { self.y + self.h }
    pub fn move_to(&mut self, d: Vec2) { self.x = d.x; self.y = d.y; }
    pub fn scale(&mut self, sx: f32, sy: f32) { self.w *= sx; self.h *= sy; }
    pub fn offset(self, o: Vec2) -> Self { Self::new(self.x + o.x, self.y + o.y, self.w, self.h) }
    pub fn contains(&self, pt: Vec2) -> bool {
        pt.x >= self.left() && pt.x < self.right() && pt.y >= self.top() && pt.y < self.bottom()
    }
    pub fn overlaps(&self, o: &Rect) -> bool {
        self.left() <= o.right() && self.right() >= o.left()
            && self.top() <= o.bottom() && self.bottom() >= o.top()
    }
    pub fn combine_with(self, o: Rect) -> Self {
        let x = self.x.min(o.x); let y = self.y.min(o.y);
        Self::new(x, y, self.right().max(o.right()) - x, self.bottom().max(o.bottom()) - y)
    }
    pub fn intersect(&self, o: Rect) -> Option<Rect> {
        let l = self.left().max(o.left()); let r = self.right().min(o.right());
        let t = self.top().max(o.top());   let bt = self.bottom().min(o.bottom());
        (r >= l && bt >= t).then(|| Rect::new(l, t, r - l, bt - t))
    }
}