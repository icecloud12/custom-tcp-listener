use std::{collections::HashMap, future::Future, io::Error, ops::Range, pin::Pin, process::Output, slice::Iter, sync::Arc};
use http::HeaderMap;
use regex::{Regex, RegexBuilder};
use tokio::net::TcpStream;
use http::Response;
use super::types::Request;
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;



pub fn response_to_bytes( response: Response<&[u8]>) -> Vec<u8>{
	let (parts, body) = response.into_parts();
	let mut _response_line = format!("{:#?} {}\r\n", parts.version, parts.status);
	parts.headers.iter().for_each(
		|(header_name, header_value)|{
			_response_line = _response_line.clone() + &format!("{}:{}\r\n", header_name.as_str(), header_value.to_str().unwrap());
		}		 
	);
	let formatted_response = format!("{}\r\n{}", _response_line,String::from_utf8(body.to_vec()).unwrap());
	formatted_response.as_bytes().to_vec()
}

//#region ERouterMethod
pub enum  ERouterMethod {
	CONNECT,
	DELETE,
	GET,
	HEAD,
	OPTIONS,
	PATCH,
	POST,
	PUT,
	TRACE,
}
//#endregion
//#region

pub struct Route<F, O>
where 
	F: Fn(Request, TcpStream) -> O + std::marker::Sync + std::marker::Send + 'static,
	O: Future<Output = ()> +std::marker::Send  + 'static
{
	pub method: ERouterMethod,
	pub path: String,
	pub handler: F,
	pub parameters: Vec<String>,
	pub regex: Regex
}

impl <F, O> Route<F, O>
where
	F: Fn(Request, TcpStream) -> O + std::marker::Sync + std::marker::Send + 'static,
	O: Future<Output = ()> +std::marker::Send  + 'static
{
	pub fn connect (handler: F)->(ERouterMethod, F){
		(ERouterMethod::CONNECT, handler)
	}
	pub fn get(handler: F) -> (ERouterMethod, F){
		(ERouterMethod::GET,handler)
	}
	pub fn delete (handler: F)->(ERouterMethod, F){
		(ERouterMethod::DELETE, handler)
	}
	pub fn head (handler :F)->(ERouterMethod, F){
		(ERouterMethod::HEAD, handler)
	}
	pub fn option (handler :F)->(ERouterMethod, F){
		(ERouterMethod::OPTIONS, handler)
	}
	pub fn patch (handler :F)->(ERouterMethod, F){
		(ERouterMethod::PATCH, handler)
	}
	pub fn post (handler :F)->(ERouterMethod, F){
		(ERouterMethod::POST, handler)
	}
	pub fn put(handler :F)->(ERouterMethod, F){
		(ERouterMethod::PUT, handler)
	}
	pub fn trace(handler :F)->(ERouterMethod, F){
		(ERouterMethod::TRACE, handler)
	}

}
//#endregion

impl ERouterMethod {
	fn as_str(&self) -> &'static str {
		match self {
			ERouterMethod::CONNECT => "CONNECT",
			ERouterMethod::DELETE => "DELETE",
			ERouterMethod::GET => "GET",
			ERouterMethod::HEAD => "HEAD",
			ERouterMethod::OPTIONS => "OPTIONS",
			ERouterMethod::PATCH => "PATCH",
			ERouterMethod::POST => "POST",
			ERouterMethod::PUT => "PUT",
			ERouterMethod::TRACE => "TRACE",
		}
	}
	fn as_slice() -> [ERouterMethod; 9]{
		[
			ERouterMethod::CONNECT,ERouterMethod::DELETE,ERouterMethod::GET,ERouterMethod::HEAD,ERouterMethod::OPTIONS,ERouterMethod::PATCH,ERouterMethod::POST,ERouterMethod::PUT,ERouterMethod::TRACE
		]
	}
}
pub struct Router <F, O>
	where 
	F: Fn(Request, TcpStream) -> O + std::marker::Sync + std::marker::Send + 'static,
	O: Future<Output = ()> +std::marker::Send  + 'static
{
	pub routes : HashMap<String, HashMap<String, Route<F, O>>>,
	pub keys: Vec<String>
}
impl <F, O> Router<F, O>
	where 
		F: Fn(Request, TcpStream) -> O + std::marker::Sync + std::marker::Send + 'static,
		O: Future<Output = ()> +std::marker::Send  + 'static
{
	pub fn new() -> Router<F, O>
	{
		let mut hm: HashMap<String, HashMap<String, Route<F, O>>> = HashMap::new();
		Router {
    		routes: hm,
			keys: vec![]
		}		
	}
	pub fn route (self, path:String, route_helper:(ERouterMethod, F)) -> Router<F, O>
	{
		let mut hm= self.routes;
		let mut keys = self.keys;
		let (regex_path, path_parameters) = Router::<F, O>::extract_path_parameter(&path);
		
		let method_route_pairs  = match hm.get_mut(&regex_path.as_str().to_string()) {
			Some(p) => p,
			None => {
				let new_val = HashMap::<String, Route<F, O>>::new();
				hm.insert(regex_path.as_str().to_string(), new_val);
				hm.get_mut(&regex_path.as_str().to_string()).unwrap()
			}
		};
		if !keys.contains(&regex_path.as_str().to_string()) {
			keys.push(regex_path.as_str().to_string());
		}
		
		
		
		method_route_pairs.insert(route_helper.0.as_str().to_string(), Route{
			method: route_helper.0,
			path,
			handler: route_helper.1,
			parameters: path_parameters,
			regex: regex_path.clone()
		});
		
		Router {
			routes: hm,
			keys
		}
	}
	fn extract_path_parameter(path:&String) -> (Regex, Vec<String>){
		let re = Regex::new(r":([^\/]*):").unwrap();
		let mut route_regex:String = path.clone().replace("/", "\\/");
		let mut ranges:Vec<Range<usize>> = vec![];
		//let mut path_parameter_map: HashMap<&String, String> = HashMap::new();
		let path_parameters = re.captures_iter(&route_regex.as_str()).
			filter_map( 
				|fm| {
					fm.get(1).map( 
						|m| {
							ranges.push(m.range());
							m.as_str().to_string()
						}
					)
				}
			).collect::<Vec<String>>();
		ranges.iter().enumerate().rev().for_each(|(index, r)| {
			route_regex.replace_range(r.start-1..r.end+1, format!("(?<{}>[^\\/]*)",path_parameters[index]).as_str());
		});
		
		if route_regex.ends_with("\\/*"){

			let len = route_regex.len();
			route_regex.replace_range(len-1..len, "(.+)");
		}
		let created_regex = RegexBuilder::new(route_regex.as_str()).build().unwrap();
		(created_regex, path_parameters)
	}
	
}








