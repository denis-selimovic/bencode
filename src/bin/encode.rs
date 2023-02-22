use bencode::Encoder;
use bencode::Type;


fn main() {
    let input_file = std::env::args().nth(1)
            .expect("Usage: parse <input_file> <output_file>");
    let output_file = std::env::args().nth(2)
            .expect("Usage: parse <input_file> <output_file>");

    match Type::load_from_json(input_file) {
        Err(err) => println!("{}", err.to_string()),
        Ok(t) => {
            match Encoder::encode_to(&t, output_file) {
                Err(err) => println!("{}", err.to_string()),
                Ok(_) => println!("Encoded bencode saved to binary file"),
            }
        },
    }
}
