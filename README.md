[![Build Status](https://travis-ci.com/notviri/minio.svg?branch=master)](https://travis-ci.com/notviri/minio)
# minio
Minimal `std::io` extension for `Read`ing & `Write`ing simple data types.

## Usage
### Reading
```rust
use minio::{ReadPrimitives, ReadStrings};
use std::io;

fn main() -> io::Result<()> {
    let mut reader = imaginary_reader(); // anything implementing io::Read
    let a_little_endian_i32 = reader.read_i32_le()?;
    
    // size-types not provided because of inconsistencies across platforms
    let wstr_length = reader.read_u64_le()? as usize;
    let a_wide_string = reader.read_string_utf16(wstr_length);
    
    // ffi strings are supported too
    // here we read a max of 1024 bytes with an unknown final size
    let a_cstring = reader.read_cstring_utf8(Some(1024), None)?.unwrap();

    Ok(())
}
```
