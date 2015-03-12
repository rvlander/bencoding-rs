# bencoding-rs
A simple bencoding parser.

This library aims at parsing Strings encoded in the Bencoding format described here:
https://wiki.theory.org/BitTorrentSpecification#Bencoding

Example:
```rust
use becoding-rs::bencoding::bdecoder;

let mut decoder = BDecoder::new("d4:papa:l4:toto:i128ee:1:c:i25ee");
```
which should result in the following data structure:
```rust
Ok(
	Dictionary({
		"papa": List([
			String("toto"),
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
	String(String),
	Integer(i64),
}
```