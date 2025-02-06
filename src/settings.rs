use std::ffi::c_int;
use gl::types::GLenum;
use glfw::ffi::{KEY_A, KEY_D, KEY_E, KEY_ENTER, KEY_ESCAPE, KEY_Q, KEY_S, KEY_TAB, KEY_W, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT};


/// тип рендера треугольники
pub const TRIANGLES: GLenum = gl::TRIANGLES;


/// Левая кнопка мыши
pub const LCM: c_int = MOUSE_BUTTON_LEFT;
/// Правая кнопка мыши
pub const PCM: c_int = MOUSE_BUTTON_RIGHT;
/// Средняя кнопка мыши
pub const SCM: c_int = MOUSE_BUTTON_MIDDLE;
pub const ESCAPE: c_int = KEY_ESCAPE;
pub const TAB: c_int = KEY_TAB;
pub const W: c_int = KEY_W;
pub const A: c_int = KEY_A;
pub const S: c_int = KEY_S;
pub const D: c_int = KEY_D;
pub const Q: c_int = KEY_Q;
pub const E: c_int = KEY_E;