use prtgn_encoding::read;

fn main() {

    let filename = "test.prtgn".to_string();

    let read_text = read(filename).unwrap().to_string();

    println!("{}", read_text);
}