use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Hello from the PRTGN Development Team");

        let text = "Hello from PRTGN Encoding!".to_string();
        let og_txt = text.clone();
        write("test.prtgn".to_string(), text).unwrap();

        let dcoded = read("test.prtgn".to_string()).unwrap().to_string();

        println!("Original : {:?}", og_txt);

        println!("Decoded : {:?}", dcoded);
        assert_eq!(dcoded, og_txt);

        println!("Test passed!");

        std::fs::remove_file("test.prtgn").unwrap();
    }
}


pub use read::read;
pub use write::write;


const XOR_KEY: u8 = 0xA3;
const FILE_HEADER: &[u8] = b"Encoded with PRTGN | https://github.com/PRTGN-Development-Team\x01\xFF\x00";

pub mod write {
    use super::*;

    pub fn write(filename: String, text: String) -> std::io::Result<()> {
        {
            let mut file = File::create(filename)?;
            // Write a slice of bytes to the file

            //let bytes = text.into_bytes();

            file.write_all(FILE_HEADER)?;


            let encoded_bytes: Vec<u8> = text.as_bytes().iter().map(|byte| byte ^ XOR_KEY).collect();


            file.write_all(&encoded_bytes)?;
        }


        Ok(())
    }
}

pub mod read {
    use std::io::{Error, ErrorKind};
    use std::string::FromUtf8Error;
    use super::*;

    pub fn read(filename: String) -> Result<String> {
        {
            let mut file = File::open(filename)?;

            let mut header_buffer = vec![0u8; FILE_HEADER.len()];

            file.read_exact(header_buffer.as_mut_slice())?;

            if header_buffer != FILE_HEADER {
                return Err(Error::new(ErrorKind::InvalidData, "Not a valid PRTGN Encoded file. Try Again."))
            }

            let mut encoded_buffer = Vec::new();
            file.read_to_end(&mut encoded_buffer)?;


            let decoded_byte: Vec<u8> = encoded_buffer.iter().map(|byte| byte ^ XOR_KEY).collect();

            String::from_utf8(decoded_byte).map_err(|e: FromUtf8Error| Error::new(ErrorKind::InvalidData, e.to_string()))

        }

    }

}
