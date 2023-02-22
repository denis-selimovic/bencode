use std::fs::File;
use std::io::{BufReader, Read};

use bencode::protocol::decode;


fn main() {
    let input_file = std::env::args().nth(1)
            .expect("Usage: parse <input_file> <output_file>");
    let output_file = std::env::args().nth(2)
            .expect("Usage: parse <input_file> <output_file>");

    match File::open(input_file) {
        Err(err) => {
            println!("{}", err.to_string());
            return;
        },
        Ok(file) => {
            let mut reader = BufReader::new(file).bytes().map(|c| c.unwrap());
            let t = decode(&mut reader);

            match t  {
                Err(err) => {
                    println!("{}", err.to_string());
                    return;
                },
                Ok(t) => {
                    match t.save_to_json(output_file) {
                        Err(err) => {
                            println!("{}", err.to_string());
                            return;
                        },
                        Ok(_) => println!("Saved to json file"),
                    }
                },
            }
        },
    }
}
