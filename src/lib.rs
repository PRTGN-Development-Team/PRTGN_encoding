use std::fs::File;
use std::io::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        println!("Hello, world!");
        println!("Test : {result}");


//       let la = la();

//        supported_unicode_char()

    }
}

// https://stackoverflow.com/a/53827079

pub mod write {
    use super::*;

    fn write(filename: String, text: String) -> std::io::Result<()> {
        {
            let mut file = File::create(filename)?;
            // Write a slice of bytes to the file

            let bytes = text.into_bytes();

            file.write_all(&bytes)?;
        }


        Ok(())
    }
}

pub mod read {
    use super::*;

    fn read(filename: String) -> std::io::Result<()> {
        {
            let mut file = File::open(filename)?;
            // read the same file back into a Vec of bytes
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer)?;
            println!("{:?}", buffer);
        }

        Ok(())
    }

}



// fn supported_unicode_char() {
//     //let v = Vec::from_iter('\u{0000}'..='\u{10FFFF}');
//     // println!("Unicode : {:?}", v);
//     // println!("Unicode Char : {}", String::from_iter(v));
//
// //! **Supported Unicode Blocks : https://en.wikipedia.org/wiki/Unicode_block#List_of_blocks**
//
//     let basic_latin = Vec::from_iter('\u{0000}'..='\u{007F}');
//
//     let latin_one_supplement = Vec::from_iter('\u{0080}'..='\u{00FF}');
//
//     let latin_extended_a = Vec::from_iter('\u{0100}'..='\u{017F}');
//
//     let latin_extended_b = Vec::from_iter('\u{0180}'..='\u{024F}');
//
//     let ipa_extensions = Vec::from_iter('\u{0250}'..='\u{02AF}');
//
//     let spacing_modifier_letters = Vec::from_iter('\u{02B0}'..='\u{02FF}');
//
//     let combining_diacritical_marks = Vec::from_iter('\u{03000}'..='\u{036F}');
//
//     let general_punctuation = Vec::from_iter('\u{2000}'..='\u{206F}');
//
//     let super_sub_script = Vec::from_iter('\u{2070}'..='\u{209F}');
//
//     let currency_symbols = Vec::from_iter('\u{200A0}'..='\u{20CF}');
//
//     let letterlike_symbols = Vec::from_iter('\u{2100}'..='\u{214F}');
//
//     let maths_operators = Vec::from_iter('\u{2200}'..='\u{22FF}');
//
//     let box_draw = Vec::from_iter('\u{2500}'..='\u{257F}');
//
//
// }


// use std::fs;
// use std::io::{BufWriter, Write};
//
// fn append_data_to_file(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
//     let file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
//     let mut file = BufWriter::new(file);
//
//     // You can either try to write all data at once
//     file.write_all(&data)?;
//
//     // Or try to write as much as possible, but need
//     // to take care of the remaining bytes yourself
//     let remaining = file.write(&data)?;
//     if remaining != 0 {
//         // handle...
//     }
//
//     // You definitely need to flush a BufWriter
//     // as it cannot guarantee emptying its buffer
//     // when it goes out of scope
//     file.flush()?;
//
//     Ok(())
// }