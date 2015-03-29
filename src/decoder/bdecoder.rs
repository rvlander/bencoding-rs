extern crate core;

use super::super::btree::bvalue::BValue;

extern crate linked_hash_map;
use self::linked_hash_map::LinkedHashMap;

use self::core::slice::Iter;


pub struct BDecoder <'a>{
	to_parse: Iter<'a, u8>
}

impl  <'a> BDecoder <'a>{

	pub fn new(to_parse: &'a Vec<u8>) -> BDecoder <'a> {
		BDecoder {
			to_parse: to_parse.iter()
		}
	}

	pub fn parse(&mut self) -> Result<BValue, String> {
		self.inner_parse(None)
	}

	fn inner_parse (&mut self, cin: Option<u8>) -> Result<BValue, String> {
		let next = match cin {
			Some(a) => a,
			None => match self.to_parse.next(){
				Some(c) => *c,
				None => return self.parse_error("premature end of input !"),
			},
		};
    	match next as char {
            'd' => self.parse_dictionary(),
            'i' => self.parse_integer(),
            'l' => self.parse_list(),
            '0' => self.parse_string(next),
            '1' => self.parse_string(next),
            '2' => self.parse_string(next),
            '3' => self.parse_string(next),
            '4' => self.parse_string(next),
            '5' => self.parse_string(next),
            '6' => self.parse_string(next),
            '7' => self.parse_string(next),
            '8' => self.parse_string(next),
            '9' => self.parse_string(next),
            _ => self.parse_error("char not expected !"),
        }
    }

	fn parse_dictionary(&mut self) -> Result<BValue, String> {
		let mut res = LinkedHashMap::<String, BValue>::new();
		let mut next:Option<u8> = None;
		let mut key:String;
		while {
			match try!(self.inner_parse(next)) {
				BValue::String(k) => key =  match String::from_utf8(k) {
					Ok(str) => str,
					_ => return self.parse_error("dictionary key is not a valid UTF8 string !")
				},
				_ => return self.parse_error("dictionary key must be a string !"),
			};
			res.insert(key, try!(self.inner_parse(None)));
			let next_char = match self.to_parse.next() {
				Some(a) => *a,
				None => return self.parse_error("premature end of input !"),
			};
			next = Some(next_char);
			next_char as char != 'e'
		} {};
		Ok(BValue::Dictionary(res))
	}

	fn parse_integer(&mut self) -> Result<BValue, String> {
		let integer: String = self.to_parse.by_ref().map(|c| *c as char).take_while(|&c| c != 'e').collect();
		match (&integer).parse::<i64>() {
			Ok(a) => Ok(BValue::Integer(a)),
			Err(_) => self.parse_error("integer malformed"),
		}	
	}

	fn parse_list(&mut self) -> Result<BValue, String> {
		let mut res = Vec::<BValue>::new();
		let mut next: Option<u8> = None;
		while {
			match self.inner_parse(next) {
				Ok(bvalue) => res.push(bvalue),
				Err(err) => return Err(err),
			};
			let next_char = match self.to_parse.next() {
				Some(a) => *a,
				None => return self.parse_error("premature end of input !"),
			};
			next = Some(next_char);
			next_char as char != 'e'
		} {};
		Ok(BValue::List(res))
	}

	fn parse_string(&mut self, cin: u8) -> Result<BValue, String> {
		let mut semi = 'c';
		let mut tail: String = self.to_parse.by_ref().map(|c| *c as char).take_while(|&c| {semi = c; is_num(c)}).collect();
		tail.insert(0, cin as char);
		match (&tail).parse::<usize>() {
			Ok(a) => {
				if semi != ':' {return self.parse_error("':' expected")};
				let res: Vec<u8> = self.to_parse.by_ref().map(|c| *c).take(a).collect();
				Ok(BValue::String(res))
			},
			Err(_) => self.parse_error("string size malformed"), 
		}
	}

	fn parse_error(&mut self, message: &'a str) -> Result<BValue, String> {
		let mut error =  String::new();
		error.push_str("Error: bcode could not be parsed: ");
		error.push_str(message);
		error.push_str(" Precedes : ");
		let follows: Vec<u8> = self.to_parse.by_ref().take(20).map(|c| *c).collect();
		match String::from_utf8(follows) {
			Ok(a) => error.push_str(&a),
			_=> return Err("can not display following bytes".to_string()),
		}
		return Err(error);
	}
}

fn is_num(c: char) -> bool {
	c =='0'
	|| c == '1'
	|| c == '2'
	|| c == '3'
	|| c == '4'
	|| c == '5'
	|| c == '6'
	|| c == '7'
	|| c == '8'
	|| c == '9'
}

#[cfg(test)]
mod test {
	use super::BDecoder;
	use super::super::super::btree::bvalue::BValue;
	use super::linked_hash_map::LinkedHashMap;

	#[test]
	fn test_parse_garbage() {
		let to_parse = "garbage".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		assert_eq!(decoder.parse(), Err("Error: bcode could not be parsed: char not expected ! Precedes : arbage".to_string()));
	}

	#[test]
	fn test_parse_eoi() {
		let to_parse = "".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		assert_eq!(decoder.parse(), Err("Error: bcode could not be parsed: premature end of input ! Precedes : ".to_string()));	
	}

	#[test]
	fn test_parse_integer() {
		let to_parse = "i128e".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		assert_eq!(decoder.parse(), Ok(BValue::Integer(128)));	
	}

	#[test]
	fn test_parse_string() {
		let to_parse = "4:toto".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		assert_eq!(decoder.parse(), Ok(BValue::String(String::from_str("toto").into_bytes())));

		let to_parse1 = "0:".to_string().into_bytes();
		let mut decoder1 = BDecoder::new(&to_parse1);
		assert_eq!(decoder1.parse(), Ok(BValue::String(String::from_str("").into_bytes())));	
	}

	#[test]
	fn test_parse_list() {
		let to_parse = "l4:totoi128ee".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));
		assert_eq!(decoder.parse(), Ok(BValue::List(res)));	
	}

	#[test]
	fn test_parse_dictionary() {
		let to_parse = "d4:papal4:totoi128ee1:ci25ee".to_string().into_bytes();
		let mut decoder = BDecoder::new(&to_parse);
		let mut map = LinkedHashMap::<String, BValue>::new();

		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));
		
		map.insert("papa".to_string(), BValue::List(res));
		map.insert("c".to_string(), BValue::Integer(25));

		assert_eq!(decoder.parse(), Ok(BValue::Dictionary(map)));	
	}
}