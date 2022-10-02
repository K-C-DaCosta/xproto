use std::{
    fmt::Debug,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use xproto::{io::*, *};

#[derive(Copy, Clone)]
pub enum ByteOrder {
    LittleEndian = b'l' as isize,
    BigEndian = b'b' as isize,
}

impl Into<u8> for ByteOrder {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Failed = 0,
    Accepted = 1,
    AuthenticationNeeded = 2,
}
impl ConnectionStatus {
    pub fn from_code(status_code: u8) -> Self {
        match status_code {
            0 => ConnectionStatus::Failed,
            1 => ConnectionStatus::Accepted,
            2 => ConnectionStatus::AuthenticationNeeded,
            _ => panic!("invalid status code, possible IO bug"),
        }
    }
}

type ConnectionResult<T> = Result<T, ConnectionErr>;

#[derive(Clone, Default)]
pub struct ConnectionAcceptedInfo {
    unused_1: CARD8,
    major: CARD16,
    minor: CARD16,
    additional_data: CARD16,
    release_number: CARD32,
    resource_id_base: CARD32,
    resource_id_mask: CARD32,
    motion_buffer_size: CARD32,
    vendor_len: CARD16,
    max_req_len: CARD16,
    number_of_screens: CARD8,
    number_for_formats_in_pixmap_fonts: CARD8,
    image_byte_order: CARD8,
    bitmap_format_bit_order: CARD8,
    bitmap_format_scaline_unit: CARD8,
    bitmap_format_scaline_pad: CARD8,
    min_keycode: CARD8,
    max_keycode: CARD8,
    unused_2: CARD32,
    vendor: String,
    formats: Vec<XFormat>,
    list_of_screen: Vec<XScreen>,
}
impl ConnectionAcceptedInfo {
    pub fn from_socket<S: Read>(mut socket: S) -> Result<Self, std::io::Error> {
        let mut res = Self::default();
        res.unused_1 = read_primitive(&mut socket)?;
        res.major = read_primitive(&mut socket)?;
        res.minor = read_primitive(&mut socket)?;
        res.additional_data = read_primitive(&mut socket)?;
        res.release_number = read_primitive(&mut socket)?;
        res.resource_id_base = read_primitive(&mut socket)?;
        res.resource_id_mask = read_primitive(&mut socket)?;
        res.motion_buffer_size = read_primitive(&mut socket)?;
        res.vendor_len = read_primitive(&mut socket)?;
        res.max_req_len = read_primitive(&mut socket)?;
        res.number_of_screens = read_primitive(&mut socket)?;
        res.number_for_formats_in_pixmap_fonts = read_primitive(&mut socket)?;
        res.image_byte_order = read_primitive(&mut socket)?;
        res.bitmap_format_bit_order = read_primitive(&mut socket)?;
        res.bitmap_format_scaline_unit = read_primitive(&mut socket)?;
        res.bitmap_format_scaline_pad = read_primitive(&mut socket)?;
        res.min_keycode = read_primitive(&mut socket)?;
        res.max_keycode = read_primitive(&mut socket)?;
        res.unused_2 = read_primitive(&mut socket)?;
        res.vendor = read_ascii_string(&mut socket, res.vendor_len as usize)?;
        let p = read_padding(&mut socket, res.vendor_len as usize)?;
        res.formats =
            read_primitive_list(&mut socket, res.number_for_formats_in_pixmap_fonts as usize)?;

        let screen =
            XScreen::from_socket(&mut socket, res.number_for_formats_in_pixmap_fonts as usize)?;
        res.list_of_screen = vec![screen];


        Ok(res)
    }
}

impl Debug for ConnectionAcceptedInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "unused = {:?}", self.unused_1)?;
        writeln!(f, "major = {:?}", self.major)?;
        writeln!(f, "minor = {:?}", self.minor)?;
        writeln!(f, "additional_data = {:?}", self.additional_data)?;
        writeln!(f, "release_number = {:?}", self.release_number)?;
        writeln!(f, "resource_id_base = {:?}", self.resource_id_base)?;
        writeln!(f, "resource_id_mask = {:?}", self.resource_id_mask)?;
        writeln!(f, "motion_buffer_size = {:?}", self.motion_buffer_size)?;
        writeln!(f, "vendor_len = {:?}", self.vendor_len)?;
        writeln!(f, "max_req_len = {:?}", self.max_req_len)?;
        writeln!(f, "number_of_screens = {:?}", self.number_of_screens)?;
        writeln!(
            f,
            "number_for_formats_in_pixmap_fonts = {:?}",
            self.number_for_formats_in_pixmap_fonts
        )?;
        writeln!(f, "image_byte_order = {:?}", self.image_byte_order)?;
        writeln!(
            f,
            "bitmap_format_bit_order = {:?}",
            self.bitmap_format_bit_order
        )?;
        writeln!(
            f,
            "bitmap_format_scaline_unit = {:?}",
            self.bitmap_format_scaline_unit
        )?;
        writeln!(
            f,
            "bitmap_format_scaline_pad = {:?}",
            self.bitmap_format_scaline_pad
        )?;
        writeln!(f, "min_keycode = {:?}", self.min_keycode)?;
        writeln!(f, "max_keycode = {:?}", self.max_keycode)?;
        writeln!(f, "unused_2 = {:?}", self.unused_2)?;
        writeln!(f, "vendor = {:?}", self.vendor)?;

        for format in &self.formats {
            writeln!(f, "format = {:?}", format)?;
        }
        writeln!(f,"[")?;
        for screen in &self.list_of_screen{
            writeln!(f,"{:?}",screen)?;
        }
        writeln!(f,"]")?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConnectionErr {
    ConnectionRefused(RefusedInfo),
    SocketErr(std::io::Error),
}

