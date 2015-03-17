extern crate bencoding;
use bencoding::decoder::bdecoder::BDecoder;
use bencoding::encoder::bencoder::bencode;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    // Create a path to the desired file
    let path = Path::new("./examples/ubuntu.torrent");
    let display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let mut file = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `IoResult<String>`
    let mut to_parse = Vec::<u8>::new();
    match file.read_to_end(&mut to_parse) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(file) => (), 
    }

	let mut decoder = BDecoder::new(&to_parse);
    let decoded = match decoder.parse() {
        Ok(a) => a,
        Err(err) => panic!("parsing error {}", err),
    };

    let mut encoded = bencode(decoded.clone());

    let mut decoder2 = BDecoder::new(&encoded);
    let decoded2 = match decoder2.parse() {
        Ok(a) => a,
        Err(err) => panic!("parsing error {}", err),
    };

    assert_eq!(decoded, decoded2);
}
