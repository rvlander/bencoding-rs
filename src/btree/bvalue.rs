use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum BValue {
	Dictionary(HashMap<String, BValue>),
	List(Vec<BValue>),
	String(String),
	Integer(i64),
}