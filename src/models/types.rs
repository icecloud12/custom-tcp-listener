use std::{collections::HashMap, sync::Arc};

use http::HeaderMap;

use super::router::ERouterMethod;

#[derive(Debug)]
pub struct Request {
	pub method: String,
	// pub body: &'a [u8],
	pub parameters: HashMap<String, String>,
	pub headers: HeaderMap,
	pub path: String
}





