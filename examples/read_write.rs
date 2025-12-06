use prtgn_encoding::{read, write};

fn main() {
    let filename = "test.prtgn".to_string();
    let text = "Hello world! This is some example text.".to_string();

    write(filename, text).unwrap();

    let filename = "test.prtgn".to_string();

    let read_text = read(filename).unwrap().to_string();

    println!("{}", read_text)
}