use std::{fmt::Write, num::ParseIntError};
use std::iter::Iterator;
use base64::{engine, Engine};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn base64_encode(bytes: &[u8], alphabet: base64::alphabet::Alphabet) -> String {
    let config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

    let new_engine = engine::GeneralPurpose::new(&alphabet, config);
    new_engine.encode(bytes)
}