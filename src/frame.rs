use std::io::{Cursor, Read};
use bytes::Buf;
use eyre::Result;


pub(crate) fn read_u16_sized_string(src: &mut (impl Read + Buf)) -> Result<String> {
    let string_len = src.get_u16_le();
    read_str_with_len(src, string_len as u32)
}

pub(crate) fn read_8_char_sized_string(src: &mut (impl Read + Buf)) -> Result<String> {
    let string_len = 8;
    read_str_with_len(src, string_len)
}

fn read_str_with_len(src: &mut (impl Read + Buf), len: u32) -> Result<String> {
    if len > 0 {
        let mut string = vec![0u8; len as usize];
        src.read_exact(&mut string)?;
        Ok(String::from_utf8(string)?)
    } else {
        Ok("".to_string())
    }
}
