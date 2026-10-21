//! Ponto unico de acesso ao backend. Trocar aqui = trocar plataforma.
pub use macroquad as mq;

/// Matematica (glam) - portavel, nao muda no Xbox.
pub mod math {
    pub use macroquad::prelude::{
        Vec2, Vec3, Vec4, IVec3, Quat, Mat3, Mat4, EulerRot, Rect,
        vec2, vec3, vec4, ivec3,
    };
    pub use macroquad::math::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};
}

/// Desenho 2D/3D e cores.
pub mod gfx {
    pub use macroquad::prelude::{
        Color, BLACK, WHITE, GRAY, BLUE, GREEN, RED, BROWN, SKYBLUE,
        clear_background, draw_rectangle, draw_rectangle_lines, draw_line, draw_circle,
        draw_circle_lines, draw_text, draw_text_ex, measure_text, draw_texture_ex,
        TextParams, DrawTextureParams,
        draw_line_3d, draw_cube, draw_sphere, draw_mesh,
        Camera3D, Projection, set_camera, set_default_camera,
        Texture2D, Image, FilterMode, get_screen_data,
    };
}

/// Baixo nivel GL - maior esforco de port.
pub mod gl {
    pub use macroquad::miniquad;
    pub use macroquad::prelude::{
        Material, MaterialParams, UniformDesc, UniformType, ShaderSource, load_material,
        gl_use_material, gl_use_default_material, get_internal_gl,
        Mesh, Vertex, DrawMode, RenderTarget, RenderTargetParams, RenderPass, render_target_ex,
    };
}

/// Teclado, mouse, toque.
pub mod input {
    pub use macroquad::prelude::{
        KeyCode, MouseButton, Touch, TouchPhase, touches,
        is_key_pressed, is_key_down, is_mouse_button_pressed, is_mouse_button_down,
        mouse_position, mouse_wheel, get_char_pressed, show_mouse, set_cursor_grab,
    };
}

/// Janela, loop e tempo.
pub mod window {
    pub use macroquad::prelude::{
        next_frame, screen_width, screen_height, get_time, get_frame_time,
        is_quit_requested, prevent_quit, request_new_screen_size, set_fullscreen, Conf,
    };
}

pub mod prelude {
    pub use super::{math::*, gfx::*, gl::*, input::*, window::*};
}
