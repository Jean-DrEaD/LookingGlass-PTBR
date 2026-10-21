cd D:\LookingGlass-PTBR
$p = 'src\platform'; $mf = "$p\math.rs"; $b = "$p\macroquad_backend.rs"; $m = "$p\mod.rs"
$files = 'src\menu\save_screen.rs','src\menu.rs','src\ui.rs'
$orig = @{}; foreach ($f in @($mf,$b,$m) + $files) { $orig[$f] = Get-Content $f -Raw }

# 1) Rect proprio no math.rs
$rect = @'

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
'@
$newMath = ($orig[$mf] -replace '(?ms)\r?\n// TODO\(xbox\).*$', '').TrimEnd() + "`r`n" + $rect
Set-Content $mf $newMath -NoNewline

# 2) Conversoes + helper no backend
Add-Content $b @'

impl From<crate::platform::math::Rect> for macroquad::math::Rect {
    fn from(r: crate::platform::math::Rect) -> Self { Self::new(r.x, r.y, r.w, r.h) }
}
impl From<macroquad::math::Rect> for crate::platform::math::Rect {
    fn from(r: macroquad::math::Rect) -> Self { Self::new(r.x, r.y, r.w, r.h) }
}
'@
Add-Content $m @'

/// Converte Rect portavel para o `source` do backend.
#[cfg(feature = "backend-macroquad")]
pub fn src_rect(r: impl Into<macroquad::math::Rect>) -> Option<macroquad::math::Rect> { Some(r.into()) }
'@

# 3) Corrige os 5 pontos
foreach ($f in $files) {
    Set-Content $f ($orig[$f] -replace 'source:\s*Some\(', 'source: crate::platform::src_rect(') -NoNewline
}

"math sem macroquad: " + -not (Select-String -Path $mf -Pattern 'macroquad' -Quiet)
$out = cargo check 2>&1 | Out-String
$ok = $out -notmatch '(?m)^error'
"CHECK: " + $(if ($ok) {'OK'} else {'FALHOU'})
if ($ok) {
    pwsh -NoProfile -File tools\platform-guard.ps1
    Add-Content docs\STATUS.md "- [ok] Lote 9: Rect proprio; platform::math 100% portavel; src_rect() no backend."
    git add -A; git commit -m "platform: Rect proprio, math sem macroquad, .gitattributes"; git push
} else {
    $out -split "`n" | Select-String '^error' -Context 0,4 | Select-Object -First 10
    foreach ($k in $orig.Keys) { Set-Content $k $orig[$k] -NoNewline }
    "ROLLBACK feito"
}
