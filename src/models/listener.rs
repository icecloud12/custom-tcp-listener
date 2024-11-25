use std::any;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use http::header::Keys;
use httparse;
use http::{HeaderMap, HeaderName, HeaderValue};
use regex::Regex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};

use super::router::{ PinnedFuture, Router};
use super::types::Request;




pub async fn bind<F, O>(router:Router<F,O>, address:&str) -> Result<(), Box<dyn std::error::Error>>
	where 
		F: Fn(Request) -> O + std::marker::Sync + std::marker::Send + 'static,
		O: Future<Output = ()> +std::marker::Send  + 'static
{
	let listener = TcpListener::bind(address).await?;
	let rc_keys:Arc<Vec<Regex>> = Arc::new(router.keys.iter().map(|k| Regex::from_str(k).unwrap()).collect());
	let rc_router = Arc::new(router);
	loop {
		
		let (stream, socket_addr) = listener.accept().await?;
		
		let c_rc_keys = Arc::clone(&rc_keys);
		let c_rc_router = Arc::clone(&rc_router);
		tokio::spawn(async move {
			let _listen_result = listen::<F, O>(stream, c_rc_keys, c_rc_router).await;
		});
	}
	
}

async fn listen<F, O>( mut stream: TcpStream, keys: Arc<Vec<Regex>>, router: Arc<Router<F, O>>) -> Result<(), Box<dyn std::error::Error>>
	where 
		F: Fn(Request) -> O + std::marker::Sync + std::marker::Send + 'static,
		O: Future<Output = ()> +std::marker::Send  + 'static
{
	let mut buffer = [0; 1024];
	let _bytes_read = stream.read(&mut buffer).await?;
	
	//calculating eaders
	let mut headers = [httparse::EMPTY_HEADER; 64];
	let mut req = httparse::Request::new(&mut headers);
	let parse_result = req.parse(&buffer).unwrap();
	let byte_offset = match parse_result {
		httparse::Status::Complete(offset) => Ok(offset),
		httparse::Status::Partial => {
			Err(()) // would be nice if wecould create a response already and return an error with a malformed request
		}
    };
	let content_length = req.headers.iter().find(|h| h.name == "Content-Length")
		.and_then(|h| std::str::from_utf8(h.value).ok())
		.and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);

	//method, path, body, headers
	let method = req.method.unwrap().to_string();
	let path = req.path.unwrap().to_string();
	let body= &buffer[ byte_offset.unwrap()..(byte_offset.unwrap() + content_length)];
	let mut header_map: HeaderMap<HeaderValue> = HeaderMap::new();
	req.headers.iter().for_each(|header_item|{
		let header_name = HeaderName::from_str(header_item.name).unwrap();
		let header_value = HeaderValue::from_bytes(header_item.value).unwrap();
		header_map.insert(header_name, header_value);
	});

	//resolve path
	let mut matched_paths:Vec<String> = Vec::new();
	keys.iter().for_each(|regex_entry|{
		if regex_entry.is_match(&path){
			matched_paths.push(regex_entry.as_str().to_string());
		}
	});
	let match_count = matched_paths.len();
	match match_count {
		0 => {
			//resolve using the database entries prefixes
			println!("return 404")	
		},
		count if ( count == 1) => {
			//only 1 match
			
			if let Some(route) = router.routes.get(&matched_paths[0]).unwrap().get(&method){
				let regex = &route.regex;
				let mut parameter_hash_map = HashMap::<String, String>::new();
				
				regex.captures_iter(req.path.unwrap()).into_iter().for_each(|c| {
					route.parameters.iter().enumerate().for_each(|(index, key)|{
						parameter_hash_map.insert( key.clone(), c[key.as_str()].to_string());
					});			
				});
				println!("{:#?}", parameter_hash_map);
				let req: Request = Request {
					method,
					// body,
					parameters: parameter_hash_map,
					headers: header_map,
					path,
				};
				&(route.closure)(req).await;
				
			}
		},
		_ => {
			//multiple matches
		}
	}
	return Ok(());


}
