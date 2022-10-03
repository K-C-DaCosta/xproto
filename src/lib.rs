use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    fmt::Debug,
    io,
    mem::MaybeUninit,
    rc::Rc,
};

pub const PATH_TO_UNIX_DOMAIN_SOCKET: &str = "/tmp/.X11-unix/X0";

/// code makes the first connection to the server
mod connection;
pub mod error;
pub mod events;
pub mod property;
pub mod window;
/// module containing common x constants
pub mod xconsts;
/// code that deals with io for the x protocol
pub mod xio;
/// module containing all xtypes
pub mod xtypes;

use xio::{read_primitive, write_padding, write_primitive};

pub use self::{connection::*, error::*, property::*, window::*, xconsts::*, xtypes::*};

pub struct XContext<Socket> {
    pub socket: Rc<RefCell<Socket>>,
    id_count: Cell<CARD32>,
    info: Rc<connection::ConnectionAcceptedInfo>,
}
impl<Socket> XContext<Socket> {
    pub fn socket_cb<F>(&self, mut cb: F)
    where
        F: FnMut(&mut Socket),
    {
        let borrowed_sock = &mut *self.socket.as_ref().borrow_mut();
        cb(borrowed_sock)
    }
}
impl<S> Clone for XContext<S> {
    fn clone(&self) -> Self {
        Self {
            socket: self.socket.clone(),
            id_count: self.id_count.clone(),
            info: self.info.clone(),
        }
    }
}
impl<Socket: io::Read + io::Write> XContext<Socket> {
    pub fn gen_id(&mut self) -> CARD32 {
        let id = self.id_count.get();
        *self.id_count.get_mut() += 1;
        id | self.info.resource_id_base
    }

    pub fn create_window(&mut self) -> WindowBuilder<'_, Socket> {
        WindowBuilder::new(self)
    }
}

pub fn map_window<S: io::Write>(socket: &mut S, window_id: CARD32) -> io::Result<()> {
    write_primitive(socket, opcodes::MAP_WINDOW)?;
    write_primitive(socket, 0u8)?;
    write_primitive(socket, 2u16)?;
    write_primitive(socket, window_id)
}

#[derive(Copy, Clone)]
pub enum SynchKind {
    Synchronous = 0,
    Asynchronous = 1,
}

#[derive(Debug)]
pub enum GrabErr {
    InvalidStatus,
    InvalidReplyHeader,
    AlreadyGrabbed,
    InvalidTime,
    NotViewable,
    Frozen,
    IO(io::Error),
}

impl From<io::Error> for GrabErr {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

pub type GrabResult = Result<(), GrabErr>;

#[repr(C, packed(1))]
#[derive(Copy, Clone, Default)]
struct GrabResultMessage {
    reply: CARD8,
    status: CARD8,
    sequence_number: CARD16,
    reply_length: CARD32,
    unused: [u8; 24],
}

pub fn grab_keyboard<S: io::Write + io::Read>(
    socket: &mut S,
    owner_events: bool,
    grab_window: WINDOW,
    timestamp: Timestamp,
    pointer_mode: SynchKind,
    keyboard_mode: SynchKind,
) -> GrabResult {
    write_primitive(socket, opcodes::GRAB_KEYBOARD)?;
    write_primitive(socket, owner_events as u8)?;
    write_primitive(socket, 4u16)?; //request-length
    write_primitive(socket, grab_window)?;
    write_primitive(socket, timestamp.data())?;
    write_primitive(socket, pointer_mode as u8)?; //pointer-mode
    write_primitive(socket, keyboard_mode as u8)?; //keyboard-mode
    write_primitive(socket, &[0u8; 2][..])?; //unused

    let response = read_primitive::<GrabResultMessage, _>(socket)?;

    (response.reply == 1)
        .then(|| ())
        .ok_or(GrabErr::InvalidReplyHeader)?;

    let e = match response.status {
        0 => return Ok(()),
        1 => GrabErr::AlreadyGrabbed,
        2 => GrabErr::InvalidTime,
        3 => GrabErr::NotViewable,
        4 => GrabErr::Frozen,
        _ => GrabErr::InvalidStatus,
    };
    Err(e)
}

pub fn ungrab_keyboard<S: io::Write>(socket: &mut S, timestamp: Timestamp) -> io::Result<()> {
    write_primitive(socket, opcodes::UNGRAB_KEYBOARD)?;
    write_primitive(socket, 0u8)?; //unused
    write_primitive(socket, 2u16)?; //request-length
    write_primitive(socket, timestamp.data())
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventMode {
    AsyncPointer = 0,
    SyncPointer = 1,
    ReplayPointer = 2,
    AsyncKeyboard = 3,
    SyncKeyboard = 4,
    ReplayKeyboard = 5,
    AsyncBoth = 6,
    SyncBoth = 7,
}

pub fn allow_events<S: io::Write>(
    socket: &mut S,
    mode: EventMode,
    timestamp: Timestamp,
) -> io::Result<()> {
    write_primitive(socket, opcodes::ALLOW_EVENTS)?;
    write_primitive(socket, mode as u8)?;
    write_primitive(socket, 2u16)?; //request-length
    write_primitive(socket, timestamp.data())
}

pub fn grab_button<S: io::Write>(
    socket: &mut S,
    owner_events: bool,
    grab_window: Atom,
    event_mask: CARD16,
    pointer_mode: SynchKind,
    keyboard_mode: SynchKind,
    confine_to: Atom,
    cursor: Atom,
    button: CARD8,
    modifiers: Option<CARD16>,
) -> io::Result<()> {
    write_primitive(socket, opcodes::GRAB_BUTTON)?;
    write_primitive(socket, owner_events as u8)?;
    write_primitive(socket, 6u16)?; //request-length
    write_primitive(socket, grab_window)?;
    write_primitive(socket, event_mask)?;
    write_primitive(socket, pointer_mode as u8)?;
    write_primitive(socket, keyboard_mode as u8)?;
    write_primitive(socket, confine_to)?;
    write_primitive(socket, cursor)?;
    write_primitive(socket, button)?;
    write_primitive(socket, 0u8)?;//unused
    write_primitive(socket, modifiers.unwrap_or(0x8000))?; //any modifier
    Ok(())
}



pub fn grab_key<S: io::Write>(
    socket: &mut S,
    owner_events: bool,
    grab_window: Atom,
    modifiers: Option<CARD16>,
    key:CARD8,
    pointer_mode: SynchKind,
    keyboard_mode: SynchKind,
) -> io::Result<()> {
    write_primitive(socket, opcodes::GRAB_KEY)?;
    write_primitive(socket, owner_events as u8)?;
    write_primitive(socket, 4u16)?; //request-length
    write_primitive(socket, grab_window)?;
    write_primitive(socket, modifiers.unwrap_or(0x8000))?; //any modifier
    write_primitive(socket,key)?; //any modifier
    write_primitive(socket, pointer_mode as u8)?;
    write_primitive(socket, keyboard_mode as u8)?;
    write_primitive(socket, [0u8;3].as_slice())?;//3 bytes unused
    Ok(())
}