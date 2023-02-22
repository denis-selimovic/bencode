# Bencode (Bee-encode)

Rust implementation for bencode encoding. Bencode is the encoding used by the peer-to-peer file sharing system BitTorrent for storing and transmitting loosely structured data. Bencoding is most commonly used in torrent files, and as such is part of the BitTorrent specification. These metadata files are simply bencoded dictionaries.

Bencoding is simple and (because numbers are encoded as text in decimal notation) is unaffected by endianness, which is important for a cross-platform application like BitTorrent. It is also fairly flexible, as long as applications ignore unexpected dictionary keys, so that new ones can be added without creating incompatibilities.

### Installation

You can install `bencode_encoder` crate in your Rust project by using:
> `$ cargo add bencode-encoder`


### Data types

Bencode specification supports 4 data types:
- byte strings (encoded as `length:content`, e.g. `4:rust`)
- integers (encoded as `inumbere`, e.g. `i20e`)
- lists (encoded as `l<contents>e`, e.g. `l4:rusti20ee`)
- dictionaries with sorted strings keys (encoded as `d<contents>e`, e.g. `d1:ki2023ee`)


### Example usage (decoding from .torrent file and storing as .json file)

```rust
use bencode_encoder::Decoder;


fn main() {
    let input_file = std::env::args().nth(1)
            .expect("Usage: decode <input_file> <output_file>");
    let output_file = std::env::args().nth(2)
            .expect("Usage: decode <input_file> <output_file>");

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
```

### Example usage (reading from .json file and encoding to .torrent file)

```rust
use bencode_encoder::{Encoder, Type};


fn main() {
    let input_file = std::env::args().nth(1)
            .expect("Usage: encode <input_file> <output_file>");
    let output_file = std::env::args().nth(2)
            .expect("Usage: encode <input_file> <output_file>");

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
```

### BNF

BNF for parsing is shown below (click [here](https://hackage.haskell.org/package/bencoding-0.4.3.0/docs/Data-BEncode.html) to read more). This crate implements simple parser according to BNF shown below.

```
<BE>    ::= <DICT> | <LIST> | <INT> | <STR>

<DICT>  ::= "d" 1 * (<STR> <BE>) "e"
<LIST>  ::= "l" 1 * <BE>         "e"
<INT>   ::= "i"     <SNUM>       "e"
<STR>   ::= <NUM> ":" n * <CHAR>; where n equals the <NUM>

<SNUM>  ::= "-" <NUM> / <NUM>
<NUM>   ::= 1 * <DIGIT>
<CHAR>  ::= %
<DIGIT> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

### Tests

To run tests use:

> `$ cargo test`
