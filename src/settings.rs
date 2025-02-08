use std::ffi::c_int;
use gl::types::GLenum;
use glfw::ffi::{KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_A, KEY_D, KEY_E, KEY_ENTER, KEY_ESCAPE, KEY_F1, KEY_F2, KEY_Q, KEY_S, KEY_TAB, KEY_W, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT};


/// тип рендера: треугольники
pub const TRIANGLES: GLenum = gl::TRIANGLES;

/// Тип рендера: линии
pub const LINES: GLenum = gl::LINES;

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

/// клавиша 1
pub const K_1: c_int = KEY_1;
/// клавиша 2
pub const K_2: c_int = KEY_2;
/// клавиша 3
pub const K_3: c_int = KEY_3;
/// клавиша 4
pub const K_4: c_int = KEY_4;
/// клавиша 5
pub const K_5: c_int = KEY_5;
/// клавиша 6
pub const K_6: c_int = KEY_6;
/// клавиша 7
pub const K_7: c_int = KEY_7;
/// клавиша F1
pub const F1: c_int = KEY_F1;
/// клавиша F2
pub const F2: c_int = KEY_F2;