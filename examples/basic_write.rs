use prtgn_encoding::write;

fn main() {

    let filename = "test.prtgn".to_string();
    let text = "Hello world! This is some example text.".to_string();

    write(filename, text).unwrap();
}