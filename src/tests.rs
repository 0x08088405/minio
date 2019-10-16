use crate::{ReadPrimitives, ReadStrings, WritePrimitives};
use std::mem::size_of;

#[test]
#[rustfmt::skip]
fn read_write_primitives() {
    const TEST: &[u8] = &[
        0xCD, 0xCC, 0xDC, 0x40,
        0xE6, 0x73, 0x87, 0xFF,
        0xD7, 0x68, 0x29, 0x22,
        0xD8, 0x88, 0xF3, 0x11,
    ];

    // read
    assert_eq!(205,                                     TEST.read_u8()     .unwrap());
    assert_eq!(205,                                     TEST.read_u8_le()  .unwrap());
    assert_eq!(205,                                     TEST.read_u8_be()  .unwrap());
    assert_eq!(-51,                                     TEST.read_i8()     .unwrap());
    assert_eq!(-51,                                     TEST.read_i8_le()  .unwrap());
    assert_eq!(-51,                                     TEST.read_i8_be()  .unwrap());
    assert_eq!(52429,                                   TEST.read_u16_le() .unwrap());
    assert_eq!(52684,                                   TEST.read_u16_be() .unwrap());
    assert_eq!(-13107,                                  TEST.read_i16_le() .unwrap());
    assert_eq!(-12852,                                  TEST.read_i16_be() .unwrap());
    assert_eq!(1088212173,                              TEST.read_u32_le() .unwrap());
    assert_eq!(3452755008,                              TEST.read_u32_be() .unwrap());
    assert_eq!(1088212173,                              TEST.read_i32_le() .unwrap());
    assert_eq!(-842212288,                              TEST.read_i32_be() .unwrap());
    assert_eq!(18412813034295446733,                    TEST.read_u64_le() .unwrap());
    assert_eq!(14829469844326549503,                    TEST.read_u64_be() .unwrap());
    assert_eq!(-33931039414104883,                      TEST.read_i64_le() .unwrap());
    assert_eq!(-3617274229383002113,                    TEST.read_i64_be() .unwrap());
    assert_eq!(6.9,                                     TEST.read_f32_le() .unwrap());
    assert_eq!(-429623296.0,                            TEST.read_f32_be() .unwrap());
    assert_eq!(23861379596529284064479147122191289549,  TEST.read_u128_le().unwrap());
    assert_eq!(273555434967085284028993218695928541969, TEST.read_u128_be().unwrap());
    assert_eq!(23861379596529284064479147122191289549,  TEST.read_i128_le().unwrap());
    assert_eq!(-66726931953853179434381388735839669487, TEST.read_i128_be().unwrap());

    // write
    let mut buf = vec![];
    buf.write_u8_le  (205                                    ).unwrap(); assert_eq!(&TEST[..size_of::<u8>()], &*buf);   buf.clear();
    buf.write_u8_le  (205                                    ).unwrap(); assert_eq!(&TEST[..size_of::<u8>()], &*buf);   buf.clear();
    buf.write_u8_be  (205                                    ).unwrap(); assert_eq!(&TEST[..size_of::<u8>()], &*buf);   buf.clear();
    buf.write_i8     (-51                                    ).unwrap(); assert_eq!(&TEST[..size_of::<i8>()], &*buf);   buf.clear();
    buf.write_i8_le  (-51                                    ).unwrap(); assert_eq!(&TEST[..size_of::<i8>()], &*buf);   buf.clear();
    buf.write_i8_be  (-51                                    ).unwrap(); assert_eq!(&TEST[..size_of::<i8>()], &*buf);   buf.clear();
    buf.write_u16_le (52429                                  ).unwrap(); assert_eq!(&TEST[..size_of::<u16>()], &*buf);  buf.clear();
    buf.write_u16_be (52684                                  ).unwrap(); assert_eq!(&TEST[..size_of::<u16>()], &*buf);  buf.clear();
    buf.write_i16_le (-13107                                 ).unwrap(); assert_eq!(&TEST[..size_of::<i16>()], &*buf);  buf.clear();
    buf.write_i16_be (-12852                                 ).unwrap(); assert_eq!(&TEST[..size_of::<i16>()], &*buf);  buf.clear();
    buf.write_u32_le (1088212173                             ).unwrap(); assert_eq!(&TEST[..size_of::<u32>()], &*buf);  buf.clear();
    buf.write_u32_be (3452755008                             ).unwrap(); assert_eq!(&TEST[..size_of::<u32>()], &*buf);  buf.clear();
    buf.write_i32_le (1088212173                             ).unwrap(); assert_eq!(&TEST[..size_of::<i32>()], &*buf);  buf.clear();
    buf.write_i32_be (-842212288                             ).unwrap(); assert_eq!(&TEST[..size_of::<i32>()], &*buf);  buf.clear();
    buf.write_u64_le (18412813034295446733                   ).unwrap(); assert_eq!(&TEST[..size_of::<u64>()], &*buf);  buf.clear();
    buf.write_u64_be (14829469844326549503                   ).unwrap(); assert_eq!(&TEST[..size_of::<u64>()], &*buf);  buf.clear();
    buf.write_i64_le (-33931039414104883                     ).unwrap(); assert_eq!(&TEST[..size_of::<i64>()], &*buf);  buf.clear();
    buf.write_i64_be (-3617274229383002113                   ).unwrap(); assert_eq!(&TEST[..size_of::<i64>()], &*buf);  buf.clear();
    buf.write_f32_le (6.9                                    ).unwrap(); assert_eq!(&TEST[..size_of::<f32>()], &*buf);  buf.clear();
    buf.write_f32_be (-429623296.0                           ).unwrap(); assert_eq!(&TEST[..size_of::<f32>()], &*buf);  buf.clear();
    buf.write_u128_le(23861379596529284064479147122191289549 ).unwrap(); assert_eq!(&TEST[..size_of::<u128>()], &*buf); buf.clear();
    buf.write_u128_be(273555434967085284028993218695928541969).unwrap(); assert_eq!(&TEST[..size_of::<u128>()], &*buf); buf.clear();
    buf.write_i128_le(23861379596529284064479147122191289549 ).unwrap(); assert_eq!(&TEST[..size_of::<i128>()], &*buf); buf.clear();
    buf.write_i128_be(-66726931953853179434381388735839669487).unwrap(); assert_eq!(&TEST[..size_of::<i128>()], &*buf); buf.clear();
}

