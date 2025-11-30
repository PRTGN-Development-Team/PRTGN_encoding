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

        supported_unicode_char()

    }
}


fn supported_unicode_char() {
    //let v = Vec::from_iter('\u{0000}'..='\u{10FFFF}');
    // println!("Unicode : {:?}", v);
    // println!("Unicode Char : {}", String::from_iter(v));

//! **Supported Unicode Blocks : https://en.wikipedia.org/wiki/Unicode_block#List_of_blocks**

    let basic_latin = Vec::from_iter('\u{0000}'..='\u{007F}');

    let latin_one_supplement = Vec::from_iter('\u{0080}'..='\u{00FF}');

    let latin_extended_a = Vec::from_iter('\u{0100}'..='\u{017F}');

    let latin_extended_b = Vec::from_iter('\u{0180}'..='\u{024F}');

    let ipa_extensions = Vec::from_iter('\u{0250}'..='\u{02AF}');


}


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