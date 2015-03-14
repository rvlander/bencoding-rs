use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum BValue {
	Dictionary(HashMap<String, BValue>),
	List(Vec<BValue>),
	String(Vec<u8>),
	Integer(i64),
}