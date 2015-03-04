use super::super::btree::bvalue::BValue;
use std::str::Chars;

struct BDecoder <'a> {
	to_parse: Chars<'a>,
}

impl  <'a> BDecoder <'a> {

	fn new(to_parse: &'a str) -> BDecoder {
		BDecoder {
			to_parse: to_parse.chars()
		}
	}

	fn parse (&mut self) -> Result<BValue, &str> {
		self.inner_parse(None)
	}

	fn inner_parse (&mut self, cin: Option<char>) -> Result<BValue, &str> {
		let opt = match cin {
			Some(c) => Some(c),
			None => self.to_parse.next(),
		};	
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
			Err(err) => Err("Error: bcode could not be parsed: integer malformed"),
		}	
	}

	fn parse_list(&self) -> Result<BValue, &str> {
		Err("Error: bcode could not be parsed: not supported yet !")
	}

	fn parse_string(&self, cin: char) -> Result<BValue, &str> {
		Err("Error: bcode could not be parsed: not supported yet !")
	}
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
}