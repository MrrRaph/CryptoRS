/**
   CryptoRS is a toolbox to use multiple cryptographic functions using one library
   Copyright (C) 2023  Raphaël Dray <raphael.dray@proton.me>

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.
*/

mod core;

#[cfg(test)]
mod tests {
    use crate::{
        core,
        core::{
            encoding::base64_encode
        }
    };

    #[test]
    fn from_hex_to_b64() {
        println!("[*] Performing `from_hex_to_b64` test");
        println!("\t[*] Trying to use `decode_hex` function");
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        if let Ok(s) = core::encoding::decode_hex(hex_string) {
            let result = base64_encode(&s, core::alphabet::alphabet::STANDARD);
            assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
            println!("\t[+] OK");
        }
    }
}
