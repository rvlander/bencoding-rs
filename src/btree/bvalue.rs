use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BValue {
	Dictionary(HashMap<String, BValue>),
	List(Vec<BValue>),
	String(Vec<u8>),
	Integer(i64),
}

impl BValue {
	pub fn as_dictionary(self) -> Result<HashMap<String, BValue>, String> {
		match self {
			BValue::Dictionary(map) => Ok(map),
			_ => Err("BValue is not a dictionnary".to_string()),
		}
	}

	pub fn as_list(self) -> Result<Vec<BValue>, String> {
		match self {
			BValue::List(list) => Ok(list),
			_ => Err("BValue is not a list".to_string()),
		}
	}

	pub fn as_bytes(self) -> Result<Vec<u8>, String> {
		match self {
			BValue::String(bytes) => Ok(bytes),
			_ => Err("BValue is not a byte string".to_string()),
		}
	}

	pub fn as_string(self) -> Result<String, String> {
		let list = try!(self.as_bytes());
		match String::from_utf8(list) {
			Ok(string) => Ok(string),
			_ => Err("BValue byte string is not convertible to utf8".to_string()),
		}
	}

	pub fn as_integer(self) -> Result<i64, String> {
		match self {
			BValue::Integer(integer) => Ok(integer),
			_ => Err("BValue is not an integer".to_string()),
		}
	}
}

#[cfg(test)]
mod test {
	use super::BValue;
	use std::collections::HashMap;

	#[test]
	fn test_as_integer() {
		let integer = BValue::Integer(23);
		assert_eq!(Ok(23), integer.as_integer());	

		let list = BValue::List(vec![BValue::Integer(23)]);
		assert_eq!(Err("BValue is not an integer".to_string()), list.as_integer());
	}

	#[test]
	fn test_as_bytes() {
		let bytes = BValue::String(vec![116, 111, 116, 111]);
		assert_eq!(Ok(vec![116, 111, 116, 111]), bytes.as_bytes());	

		let list = BValue::List(vec![BValue::Integer(23)]);
		assert_eq!(Err("BValue is not a byte string".to_string()), list.as_bytes());
	}

	#[test]
	fn test_as_string() {
		let bytes = BValue::String(vec![116, 111, 116, 111]);
		assert_eq!(Ok("toto".to_string()), bytes.as_string());	

		let list = BValue::List(vec![BValue::Integer(23)]);
		assert_eq!(Err("BValue is not a byte string".to_string()), list.as_string());
	}

	#[test]
	fn test_as_list() {
		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));

		let mut res1 = Vec::<BValue>::new();
		res1.push(BValue::String(String::from_str("toto").into_bytes()));
		res1.push(BValue::Integer(128));
		assert_eq!(Ok(res1), BValue::List(res).as_list());	

		let list = BValue::Integer(23);
		assert_eq!(Err("BValue is not a list".to_string()), list.as_list());
	}

	#[test]
	fn test_as_dictionary() {
		let mut map = HashMap::<String, BValue>::new();

		let mut res = Vec::<BValue>::new();
		res.push(BValue::String(String::from_str("toto").into_bytes()));
		res.push(BValue::Integer(128));
		
		map.insert("papa".to_string(), BValue::List(res));
		map.insert("c".to_string(), BValue::Integer(25));

		let mut map1 = HashMap::<String, BValue>::new();

		let mut res1 = Vec::<BValue>::new();
		res1.push(BValue::String(String::from_str("toto").into_bytes()));
		res1.push(BValue::Integer(128));
		
		map1.insert("papa".to_string(), BValue::List(res1));
		map1.insert("c".to_string(), BValue::Integer(25));
		assert_eq!(Ok(map1), BValue::Dictionary(map).as_dictionary());	

		let list = BValue::List(vec![BValue::Integer(23)]);
		assert_eq!(Err("BValue is not a dictionnary".to_string()), list.as_dictionary());
	}


}