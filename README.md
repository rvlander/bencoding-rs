# bencoding-rs
A simple bencoding parser.

This library aims at parsing Vec<u8> encoded in the Bencoding format described here:
https://wiki.theory.org/BitTorrentSpecification#Bencoding

Example:
```rust
extern crate bencoding;
use bencoding::decoder::bdecoder::BDecoder;

let mut decoder = BDecoder::new("d4:papa:l4:toto:i128ee:1:c:i25ee");
```
which should result in the following data structure:
```rust
Ok(
	Dictionary({
		"papa": List([
			String([116, 111, 116, 111]), //byte encoding of "toto"
			Integer(128)
		]),
		"c": Integer(25)
	})
)
```

This structure is built using the following type:
```rust
pub enum BValue {
	Dictionary(HashMap<String, BValue>),
	List(Vec<BValue>),
	String(Vec<u8>),
	Integer(i64),
}
```
It can also encode such a structure to a bencoded Vec<u8>
Example:
```rust
extern crate bencoding;
use bencoding::encoder::bencoder::bencode;

let foo: BValue;
/* ... obtain / modify a BValue in any way ... */

let mut bencoded = bencode(foo);
```