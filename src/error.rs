use super::*;

#[derive(Debug, Clone, Copy)]
pub struct XErrorGeneric {
    pub sequence_number: CARD16,
    pub major: CARD8,
    pub minor: CARD16,
}

#[derive(Debug)]
pub enum XErrorKind {
    Request {
        generic: XErrorGeneric,
    },
    Value {
        generic: XErrorGeneric,
        bad_val: CARD32,
    },
    Window {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Pixmap {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Atom {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Cursor {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Font {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Match {
        generic: XErrorGeneric,
    },
    Drawable {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Access {
        generic: XErrorGeneric,
    },
    Alloc {
        generic: XErrorGeneric,
    },
    ColorMap {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    GContext {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    IDChoice {
        generic: XErrorGeneric,
        bad_id: CARD32,
    },
    Name {
        generic: XErrorGeneric,
    },
    Length {
        generic: XErrorGeneric,
    },
    Implmentation {
        generic: XErrorGeneric,
    },
    SocketIO(io::Error),
    Unknown,
}

impl XErrorKind {
    pub fn generic(&self) -> Option<XErrorGeneric> {
        match self {
            &Self::Request { generic, .. } => Some(generic),
            &Self::Value { generic, .. } => Some(generic),
            &Self::Window { generic, .. } => Some(generic),
            &Self::Pixmap { generic, .. } => Some(generic),
            &Self::Atom { generic, .. } => Some(generic),
            &Self::Cursor { generic, .. } => Some(generic),
            &Self::Font { generic, .. } => Some(generic),
            &Self::Match { generic, .. } => Some(generic),
            &Self::Drawable { generic, .. } => Some(generic),
            &Self::Access { generic, .. } => Some(generic),
            &Self::Alloc { generic, .. } => Some(generic),
            &Self::ColorMap { generic, .. } => Some(generic),
            &Self::GContext { generic, .. } => Some(generic),
            &Self::IDChoice { generic, .. } => Some(generic),
            &Self::Name { generic, .. } => Some(generic),
            &Self::Length { generic, .. } => Some(generic),
            &Self::Implmentation { generic, .. } => Some(generic),
            _ => None,
        }
    }
    pub fn from_header(header: XErrorHeader) -> Result<(), Self> {
        if header.error != 0 {
            // header.error should always be zero
            // therefore incoming data is not an error
            return Ok(());
        }
        
        let generic = XErrorGeneric {
            sequence_number: header.sequence_number,
            major: header.major,
            minor: header.minor,
        };

        let bad_id = header.bad_id_or_value;
        
        let err_kind = match header.code {
            01 => XErrorKind::Request { generic },
            02 => XErrorKind::Value {
                generic,
                bad_val: bad_id,
            },
            03 => XErrorKind::Window { generic, bad_id },
            04 => XErrorKind::Pixmap { generic, bad_id },
            05 => XErrorKind::Atom { generic, bad_id },
            06 => XErrorKind::Cursor { generic, bad_id },
            07 => XErrorKind::Font { generic, bad_id },
            08 => XErrorKind::Match { generic },
            09 => XErrorKind::Drawable { generic, bad_id },
            10 => XErrorKind::Access { generic },
            11 => XErrorKind::Alloc { generic },
            12 => XErrorKind::ColorMap { generic, bad_id },
            13 => XErrorKind::GContext { generic, bad_id },
            14 => XErrorKind::IDChoice { generic, bad_id },
            15 => XErrorKind::Name { generic },
            16 => XErrorKind::Length { generic },
            17 => XErrorKind::Implmentation { generic },
            _ => Self::Unknown,
        };
        Err(err_kind)
    }
}
impl From<io::Error> for XErrorKind {
    fn from(ioe: io::Error) -> Self {
        Self::SocketIO(ioe)
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

pub fn check_for_error<S: io::Read>(sock: &mut S) -> Result<(), XErrorKind> {
    match xio::read_primitive::<XErrorHeader, _>(sock) {
        Ok(h) => XErrorKind::from_header(h),
        Err(e) => match e.kind() {
            //ignore timeouts
            io::ErrorKind::WouldBlock => return Ok(()),
            //forward other error
            _ => Err(XErrorKind::SocketIO(e)),
        },
    }
}
