#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEvent {
    Created,
    Destroyed,

    CloseReq,

    Show,
    Hide,

    Minimize,
    Maximize,
    Restored,

    FGainde,
    FLost,

    Move {
        x: i32,
        y: i32
    },

    Resized {
        width: u32,
        height: u32
    },

    MEnter,
    MLeft,
    MMove {
        x: i32,
        y: i32
    },
    MButtonPress(MouseButton),
    MButtonRelease(MouseButton),
    MWheel { delta: i16 },

    KPress(Key),
    KRelease(Key),

    Charracter(char),

    FileDropped
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left, Middle, Right,
    X1, X2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Unknown,

    Escape,
    Enter,
    Space,
    Tab,
    Backspace,

    Left,
    Right,
    Up,
    Down,

    Shift,
    Control,
    Alt,

    A, B, C, D, E, F,
    G, H, I, J, K, L,
    M, N, O, P, Q, R,
    S, T, U, V, W, X,
    Y, Z,

    F1, F2, F3, F4, F5,
    F6, F7, F8, F9, F10,
    F11, F12
}