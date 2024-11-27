use std::{collections::HashMap, future::Future, io::Error, ops::Range, pin::Pin, process::Output, slice::Iter, sync::Arc};
use http::HeaderMap;
use regex::{Regex, RegexBuilder};
use tokio::net::TcpStream;

use super::types::Request;
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;



async fn test(request: Request) {
	
	return ();
	
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
	pub closure: F,
	pub parameters: Vec<String>,
	pub regex: Regex
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
	pub fn route (self, path:String, method_closure:(ERouterMethod, F)) -> Router<F, O>
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
		
		
		
		method_route_pairs.insert(method_closure.0.as_str().to_string(), Route{
			method: method_closure.0,
			path,
			closure: method_closure.1,
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

// pub fn connect (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::CONNECT, closure)
// }
// pub fn delete (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::DELETE, closure)
// }
pub fn get<F, O> (closure: F) -> (ERouterMethod, F)
	where 
		F: Fn(Request, TcpStream) -> O + std::marker::Sync + std::marker::Send + 'static,
		O: Future<Output = ()> +std::marker::Send  + 'static
		{
	let ret= (ERouterMethod::GET,closure);
	ret
}
// pub fn head (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::HEAD, closure)
// }
// pub fn option (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::OPTIONS, closure)
// }
// pub fn patch (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::PATCH, closure)
// }
// pub fn post (closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::POST, closure)
// }
// pub fn put(closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::PUT, closure)
// }
// pub fn trace(closure: impl Future)->(ERouterMethod, impl Future){
// 	return (ERouterMethod::TRACE, closure)
// }

