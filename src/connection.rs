use super::*;
use std::io::{Read, Write};

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

pub type ConnectionResult<T> = Result<T, ConnectionErr>;

#[derive(Clone, Default)]
pub struct ConnectionAcceptedInfo {
    pub unused_1: CARD8,
    pub major: CARD16,
    pub minor: CARD16,
    pub additional_data: CARD16,
    pub release_number: CARD32,
    pub resource_id_base: CARD32,
    pub resource_id_mask: CARD32,
    pub motion_buffer_size: CARD32,
    pub vendor_len: CARD16,
    pub max_req_len: CARD16,
    pub number_of_screens: CARD8,
    pub number_for_formats_in_pixmap_formats: CARD8,
    pub image_byte_order: CARD8,
    pub bitmap_format_bit_order: CARD8,
    pub bitmap_format_scaline_unit: CARD8,
    pub bitmap_format_scaline_pad: CARD8,
    pub min_keycode: CARD8,
    pub max_keycode: CARD8,
    pub unused_2: CARD32,
    pub vendor: String,
    pub formats: Vec<XFormat>,
    pub list_of_screen: Vec<XScreen>,
}
impl ConnectionAcceptedInfo {
    pub fn from_socket<S: Read>(mut socket: S) -> Result<Self, std::io::Error> {
        let mut res = Self::default();
        res.unused_1 = xio::read_primitive(&mut socket)?;
        res.major = xio::read_primitive(&mut socket)?;
        res.minor = xio::read_primitive(&mut socket)?;
        res.additional_data = xio::read_primitive(&mut socket)?;
        res.release_number = xio::read_primitive(&mut socket)?;
        res.resource_id_base = xio::read_primitive(&mut socket)?;
        res.resource_id_mask = xio::read_primitive(&mut socket)?;
        res.motion_buffer_size = xio::read_primitive(&mut socket)?;
        res.vendor_len = xio::read_primitive(&mut socket)?;
        res.max_req_len = xio::read_primitive(&mut socket)?;
        res.number_of_screens = xio::read_primitive(&mut socket)?;
        res.number_for_formats_in_pixmap_formats = xio::read_primitive(&mut socket)?;
        res.image_byte_order = xio::read_primitive(&mut socket)?;
        res.bitmap_format_bit_order = xio::read_primitive(&mut socket)?;
        res.bitmap_format_scaline_unit = xio::read_primitive(&mut socket)?;
        res.bitmap_format_scaline_pad = xio::read_primitive(&mut socket)?;
        res.min_keycode = xio::read_primitive(&mut socket)?;
        res.max_keycode = xio::read_primitive(&mut socket)?;
        res.unused_2 = xio::read_primitive(&mut socket)?;
        res.vendor = xio::read_ascii_string(&mut socket, res.vendor_len as usize)?;
        let _padding = xio::read_padding(&mut socket, res.vendor_len as usize)?;
        res.formats = xio::read_primitive_list(
            &mut socket,
            res.number_for_formats_in_pixmap_formats as usize,
        )?;

        // let number_of_bytes_in_screen = 4
        //     * (res.additional_data as usize
        //         - 8
        //         - 2 * res.number_for_formats_in_pixmap_formats as usize)
        //     - res.vendor_len as usize;

        let screen = XScreen::from_socket(
            &mut socket,
            res.number_for_formats_in_pixmap_formats as usize,
        )?;
        res.list_of_screen = vec![screen];

        //i've read everything I need so I  to read everything to the end to make sure the server KNOWS i've read everything.
        //I guess this is kind of like a flush()
        xio::flush_read(&mut socket)?;

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
            self.number_for_formats_in_pixmap_formats
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
        writeln!(f, "[")?;
        for screen in &self.list_of_screen {
            writeln!(f, "{:?}", screen)?;
        }
        writeln!(f, "]")?;
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

pub enum AuthProtocol<'a> {
    Custom {
        auth_protocol_name: &'a str,
        auth_protocol_data: &'a str,
    },
    None,
}
impl<'a> AuthProtocol<'a> {
    pub fn name(&self) -> &'a [u8] {
        if let &Self::Custom {
            auth_protocol_name, ..
        } = self
        {
            auth_protocol_name.as_ref()
        } else {
            &[][..]
        }
    }
    pub fn data(&self) -> &'a [u8] {
        if let &Self::Custom {
            auth_protocol_data, ..
        } = self
        {
            auth_protocol_data.as_ref()
        } else {
            &[][..]
        }
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
    pub fn new(order: ByteOrder, major: u16, minor: u16, protocol_kind: AuthProtocol<'a>) -> Self {
        Self {
            order,
            major: major.into(),
            minor: minor.into(),
            protocol_name: protocol_kind.name(),
            protocol_data: protocol_kind.data(),
        }
    }
    pub fn connect<T: Write + Read>(self, mut sock: T) -> ConnectionResult<XContext<T>> {
        self.write_to(&mut sock)?;
        let resp = self.read_response(&mut sock)?;
        Ok(XContext{
            id_count:0,
            socket:sock,
            info:resp 
        })
    }
    fn read_response<T: Read>(
        &self,
        mut socket: T,
    ) -> Result<ConnectionAcceptedInfo, ConnectionErr> {
        let connection_status = ConnectionStatus::from_code(xio::read_primitive(&mut socket)?);

        match connection_status {
            ConnectionStatus::Failed => {
                let length_of_reason = xio::read_primitive::<CARD8, _>(&mut socket)?;
                let major = xio::read_primitive::<CARD16, _>(&mut socket)?;
                let minor = xio::read_primitive::<CARD16, _>(&mut socket)?;
                let _addition_data_len_4b = xio::read_primitive::<CARD16, _>(&mut socket)?;
                let reason = xio::read_ascii_string(&mut socket, length_of_reason as usize)?;
                xio::read_padding(socket, length_of_reason as usize)?;

                Err(ConnectionErr::ConnectionRefused(RefusedInfo {
                    reason,
                    major,
                    minor,
                }))
            }
            ConnectionStatus::Accepted => {
                let info = ConnectionAcceptedInfo::from_socket(&mut socket)?;
                println!("{:?}", info);
                Ok(info)
            }
            ConnectionStatus::AuthenticationNeeded => {
                unimplemented!("auth not implemented")
            }
        }
    }

    fn write_to<T: Write>(&self, mut out: T) -> Result<(), std::io::Error> {
        out.write([self.order as u8, 0].as_slice())?;
        xio::write_primitive(&mut out, self.major)?;
        xio::write_primitive(&mut out, self.minor)?;
        xio::write_primitive(&mut out, self.protocol_name.len())?;
        xio::write_primitive(&mut out, self.protocol_data.len())?;
        xio::write_primitive(&mut out, 0u16)?; //unused
        xio::write_primitive(&mut out, self.protocol_name)?;
        xio::write_padding(self.protocol_name.len(), &mut out)?;
        xio::write_primitive(&mut out, self.protocol_data)?;
        xio::write_padding(self.protocol_data.len(), &mut out)?;
        Ok(())
    }
}
