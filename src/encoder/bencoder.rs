extern crate core;

use super::super::btree::bvalue::BValue;
use std::collections::HashMap;


pub fn bencode (to_bencode: BValue) -> Vec<u8> {
   	match to_bencode {
        BValue::Dictionary(map) => bencode_dictionary(map),
        BValue::Integer(integer) => bencode_integer(integer),
        BValue::List(list) => bencode_list(list),
        BValue::String(string) => bencode_string(string),
    }
}

fn bencode_dictionary(map: HashMap<String, BValue>) -> Vec<u8> {
	let mut res = Vec::<u8>::new();
	
	res.push('d' as u8);

	for (key, value) in map.iter() {
		let mut bstring = bencode_string(key.clone().into_bytes());
		res.append(&mut bstring);
		let mut bvalue = bencode((*value).clone());
		res.append(&mut bvalue);
	}

	res.push('e' as u8);	
	res
}


fn bencode_integer(integer: i64) -> Vec<u8> {
	let mut res = Vec::<u8>::new();

	res.push('i' as u8);
	res.append(&mut integer.to_string().into_bytes());
	res.push('e' as u8);
	res
}

fn bencode_list(list: Vec<BValue>) -> Vec<u8> {
	let mut res = Vec::<u8>::new();
	
	res.push('l' as u8);

	for value in list.iter() {
		let mut bvalue = bencode((*value).clone());
		res.append(&mut bvalue);
	}

	res.push('e' as u8);	
	res
}

fn bencode_string(string: Vec<u8>) -> Vec<u8> {
	let mut res = Vec::<u8>::new();
	let mut stri = string;

	let size = stri.len().to_string();
	res.append(&mut size.into_bytes());
	res.push(':' as u8);
	res.append(&mut stri);
	res
}


#[cfg(test)]
mod test {
	use super::bencode;
	use super::super::super::btree::bvalue::BValue;
	use super::super::super::decoder::bdecoder::BDecoder;
	use std::collections::HashMap;

	#[test]
	fn test_parse_integer() {
		let bencoded = "i128e".to_string().into_bytes();
		assert_eq!(bencoded, bencode(BValue::Integer(128)));	
	}

	#[test]
	fn test_parse_string() {
		let bencoded = "4:toto".to_string().into_bytes();
		assert_eq!(bencoded, bencode(BValue::String(String::from_str("toto").into_bytes())));

		let bencoded1 = "0:".to_string().into_bytes();
		assert_eq!(bencoded1, bencode(BValue::String(String::from_str("").into_bytes())));	
	}

	#[test]
	fn test_parse_list() {
		let bencoded = "l4:totoi128ee".to_string().into_bytes();

		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));
		assert_eq!(bencoded, bencode(BValue::List(res)));	
	}

	#[test]
	fn test_parse_dictionary() {
		let mut map = HashMap::<String, BValue>::new();

		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));
		
		map.insert("papa".to_string(), BValue::List(res));
		map.insert("c".to_string(), BValue::Integer(25));

		let bencoded = bencode(BValue::Dictionary(map.clone()));
		let mut decoder = BDecoder::new(&bencoded);

		assert_eq!(decoder.parse(), Ok(BValue::Dictionary(map)));	
	}
}