#[cfg(test)]
mod tests;

use std::{
    convert::identity,
    io::{self, SeekFrom},
    mem::size_of,
    slice,
};

macro_rules! _read_impl {
    // Used for i8 and u8, as they are endian independent.
    ($t: ty, $name: literal, $fn: ident) => {
        #[inline(always)]
        #[doc = "Reads "] #[doc = $name] #[doc = "from the underlying reader."]
        fn $fn(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_ne_bytes(_read_impl_body!(self, $t)))
        }
    };

    ($t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        _read_impl!($t, identity, $t, $name, $le, $be, $ne);
    };

    ($read_t: ty, $map: expr, $ret_t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        #[inline(always)]
        #[doc = "Reads "] #[doc = $name] #[doc = "(little-endian) from the underlying reader."]
        fn $le(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_le_bytes(_read_impl_body!(self, $read_t))))
        }

        #[inline(always)]
        #[doc = "Reads "] #[doc = $name] #[doc = "(big-endian) from the underlying reader."]
        fn $be(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_be_bytes(_read_impl_body!(self, $read_t))))
        }

        #[inline(always)]
        #[doc = "Reads "] #[doc = $name] #[doc = "(native-endian) from the underlying reader."]
        fn $ne(&mut self) -> io::Result<$ret_t> {
            Ok($map(<$read_t>::from_ne_bytes(_read_impl_body!(self, $read_t))))
        }
    };
}

macro_rules! _read_impl_body {
    ($context: expr, $t: ty) => {{
        let mut buf = [0u8; size_of::<$t>()];
        $context.read_exact(&mut buf)?;
        buf
    }};
}

macro_rules! _write_impl {
    // Used for i8 and u8, as they are endian independent.
    ($t: ty, $name: literal, $fn: ident) => {
        #[inline(always)]
        #[doc = "Writes "] #[doc = $name] #[doc = "to the underlying writer."]
        fn $fn(&mut self, val: $t) -> io::Result<usize> {
            self.write(&val.to_ne_bytes())
        }
    };

    ($t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        _write_impl!($t, identity, $name, $le, $be, $ne);
    };

    ($t: ty, $map: expr, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        #[inline(always)]
        #[doc = "Writes "] #[doc = $name]
        #[doc = "in little-endian format to the underlying writer."]
        fn $le(&mut self, val: $t) -> io::Result<usize> {
            self.write(&$map(val).to_le_bytes())
        }

        #[inline(always)]
        #[doc = "Writes "] #[doc = $name]
        #[doc = "in big-endian format to the underlying writer."]
        fn $be(&mut self, val: $t) -> io::Result<usize> {
            self.write(&$map(val).to_be_bytes())
        }

        #[inline(always)]
        #[doc = "Writes "] #[doc = $name]
        #[doc = "in native-endian format to the underlying writer."]
        fn $ne(&mut self, val: $t) -> io::Result<usize> {
            self.write(&$map(val).to_ne_bytes())
        }
    };
}

/// Provides methods for reading primitive numbers
/// (except `isize` and `usize` as their size is platform dependent).
#[rustfmt::skip]
pub trait ReadPrimitives: io::Read {
    _read_impl!(i8, "an `i8`", read_i8);
    _read_impl!(u8, "a `u8`", read_u8);
    _read_impl!(i8, "an `i8`", read_i8_le, read_i8_be, read_i8_ne);
    _read_impl!(u8, "a `u8`", read_u8_le, read_u8_be, read_u8_ne);
    _read_impl!(i16, "an `i16`", read_i16_le, read_i16_be, read_i16_ne);
    _read_impl!(u16, "a `u16`", read_u16_le, read_u16_be, read_u16_ne);
    _read_impl!(i32, "an `i32`", read_i32_le, read_i32_be, read_i32_ne);
    _read_impl!(u32, "a `u32`", read_u32_le, read_u32_be, read_u32_ne);
    _read_impl!(i64, "an `i64`", read_i64_le, read_i64_be, read_i64_ne);
    _read_impl!(u64, "a `u64`", read_u64_le, read_u64_be, read_u64_ne);
    _read_impl!(i128, "an `i128`", read_i128_le, read_i128_be, read_i128_ne);
    _read_impl!(u128, "a `u128`", read_u128_le, read_u128_be, read_u128_ne);
    _read_impl!(u32, |x| f32::from_bits(x), f32, "an `f32`", read_f32_le, read_f32_be, read_f32_ne);
    _read_impl!(u64, |x| f64::from_bits(x), f64, "an `f64`", read_f64_le, read_f64_be, read_f64_ne);
}

impl<R> ReadPrimitives for R where R: io::Read {}

/// Provides methods for writing primitive numbers
/// (except `isize` and `usize` as their size is platform dependent).
/// 
/// All functions return bytes written, as all `io::Write` functions do.
#[rustfmt::skip]
pub trait WritePrimitives: io::Write {    
    _write_impl!(i8, "an `i8`", write_i8);
    _write_impl!(u8, "a `u8`", write_u8);
    _write_impl!(i8, "an `i8`", write_i8_le, write_i8_be, write_i8_ne);
    _write_impl!(u8, "a `u8`", write_u8_le, write_u8_be, write_u8_ne);
    _write_impl!(i16, "an `i16`", write_i16_le, write_i16_be, write_i16_ne);
    _write_impl!(u16, "a `u16`", write_u16_le, write_u16_be, write_u16_ne);
    _write_impl!(i32, "an `i32`", write_i32_le, write_i32_be, write_i32_ne);
    _write_impl!(u32, "a `u32`", write_u32_le, write_u32_be, write_u32_ne);
    _write_impl!(i64, "an `i64`", write_i64_le, write_i64_be, write_i64_ne);
    _write_impl!(u64, "a `u64`", write_u64_le, write_u64_be, write_u64_ne);
    _write_impl!(i128, "an `i128`", write_i128_le, write_i128_be, write_i128_ne);
    _write_impl!(u128, "a `u128`", write_u128_le, write_u128_be, write_u128_ne);
    _write_impl!(f32, |x: f32| x.to_bits(), "an `f32`", write_f32_le, write_f32_be, write_f32_ne);
    _write_impl!(f64, |x: f64| x.to_bits(), "an `f64`", write_f64_le, write_f64_be, write_f64_ne);
}

