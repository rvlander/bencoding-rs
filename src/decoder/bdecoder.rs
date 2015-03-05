use super::super::btree::bvalue::BValue;
use std::str::Chars;

struct BDecoder <'a> {
	to_parse: Chars<'a>
}

impl  <'a> BDecoder <'a> {

	fn new(to_parse: &'a str) -> BDecoder {
		BDecoder {
			to_parse: to_parse.chars()
		}
	}

	fn parse (&mut self) -> Result<BValue, &str> {
		let opt = self.to_parse.next();
		match opt {
			Some(c) => match c {
				'd' => self.parse_dictionnary(),
				'i' => self.parse_integer(),
				'l' => self.parse_list(),
				'0' => self.parse_string(c),
				'1' => self.parse_string(c),
				'2' => self.parse_string(c),
				'3' => self.parse_string(c),
				'4' => self.parse_string(c),
				'5' => self.parse_string(c),
				'6' => self.parse_string(c),
				'7' => self.parse_string(c),
				'8' => self.parse_string(c),
				'9' => self.parse_string(c),
				_ => Err("Error: bcode could not be parsed: char not expected !"),
			},
			None => Err("Error: bcode could not be parsed: premature end of input !"),
		}
	}

	fn parse_dictionnary(&self) -> Result<BValue, &str> {
		Err("Error: bcode could not be parsed: not supported yet !")
	}

	fn parse_integer(&mut self) -> Result<BValue, &str> {
		let integer: String = self.to_parse.by_ref().take_while(|&c| c != 'e').collect();
		match integer.as_slice().parse::<i64>() {
			Ok(a) => Ok(BValue::Integer(a)),
			Err(_) => Err("Error: bcode could not be parsed: integer malformed"),
		}	
	}

	fn parse_list(&mut self) -> Result<BValue, &str> {
		let mut res = Vec<BValue>::new();
		let next = 'x';
		do {
			res.push(self.parse());
			next = match self.to_parse.next() {
				Some(a) => a,
				None => return Err("Error: bcode could not be parsed: premature end of input !"),
			}
		} while (next == ':');
		match next {
			'e' => Ok(BValue::List(res)),
			_  => Err("Error: bcode could not be parsed: char not expected ('e or ':' is expected)!")
		}

	}

	fn parse_string(&mut self, cin: char) -> Result<BValue, &str> {
		let mut semi = 'x';
		let mut tail: String = self.to_parse.by_ref().take_while(|&c| {semi = c; is_num(c)}).collect();
		tail.insert(0, cin);
		match tail.as_slice().parse::<usize>() {
			Ok(a) => {
				if semi !=':' {return Err("Error: bcode could not be parsed: ':' expected!")};
				let res: String = self.to_parse.by_ref().take(a).collect();
				Ok(BValue::String(res))
			},
			Err(_) => Err("Error: bcode could not be parsed: string size malformed !"), 
		}
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

	#[test]
	fn test_parse_garbage() {
		let mut decoder = BDecoder::new("garbage");
		assert_eq!(decoder.parse(), Err("Error: bcode could not be parsed: char not expected !"));
	}

	#[test]
	fn test_parse_eoi() {
		let mut decoder = BDecoder::new("");
		assert_eq!(decoder.parse(), Err("Error: bcode could not be parsed: premature end of input !"));	
	}

	#[test]
	fn test_parse_integer() {
		let mut decoder = BDecoder::new("i128e");
		assert_eq!(decoder.parse(), Ok(BValue::Integer(128)));	
	}

	#[test]
	fn test_parse_string() {
		let mut decoder = BDecoder::new("4:toto");
		assert_eq!(decoder.parse(), Ok(BValue::String(String::from_str("toto"))));

		decoder = BDecoder::new("0:");
		assert_eq!(decoder.parse(), Ok(BValue::String(String::from_str(""))));	
	}

		#[test]
	fn test_parse_integer() {
		let mut decoder = BDecoder::new("l4:toto:i128ee");
		let mut res = Vec<BValue>::new();
		res.push(BValue::String(String::from_str("toto")));
		res.push(BValue::List(BValue::Integer(128)));
		assert_eq!(decoder.parse(), Ok(BValue::List(res));	
	}
}