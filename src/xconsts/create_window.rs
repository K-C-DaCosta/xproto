use super::*;

pub mod value_masks {
    use super::*;
    pub const BACKGROUND_PIXMAP: CARD32 = 0x00000001;
    pub const BACKGROUND_PIXEL: CARD32 = 0x00000002;
    pub const BORDER_PIXMAP: CARD32 = 0x00000004;
    pub const BORDER_PIXEL: CARD32 = 0x00000008;
    pub const BIT_GRAVITY: CARD32 = 0x00000010;
    pub const WIN_GRAVITY: CARD32 = 0x00000020;
    pub const BACKING_STORE: CARD32 = 0x00000040;
    pub const BACKING_PLANES: CARD32 = 0x00000080;
    pub const BACKING_PIXEL: CARD32 = 0x00000100;
    pub const OVERRIDE_REDIRECT: CARD32 = 0x00000200;
    pub const SAVE_UNDER: CARD32 = 0x00000400;
    pub const EVENT_MASK: CARD32 = 0x00000800;
    pub const DO_NOT_PROPAGATE_MASK: CARD32 = 0x00001000;
    pub const COLORMAP: CARD32 = 0x00002000;
    pub const CURSOR: CARD32 = 0x00004000;
}
