use std::{
    convert::identity, io, mem::size_of, slice, string::FromUtf16Error, string::FromUtf8Error,
};

macro_rules! read_impl {
    ($t: ty, $name: literal, $fn: ident) => {
        #[doc = "Reads a `"] #[doc = $name] #[doc = "` from the underlying reader."]
        fn $fn(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_ne_bytes(read_impl_body!(self, $t)))
        }
    };

    ($t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        read_impl!($t, identity, $t, $name, $le, $be, $ne);
    };

    ($read_t: ty, $map: expr, $ret_t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        #[doc = "Reads a `"] #[doc = $name] #[doc = "` (little-endian) from the underlying reader."]
        fn $le(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_le_bytes(read_impl_body!(self, $read_t))))
        }

        #[doc = "Reads a `"] #[doc = $name] #[doc = "` (big-endian) from the underlying reader."]
        fn $be(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_be_bytes(read_impl_body!(self, $read_t))))
        }

        #[doc = "Reads a `"] #[doc = $name] #[doc = "` (native-endian) from the underlying reader."]
        fn $ne(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_ne_bytes(read_impl_body!(self, $read_t))))
        }
    };
}

macro_rules! read_impl_body {
    ($context: expr, $t: ty) => {{
        let mut buf = [0u8; size_of::<$t>()];
        $context.read_exact(&mut buf)?;
        buf
    }};
}

pub trait ReadPrimitives: io::Read {
    read_impl!(i8, "i8", read_i8);
    read_impl!(u8, "u8", read_u8);
    read_impl!(i8, "i8", read_i8_le, read_i8_be, read_i8_ne);
    read_impl!(u8, "u8", read_u8_le, read_u8_be, read_u8_ne);
    read_impl!(i16, "i16", read_i16_le, read_i16_be, read_i16_ne);
    read_impl!(u16, "u16", read_u16_le, read_u16_be, read_u16_ne);
    read_impl!(i32, "i32", read_i32_le, read_i32_be, read_i32_ne);
    read_impl!(u32, "u32", read_u32_le, read_u32_be, read_u32_ne);
    read_impl!(i64, "i64", read_i64_le, read_i64_be, read_i64_ne);
    read_impl!(u64, "u64", read_u64_le, read_u64_be, read_u64_ne);
    read_impl!(i128, "i128", read_i128_le, read_i128_be, read_i128_ne);
    read_impl!(u128, "u128", read_u128_le, read_u128_be, read_u128_ne);

    #[rustfmt::skip]
    read_impl!(u32, |x| f32::from_bits(x), f32, "f32", read_f32_le, read_f32_be, read_f32_ne);

    #[rustfmt::skip]
    read_impl!(u64, |x| f64::from_bits(x), f64, "f64", read_f64_le, read_f64_be, read_f64_ne);
}

impl<R> ReadPrimitives for R where R: io::Read {}

pub trait ReadStrings: io::Read {
    /// Reads a UTF-8 encoded string from the underlying reader with a given length.
    /// (of bytes, not characters).
    fn read_string_utf8(&mut self, len: usize) -> io::Result<Result<String, FromUtf8Error>> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf8(buf))
    }

    /// Reads a UTF-8 encoded string from the underlying reader with a given length
    /// (of bytes, not characters).
    ///
    /// If any invalid UTF-8 sequences are present, they are replaced
    /// with U+FFFD REPLACEMENT CHARACTER, which looks like this: �
    fn read_string_utf8_lossy(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    /// Reads a UTF-16 encoded string from the underlying reader with a given length
    /// (count of 16-bit integers, not of bytes or characters).
    ///
    /// # Panics
    /// Panics if `len * 2` overflows usize.
    fn read_string_utf16(&mut self, len: usize) -> io::Result<Result<String, FromUtf16Error>> {
        let mut buf = vec![0u8; len.checked_mul(2).expect("input length overflows usize")];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf16(unsafe {
            slice::from_raw_parts(buf.as_ptr() as *const _, len)
        }))
    }

    /// Reads a UTF-16 encoded string from the underlying reader with a given length
    /// (count of 16-bit integers, not of bytes or characters).
    ///
    /// If any invalid UTF-16 sequences are present, they are replaced
    /// with U+FFFD REPLACEMENT CHARACTER, which looks like this: �
    ///
    /// # Panics
    /// Panics if `len * 2` overflows usize.
    fn read_string_utf16_lossy(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0u8; len.checked_mul(2).expect("input length overflows usize")];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf16_lossy(unsafe {
            slice::from_raw_parts(buf.as_ptr() as *const _, len)
        }))
    }

    /// Reads a UTF-8 encoded, null-terminated string from the underlying reader
    /// with an unknown length.
    ///
    /// Stops reading at the first null terminator.
    ///
    /// Providing `max` will make the reading halt after reading that many bytes without
    /// finding a null terminator, as a safety measure.
    /// It will return io::ErrorKind::UnexpectedEof.
    ///
    /// Providing `size_hint` will speed up the reading slightly, especially on larger strings.
    fn read_cstring_utf8(
        &mut self,
        max: Option<usize>,
        size_hint: Option<usize>,
    ) -> io::Result<Result<String, FromUtf8Error>> {
        let mut buf = Vec::with_capacity(size_hint.unwrap_or(0));
        let mut count = 0;
        loop {
            if let Some(max) = max {
                if count > max {
                    break Err(io::ErrorKind::UnexpectedEof.into());
                }
            }

            let mut next = [0u8; 1];
            self.read_exact(&mut next[..])?;
            if next[0] != 0x00 {
                buf.push(next[0]);
                count += 1;
            } else {
                break Ok(String::from_utf8(buf));
            }
        }
    }
}