impl From<std::io::Error> for ConnectionErr {
    fn from(io_err: std::io::Error) -> Self {
        Self::SocketErr(io_err)
    }
}

#[derive(Debug, Default)]
pub struct RefusedInfo {
    reason: String,
    major: CARD16,
    minor: CARD16,
}

#[derive(Default)]
pub struct ConnectionResponse {
    reason_failed_length: u8,
    protocol_major: CARD16,
    protocol_minor: CARD16,
    additional_data: CARD16,
    reason: String,
}

impl ConnectionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct RequestConnection<'a> {
    order: ByteOrder,
    major: CARD16,
    minor: CARD16,
    protocol_name: &'a [u8],
    protocol_data: &'a [u8],
}
impl<'a> RequestConnection<'a> {
    pub fn new<'b, T: AsRef<[u8]>>(
        order: ByteOrder,
        major: u16,
        minor: u16,
        protocol_name: &'b T,
        protocol_data: &'b T,
    ) -> Self
    where
        'b: 'a,
    {
        Self {
            order,
            major: major.into(),
            minor: minor.into(),
            protocol_name: protocol_name.as_ref(),
            protocol_data: protocol_data.as_ref(),
        }
    }
    pub fn make_connection<T: Write + Read>(&self, mut sock: T) -> ConnectionResult<()> {
        self.write_to(&mut sock)?;

        let resp = self.read_response(&mut sock)?;

        Ok(())
    }
    fn read_response<T: Read>(&self, mut socket: T) -> Result<(), ConnectionErr> {
        let connection_status =
            ConnectionStatus::from_code(read_primitive::<CARD8, _>(&mut socket)?);

        match connection_status {
            ConnectionStatus::Failed => {
                let length_of_reason = read_primitive::<CARD8, _>(&mut socket)?;
                let major = read_primitive::<CARD16, _>(&mut socket)?;
                let minor = read_primitive::<CARD16, _>(&mut socket)?;
                let _addition_data_len_4b = read_primitive::<CARD16, _>(&mut socket)?;
                let mut reason = String::with_capacity(length_of_reason as usize);
                for _ in 0..length_of_reason {
                    reason.push(read_primitive::<CARD8, _>(&mut socket)? as char);
                }
                read_padding(socket, length_of_reason as usize)?;

                Err(ConnectionErr::ConnectionRefused(RefusedInfo {
                    reason,
                    major,
                    minor,
                }))
            }
            ConnectionStatus::Accepted => {
                let info = ConnectionAcceptedInfo::from_socket(&mut socket)?;
                println!("{:?}", info);
                Ok(())
            }
            ConnectionStatus::AuthenticationNeeded => {
                unimplemented!("auth not implemented")
            }
        }
    }

    fn write_to<T: Write>(&self, mut out: T) -> Result<(), std::io::Error> {
        out.write([self.order as u8, 0].as_slice())?;
        write_primitive(&mut out, self.major)?;
        write_primitive(&mut out, self.minor)?;
        write_primitive(&mut out, self.protocol_name.len())?;
        write_primitive(&mut out, self.protocol_data.len())?;
        write_primitive(&mut out, 0u16)?; //unused
        write_primitive(&mut out, self.protocol_name)?;
        write_padding(self.protocol_name.len(), &mut out)?;
        write_primitive(&mut out, self.protocol_data)?;
        write_padding(self.protocol_data.len(), &mut out)?;
        Ok(())
    }
}

fn main() {
    let mut xserver = UnixStream::connect("/tmp/.X11-unix/X0").expect("cant connect to xserver");

    let req = RequestConnection::new(ByteOrder::LittleEndian, 11, 0, &"", &"");

    req.make_connection(&mut xserver).unwrap();
}