#[test]
fn read_write_strings() {
    use std::{io::Cursor, slice};

    let test_utf8 = "ℍ𝕖𝕝𝕝𝕠, 𝕨𝕠𝕣𝕝𝕕! 👋";
    let test_utf8_invalid = b"Hello, \x80world!";
    let test_cstring = b"Hello, world!\0";
    let test_cstring_unterminated = b"Hello, world!"; // no null

    // reading
    assert_eq!(
        test_utf8,
        Cursor::new(test_utf8.as_bytes())
            .read_str_utf8(test_utf8.len())
            .unwrap()
            .unwrap()
            .as_str()
    );

    assert!(
        Cursor::new(test_utf8_invalid)
            .read_str_utf8(test_utf8_invalid.len())
            .unwrap()
            .is_err()
    );
    assert_eq!(
        "Hello, �world!",
        Cursor::new(test_utf8_invalid)
            .read_str_utf8_lossy(test_utf8_invalid.len())
            .unwrap()
            .as_str()
    );

    let utf16_bytes = test_utf8
        .encode_utf16()
        .collect::<Vec<_>>()
        .into_boxed_slice();
    assert_eq!(
        test_utf8,
        Cursor::new(unsafe {
            slice::from_raw_parts(utf16_bytes.as_ptr() as *const u8, utf16_bytes.len() * 2)
        })
        .read_str_utf16(utf16_bytes.len())
        .unwrap()
        .unwrap()
        .as_str()
    );

    assert_eq!(
        "Hello, world!",
        Cursor::new(test_cstring)
            .read_cstr_utf8(None)
            .unwrap()
            .unwrap()
            .as_str()
    );

    // max chars = 4, no null found
    assert!(Cursor::new(test_cstring).read_cstr_utf8(Some(4)).is_err());
    assert!(
        Cursor::new(test_cstring_unterminated)
            .read_cstr_utf8(None)
            .is_err()
    );

    // len-1 trims the null here for fair comparison
    assert_eq!(
        &test_cstring[..test_cstring.len() - 1],
        Cursor::new(test_cstring)
            .read_cstr_utf8_fast(None)
            .unwrap()
            .unwrap()
            .as_bytes()
    );

    // writing
    // ...... oh that doesn't exist yet!
}
