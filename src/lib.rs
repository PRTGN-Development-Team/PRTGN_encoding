//! # Overview
//!
//! `PRTGN_encoding` (listed as `prtgn_encoding` for Cargo.TOML) is a library used by programs compatible with PRTGN files to encode and decode them.
//!
//! Any file can be encoded with PRTGN, though for user convenience it is highly recommended to use the .prtgn extension. What is being written isn't a text file, simply add the original file extension to the end. Such as .prtgn_wav does.
//!
//! Going along with that, anything can be encoded with PRTGN. As long as what is given to the `write` function as a string.
//!     An example of this is used by PRTGN for the wav file. [wav_converter.rs | PRTGN version 0.3.1, added in version 0.3.0](https://github.com/PRTGN-Development-Team/.prtgn/blob/83d6a200cdf14e82b84684480198a63ae40c63da/src/command/prtgn_wav/wav_converter.rs).
//!     `wav_converter.rs` takes a wav file, reads its data, converts the data buffer to a string, and then writes it to a `prtgn_wav` file. The opposite is done for playing the wav file.
//!
//! Simply add PRTGN_encoding to your Cargo.TOML and add the following to your file to access commands depending on what you need!
//!
//! ## Read Only
//! ```Rust
//! use prtgn_encoding::read;
//! ```
//!
//! ## Write Only
//! ```Rust
//! use prtgn_encoding::write;
//! ```
//!
//! ## Read and Write
//! ```Rust
//! use prtgn_encoding::{read, write};
//! ```
//!
//!
//! ---
//! ---
//!
//!
//! # Example File Usage
//!
//!
//! ## Basic Write Example
//! ```commandline
//! cargo run --example basic_write
//! ```
//!
//! ## Basic Read Example
//!
//! **Run the write example first in order to have a correctly encoded PRTGN file with the right name**
//!
//! ```commandline
//! cargo run --example basic_read
//! ```
//!
//! ## Read Write Combo Example
//! ```commandline
//! cargo run --example read_write
//! ```
//!
//!
//! # Development Note
//!
//! To test documentation, run the following.
//! ```commandline
//! cargo doc --open
//! ```

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::io::SeekFrom;
use std::string::FromUtf8Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_test() {
        println!("Hello from the PRTGN Development Team");

        let text = "Hello from PRTGN Encoding!".to_string();
        let og_txt = text.clone();
        write("test.prtgn".to_string(), text).unwrap();

        let decoded = read("test.prtgn".to_string()).unwrap().to_string();

        println!("Original : {:?}", og_txt);

        println!("Decoded : {:?}", decoded);
        assert_eq!(decoded, og_txt);

        println!("Test passed!");

        std::fs::remove_file("test.prtgn").unwrap();
    }
}

const XOR_KEY: u8 = 0x66;
const FILE_HEADER: &[u8] = b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team | Version 0.2.0 | \x66 ";


    /// Writes to a file with PRTGN encoding.
    ///
    /// # Examples
    ///
    /// ```Rust
    ///     let filename = "test.prtgn".to_string();
    ///
    ///     let text = "Hello world! This is some example text.".to_string();
    ///
    ///     write(filename, text).unwrap();
    /// ```
    pub fn write(filename: String, text: String) -> Result<()> {
        {
            let mut file = File::create(filename)?;

            file.write_all(FILE_HEADER)?;


            let encoded_bytes: Vec<u8> = text.as_bytes().iter().map(|byte| byte ^ XOR_KEY).collect();


            file.write_all(&encoded_bytes)?;
        }


        Ok(())
    }

    /// Reads a PRTGN encoded file.
    ///
    /// # Examples
    ///
    /// ```Rust
    ///     let filename = "test.prtgn".to_string();
    ///
    ///     let read_text = read(filename).unwrap().to_string();
    ///
    ///     println!("{}", read_text);
    /// ```
    ///
    /// If the file is using an older encoding standard it is automatically updated to the newest version.
    pub fn read(filename: String) -> Result<String> {
        let mut file = File::open(filename.clone())?;

        let mut header_buffer = vec![0u8; FILE_HEADER.len()];
        file.read_exact(&mut header_buffer)?;

        let older_version = if header_buffer.starts_with(FILE_HEADER) {
            "newest"
        } else if header_buffer.starts_with(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team \x66 ") {
            file.seek(SeekFrom::Start(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team \x66 ".len() as u64))?;
            "0.1.3"
        } else if header_buffer.starts_with(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team\x01\xFF\x00 ") {
            file.seek(SeekFrom::Start(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team\x01\xFF\x00 ".len() as u64))?;
            "0.1.2"
        } else if header_buffer.starts_with(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team\x01\xFF\x00") {
            file.seek(SeekFrom::Start(b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team\x01\xFF\x00".len() as u64))?;
            "0.1.1"
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Not a valid PRTGN encoded file."));
        };

        if older_version != "newest" {

            println!("The version of encoding is outdated. Automatically rewriting the file after read.");
            println!("---------------------------------------------");
            print!("");

        };

        let mut encoded_buffer = Vec::new();
        file.read_to_end(&mut encoded_buffer)?;


        let decoded_byte: Vec<u8> = if older_version == "newest" {
            encoded_buffer.iter().map(|byte| byte ^ XOR_KEY).collect()
        } else if older_version == "0.1.3" {
            encoded_buffer.iter().map(|byte| byte ^ 0x66).collect()
        } else if older_version == "0.1.2" {
            encoded_buffer.iter().map(|byte| byte ^ 0xA3).collect()
        } else if older_version == "0.1.1" {
            encoded_buffer.iter().map(|byte| byte ^ 0xA3).collect()
        } else {
            unreachable!()
        };

        write(filename, String::from_utf8(decoded_byte.clone()).unwrap())?;

        String::from_utf8(decoded_byte).map_err(|e: FromUtf8Error| Error::new(ErrorKind::InvalidData, e.to_string()))
    }
