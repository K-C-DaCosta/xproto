use std::{fmt::Debug, io, mem::MaybeUninit, os::unix::net::UnixStream};

pub const PATH_TO_UNIX_DOMAIN_SOCKET: &str = "/tmp/.X11-unix/X0";

/// code that deals with io for the x protocol
pub mod xio;

/// code makes the first connection to the server
mod connection;

/// module containing all xtypes
mod xtypes;

/// module containing common x constants
mod xconsts;

use xio::write_primitive;

pub use self::{connection::*, xconsts::*, xtypes::*};

#[derive(Debug, Clone)]
pub enum XErrorKind {
    Request,
    Value { bad_val: CARD32 },
    Window { bad_id: CARD32 },
    Pixmap { bad_id: CARD32 },
    Atom { bad_id: CARD32 },
    Cursor { bad_id: CARD32 },
    Font { bad_id: CARD32 },
    Match,
    Drawable { bad_id: CARD32 },
    Access,
    Alloc,
    ColorMap { bad_id: CARD32 },
    GContext { bad_id: CARD32 },
    IDChoice { bad_id: CARD32 },
    Name,
    Length,
    Implmentation,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct XError {
    sequence_number: CARD16,
    major: CARD8,
    minor: CARD16,
    kind: XErrorKind,
}

impl XError {
    pub fn from_header(header: XErrorHeader) -> Self {
        let mut res_uninit = MaybeUninit::<Self>::zeroed();
        assert_eq!(header.error, 0, "first byte should always be zero");
        unsafe {
            let res = res_uninit.assume_init_mut();
            res.sequence_number = header.sequence_number;
            res.major = header.major;
            res.minor = header.minor;
            let bad_id = header.bad_id_or_value;
            res.kind = match header.code {
                1 => XErrorKind::Request,
                2 => XErrorKind::Value { bad_val: bad_id },
                3 => XErrorKind::Window { bad_id },
                4 => XErrorKind::Pixmap { bad_id },
                5 => XErrorKind::Atom { bad_id },
                6 => XErrorKind::Cursor { bad_id },
                7 => XErrorKind::Font { bad_id },
                8 => XErrorKind::Match,
                9 => XErrorKind::Drawable { bad_id },
                10 => XErrorKind::Access,
                11 => XErrorKind::Alloc,
                12 => XErrorKind::ColorMap { bad_id },
                13 => XErrorKind::GContext { bad_id },
                14 => XErrorKind::IDChoice { bad_id },
                15 => XErrorKind::Name,
                16 => XErrorKind::Length,
                17 => XErrorKind::Implmentation,
                _ => panic!("encountered unknown error"),
            };

            res_uninit.assume_init_read()
        }
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default, Debug)]
pub struct XErrorHeader {
    error: CARD8,
    code: CARD8,
    sequence_number: CARD16,
    bad_id_or_value: CARD32,
    minor: CARD16,
    major: CARD8,
    padding: [u8; 21],
}

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default)]
pub struct WindowValue {
    background_pixmap: Atom,
    background_pixel: CARD32,
    border_pixmap: Atom,
    border_pixel: CARD32,
    bit_gravity: CARD8,
    win_gravity: CARD8,
    backing_store: CARD8,
    backing_planes: CARD32,
    backing_pixel: CARD32,
    override_redirect: CARD8,
    save_under: CARD8,
    event_mask: CARD32,
    do_not_propagate_mask: CARD32,
    colormap: CARD32,
    cursor: CARD32,
}

pub struct WindowBuilder<'a, T> {
    ctx: &'a mut XContext<T>,
    opcode: CARD8,
    depth: CARD8,
    request_length: CARD16,
    wid: Atom,
    parent: Atom,
    x: INT16,
    y: INT16,
    width: CARD16,
    height: CARD16,
    border_width: CARD16,
    class: CARD16,
    visual: Atom,
    value_mask: CARD32,
    value_list: Vec<WindowValue>,
    window_id: Atom,
}
impl<'a, T> WindowBuilder<'a, T>
where
    T: io::Write + io::Read,
{
    pub fn new(state: &'a mut XContext<T>) -> Self {
        let mut res_unitit = MaybeUninit::<Self>::zeroed();
        let window_id = Atom(state.gen_id());
        unsafe {
            let res = res_unitit.assume_init_mut();
            res.window_id = window_id;
            res.opcode = 1;
            res.request_length = 8;
            res.wid = window_id;
            res.parent = state.info.list_of_screen[0].root;
            res.depth = 0;
            res.border_width = 0;
            res.class = 0;
            res.visual = Atom(0); //copy from parent
            res.value_mask = 0;
            res.value_list = vec![];
            res.ctx = state;

            res_unitit.assume_init_read()
        }
    }

    pub fn with_pos(mut self, pos: (i16, i16)) -> Self {
        self.x = pos.0;
        self.y = pos.1;
        self
    }

    pub fn with_height(mut self, val: CARD16) -> Self {
        self.height = val;
        self
    }

    pub fn with_width(mut self, val: CARD16) -> Self {
        self.width = val;
        self
    }

    pub fn with_visual(mut self, val: CARD32) -> Self {
        self.visual.0 = val;
        self
    }

    pub fn with_value(mut self, value: WindowValue) -> Self {
        self.request_length += 1;
        self.value_list.push(value);
        self
    }

    pub fn build(self) -> Result<(), io::Error> {
        {
            xio::write_primitive(&mut self.ctx.socket, self.opcode)?;
            xio::write_primitive(&mut self.ctx.socket, self.depth)?;
            xio::write_primitive(&mut self.ctx.socket, self.request_length)?;
            xio::write_primitive(&mut self.ctx.socket, self.wid)?;
            xio::write_primitive(&mut self.ctx.socket, self.parent)?;
            xio::write_primitive(&mut self.ctx.socket, self.x)?;
            xio::write_primitive(&mut self.ctx.socket, self.y)?;
            xio::write_primitive(&mut self.ctx.socket, self.width)?;
            xio::write_primitive(&mut self.ctx.socket, self.height)?;
            xio::write_primitive(&mut self.ctx.socket, self.border_width)?;
            xio::write_primitive(&mut self.ctx.socket, self.class)?;
            xio::write_primitive(&mut self.ctx.socket, self.visual)?;
            xio::write_primitive(&mut self.ctx.socket, self.value_mask)?;
        }
        if let Ok(header) = xio::read_primitive::<XErrorHeader, _>(&mut self.ctx.socket) {
            let err = XError::from_header(header);
            println!("err = {:?}", err);
        } else {
            println!("no errors");
        }

        map_window(&mut self.ctx.socket, self.window_id.0)?;

        Ok(())
    }
}

pub struct XContext<Socket> {
    socket: Socket,
    id_count: CARD32,
    info: connection::ConnectionAcceptedInfo,
}
impl<Socket: io::Read + io::Write> XContext<Socket> {
    pub fn gen_id(&mut self) -> CARD32 {
        let id = self.id_count;
        self.id_count += 1;
        id | self.info.resource_id_base
    }

    pub fn create_window(&mut self) -> WindowBuilder<'_, Socket> {
        WindowBuilder::new(self)
    }
}

pub fn map_window<S: io::Write>(mut socket: S, window_id: CARD32) -> io::Result<()> {
    write_primitive(&mut socket, 8u8)?;
    write_primitive(&mut socket, 0u8)?;
    write_primitive(&mut socket, 2u16)?;
    write_primitive(&mut socket, window_id)
}