impl<W> WritePrimitives for W where W: io::Write {}

macro_rules! _null_chunk {
    ($rdr: expr, $max: expr) => {{
        let mut length = 0usize;
while $rdr.read_u8()? != 0 {
    length += 1;
    if let Some(max) = $max {
        if max >= length {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
    }
}
$rdr.seek(SeekFrom::Current(-(length as i64 + 1)))?;

let mut buf = vec![0u8; length];
$rdr.read_exact(&mut buf[..])?;
Ok(buf)
    }};
}

/// Provides methods for reading strings of various encodings.
pub trait ReadStrings: io::Read {
    /// Reads a UTF-8 encoded string from the underlying reader with a given length (in bytes).
    fn read_str_utf8(
        &mut self,
        len: usize,
    ) -> io::Result<Result<String, std::string::FromUtf8Error>> {
        Ok(String::from_utf8({
            let mut buf = vec![0u8; len];
            self.read_exact(&mut buf[..])?;
            buf
        }))
    }

    /// Reads a UTF-8 encoded string from the underlying reader with a given length (in bytes).
    /// The validity of the UTF-8 is not checked, therefore this is unsafe.
    unsafe fn read_str_utf8_unchecked(&mut self, len: usize) -> io::Result<String> {
        Ok(String::from_utf8_unchecked({
            let mut buf = vec![0u8; len];
            self.read_exact(&mut buf[..])?;
            buf
        }))
    }

    /// Reads a UTF-8 encoded string from the underlying reader with a given length (in bytes).
    /// If any invalid UTF-8 sequences are present,
    /// they are replaced with U+FFFD REPLACEMENT CHARACTER,
    /// which looks like this: �
    fn read_str_utf8_lossy(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    /// Reads a UTF-16 encoded string from the underlying reader with a given length
    /// (count of 16-bit integers, not of bytes or characters).
    ///
    /// # Panics
    /// Panics if `len * 2` overflows usize.
    #[inline(always)]
    fn read_str_utf16(
        &mut self,
        len: usize,
    ) -> io::Result<Result<String, std::string::FromUtf16Error>> {
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
    #[inline(always)]
    fn read_str_utf16_lossy(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0u8; len.checked_mul(2).expect("input length overflows usize")];
        self.read_exact(&mut buf[..])?;
        Ok(String::from_utf16_lossy(unsafe {
            slice::from_raw_parts(buf.as_ptr() as *const _, len)
        }))
    }

    /// Reads a UTF-8 encoded, null-terminated string from the underlying reader
    /// with an unknown length. Stops reading at the first null terminator.
    ///
    /// Providing `max` will make the reading halt after reading that many bytes without
    /// finding a null terminator, as a safety measure.
    /// It will return io::ErrorKind::UnexpectedEof.
    ///
    /// Providing a `size_hint` will speed up the reading slightly, especially on larger strings.
    fn read_cstr_utf8(
        &mut self,
        max: Option<usize>,
        size_hint: Option<usize>,
    ) -> io::Result<Result<String, std::string::FromUtf8Error>> {
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

    /// Reads a UTF-8 encoded, null-terminated string from the underlying reader
    /// with an unknown length. Stops reading at the first null terminator.
    ///
    /// If any invalid UTF-8 sequences are present, they are replaced
    /// with U+FFFD REPLACEMENT CHARACTER, which looks like this: �
    ///
    /// Providing `max` will make the reading halt after reading that many bytes without
    /// finding a null terminator, as a safety measure.
    /// It will return io::ErrorKind::UnexpectedEof.
    ///
    /// Providing a `size_hint` will speed up the reading slightly, especially on larger strings.
    fn read_cstr_utf8_lossy(
        &mut self,
        max: Option<usize>,
        size_hint: Option<usize>,
    ) -> io::Result<String> {
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
                break Ok(String::from_utf8_lossy(&buf).into_owned());
            }
        }
    }

    fn read_cstr_utf8_fast(
        &mut self,
        max: Option<usize>,
    ) -> io::Result<Result<String, std::string::FromUtf8Error>>
    where
        Self: ReadPrimitives + io::Seek,
    {
        let chunk: io::Result<Vec<u8>> = _null_chunk!(self, max);
        Ok(String::from_utf8(chunk?))
    }

    fn read_cstr_utf8_lossy_fast(
        &mut self,
        max: Option<usize>,
    ) -> io::Result<String>
    where
        Self: ReadPrimitives + io::Seek,
    {
        let chunk: io::Result<Vec<u8>> = _null_chunk!(self, max);
        let chunk = chunk?;
        Ok(String::from_utf8_lossy(&chunk).into_owned())
    }
}

impl<R> ReadStrings for R where R: io::Read {}
