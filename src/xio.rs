use super::*;

use std::io::{self, Read, Write};

mod hasbytes;

pub use hasbytes::*;

pub fn write_primitive<T, Writable: Write>(file: &mut Writable, prim: T) -> io::Result<()>
where
    T: HasBytes,
{
    let mut buffer = [0u8; 16];
    file.write_all(prim.as_bytes_le(&mut buffer))
}

pub fn write_primitive_alt<T, Writable: Write>(file: &mut Writable, prim: T) -> io::Result<()>
where
    T: HasBytes,
{
    let mut buffer = [0u8; 16];
    file.write_all(prim.as_bytes_le(&mut buffer))
}

pub fn read_primitive<T, Readable>(file: &mut Readable) -> Result<T, std::io::Error>
where
    T: Copy + Default + Sized,
    Readable: Read,
{
    let mut data = T::default();
    let bytes_to_read: usize = std::mem::size_of::<T>();
    let data_as_slice =
        unsafe { std::slice::from_raw_parts_mut((&mut data) as *mut T as *mut u8, bytes_to_read) };
    file.read_exact(data_as_slice)?;
    Ok(data)
}

pub fn read_primitive_list<Prim, Data: Read>(
    socket: &mut Data,
    len: usize,
) -> Result<Vec<Prim>, std::io::Error>
where
    Prim: Copy + Default + Sized,
{
    (0..len).map(|_| read_primitive(socket)).collect()
}

pub fn read_ascii_string<Data: Read>(
    mut socket: Data,
    string_len: usize,
) -> Result<String, std::io::Error> {
    (0..string_len)
        .map(|_| read_primitive::<CARD8, _>(&mut socket).map(|a| a as char))
        .collect()
}

pub fn write_padding<T: Write>(n: usize, out: &mut T) -> Result<usize, std::io::Error> {
    let padding = (4 - n % 4) % 4;
    for _ in 0..padding {
        out.write(&[0])?;
    }
    Ok(padding)
}

/// returns padding on OK
pub fn read_padding<T: Read>(mut out: T, n: usize) -> Result<usize, std::io::Error> {
    let padding = (4 - n % 4) % 4;
    let mut byte = [0u8];
    for _ in 0..padding {
        out.read(&mut byte)?;
    }
    Ok(padding)
}

pub fn flush_read<T: Read>(mut file: T) -> Result<(), std::io::Error> {
    let mut buffer = [0u8; 31];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read < buffer.len() || bytes_read == 0 {
            break;
        }
    }
    Ok(())
}

#[test]
pub fn read_test() {
    let mut buffer = [0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd].as_slice();
    assert_eq!(read_primitive::<u32, _>(&mut buffer).unwrap(), 0x44332211);
    assert_eq!(read_primitive::<u32, _>(&mut buffer).unwrap(), 0xddccbbaa);

    assert_eq!(
        read_primitive::<u32, _>(&mut buffer).ok(),
        None,
        "buffer out of bytes so io error is expected"
    );
}
