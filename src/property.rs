use super::*; 

#[derive(Copy, Clone)]
pub enum PropertyMode {
    Replace = 0,
    Prepend = 1,
    Append = 2,
}

#[derive(Copy, Clone)]
pub enum PropertyFormat {
    Bytes = 8,
    Shorts = 16,
    Word = 32,
}
pub fn change_property<S: io::Write, T: AsRef<[u8]>>(
    socket: &mut S,
    mode: PropertyMode,
    property: Atom,
    ptype: Atom,
    window_id: CARD32,
    format: PropertyFormat,
    data: T,
) -> io::Result<()> {
    let mode = mode as u8;
    let data = data.as_ref();
    let padding = (4 - data.len() as u16 % 4) % 4;
    let request_len = 6 + (data.len() as u16 + padding) / 4;
    let format_log = ((format as u32) - 1).count_ones();
    let divisor = format_log - 3;
    /*
       format|exp form| fract| rshift
       -----------------------------
       8     | 2^3    | 1/1  |  >> 0
       16    | 2^4    | 1/2  |  >> 1
       32    | 2^5    | 1/4  |  >> 2

       divisor = 2^(log_2(format)-3)
       fract = 1/divisor
       rshift = divisor
    */
    let length_of_data_in_format_units = data.len() as u32 >> divisor;
    write_primitive(socket, opcodes::CHANGE_PROPERTY)?;
    write_primitive(socket, mode)?;
    write_primitive(socket, request_len)?;
    write_primitive(socket, window_id)?;
    write_primitive(socket, property)?;
    write_primitive(socket, ptype)?;
    write_primitive(socket, format as u8)?;
    write_primitive(socket, &[0u8; 3][..])?; //unused
    write_primitive(socket, length_of_data_in_format_units)?;
    socket.write_all(data)?;
    write_padding(data.len(), socket)?;
    Ok(())
}
