[![Build Status](https://travis-ci.com/notviri/minio.svg?branch=master)](https://travis-ci.com/notviri/minio)
# minio
Minimal `std::io` extension for reducing boilerplate while `Read`ing and `Write`ing primitives & strings.

## Usage
In some examples, `.unwrap()` is used, obviously in production code you wouldn't want to unless you actually want it to panic on failure (even then ``called `Result::unwrap()` on an `Err` value`` is not exactly helpful).
### Reading
```rust
use minio::{ReadPrimitives, ReadStrings};
use std::io;

fn main() -> io::Result<()> {
    let mut reader = imaginary_reader(); // anything implementing io::Read
    let a_little_endian_i32 = reader.read_i32_le()?;
    
    // size-types not provided because of byte length inconsistency across architectures
    let length = reader.read_u32_le()? as usize;
    let a_wide_string = reader.read_string_utf16(length)?.unwrap();
    
    // ffi strings are supported as well
    // here we read a max of 1024 bytes with an unknown final size
    let a_cstring = reader.read_cstring_utf8(Some(1024), None)?.unwrap();

    Ok(())
}
```
### Writing
```rust
use minio::{WritePrimitives, /* WriteStrings */};
use std::io;

fn main() -> io::Result<()> {
    let mut writer = imaginary_writer(); // anything implementing io::Write
    writer.write_i32_le( ... );
    Ok(())
}
```
