use bencode::Decoder;


fn main() {
    let input_file = std::env::args().nth(1)
            .expect("Usage: parse <input_file> <output_file>");
    let output_file = std::env::args().nth(2)
            .expect("Usage: parse <input_file> <output_file>");

    match Decoder::decode_from(input_file) {
        Err(err) => println!("{}", err.to_string()),
        Ok(t) => {
            match t.save_to_json(output_file) {
                Err(err) => println!("{}", err.to_string()),
                Ok(_) => println!("Decoded bencode saved to .json file"),
            }
        },
    }
}
