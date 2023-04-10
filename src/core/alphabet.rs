pub mod alphabet {
    use base64::alphabet;

    pub const STANDARD: alphabet::Alphabet = alphabet::STANDARD;
    pub const URL_SAFE: alphabet::Alphabet = alphabet::URL_SAFE;
    pub const UNIX_CRYPT: alphabet::Alphabet = alphabet::CRYPT;
    pub const BIN_HEX: alphabet::Alphabet = alphabet::BIN_HEX;

    lazy_static::lazy_static! {
        pub static ref FILENAME_SAFE: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .chain(b'0'..=b'9')
                    .collect::<Vec<u8>>(), vec![b'+', b'-']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref ITOA64: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [vec![b'.', b'/'], (b'0'..=b'9')
                    .chain(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .collect::<Vec<u8>>()].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref XML: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .chain(b'0'..=b'9')
                    .collect::<Vec<u8>>(), vec![b'_', b'.']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref Z64: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'0'..=b'9')
                    .chain(b'a'..=b'z')
                    .chain(b'A'..=b'Z')
                    .collect::<Vec<u8>>(), vec![b'+', b'/']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref Y64: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .chain(b'0'..=b'9')
                    .collect::<Vec<u8>>(), vec![b'.', b'_', b'-']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref RADIX64: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'0'..=b'9')
                    .chain(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .collect::<Vec<u8>>(), vec![b'+', b'/']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref XX_ENCODING: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [vec![b'+', b'-'], (b'0'..=b'9')
                    .chain(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .collect::<Vec<u8>>()].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref ROT13: alphabet::Alphabet = alphabet::Alphabet::new(
            String::from_utf8(
                [(b'N'..=b'Z')
                    .chain(b'A'..=b'M')
                    .chain(b'n'..=b'z')
                    .chain(b'a'..=b'm')
                    .chain(b'0'..=b'9')
                    .collect(), vec![b'+', b'/']].concat()
            ).unwrap().as_str()
        ).unwrap();
        pub static ref UU_ENCODING: alphabet::Alphabet = alphabet::Alphabet::new(" -_").unwrap();
    }
}
