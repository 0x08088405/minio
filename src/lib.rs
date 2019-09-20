use std::{io, mem::size_of};

macro_rules! read_impl {
    ($t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (little-endian) from the underlying reader."]
        fn $le(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_le_bytes(read_impl_body!(self, $t)))
        }

        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (big-endian) from the underlying reader."]
        fn $be(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_be_bytes(read_impl_body!(self, $t)))
        }

        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (native-endian) from the underlying reader."]
        fn $ne(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_ne_bytes(read_impl_body!(self, $t)))
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
    read_impl!(u16, "u16", read_u16_le, read_u16_be, read_u16_ne);
}
