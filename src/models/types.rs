use std::{collections::HashMap, sync::Arc};

use http::HeaderMap;


#[derive(Debug)]
pub struct Request {
	pub method: String,
	pub body: Vec<u8>,
	pub parameters: HashMap<String, String>,
	pub headers: HeaderMap,
	pub path: String
}