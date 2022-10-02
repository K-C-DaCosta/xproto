use crate::Atom;

pub trait HasBytes {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8];
    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8];
}

impl HasBytes for Atom {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.0.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.0.to_be_bytes(), buffer)
    }
}

impl HasBytes for &[u8] {
    //NOTE: This is NOT implemented correctly but im just gonna keep it like this for now
    fn as_bytes_le<'a>(&self, _buffer: &'a mut [u8; 16]) -> &'a [u8] {
        let slice_ptr = (*self) as *const [u8];
        unsafe { &*slice_ptr }
    }

    fn as_bytes_be<'a>(&self, _buffer: &'a mut [u8; 16]) -> &'a [u8] {
        let slice_ptr = (*self) as *const [u8];
        unsafe { &*slice_ptr }
    }
}

impl HasBytes for i128 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for i64 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}
impl HasBytes for i32 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}
impl HasBytes for i16 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}
impl HasBytes for i8 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for u128 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for u64 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for u32 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for u16 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for u8 {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

impl HasBytes for usize {
    fn as_bytes_le<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_le_bytes(), buffer)
    }

    fn as_bytes_be<'a>(&self, buffer: &'a mut [u8; 16]) -> &'a [u8] {
        copy_array_then_slice(self.to_be_bytes(), buffer)
    }
}

fn copy_array_then_slice<const N: usize>(data: [u8; N], buffer: &mut [u8; 16]) -> &[u8] {
    let len = data.len();
    buffer
        .iter_mut()
        .zip(data.iter())
        .for_each(|(out, &dat)| *out = dat);
    &buffer[0..len]
}

