use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use httparse;
use http::{HeaderMap, HeaderName, HeaderValue};
use regex::Regex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

use super::router::Router;
use super::types::Request;




pub async fn bind(router:Router, address:&str) -> Result<(), Box<dyn std::error::Error>>
{
	let listener = TcpListener::bind(address).await?;
	let rc_keys:Arc<Vec<Regex>> = Arc::new(router.keys.iter().map(|k| Regex::from_str(k).unwrap()).collect());
	let rc_router = Arc::new(router);
	loop {
		
		let (stream, socket_addr) = listener.accept().await?;
		
		let c_rc_keys = Arc::clone(&rc_keys);
		let c_rc_router = Arc::clone(&rc_router);
		
		tokio::spawn(async move {
			let _listen_result = listen(stream, c_rc_keys, c_rc_router).await;
		});
	}
	
}


async fn listen( mut stream: TcpStream, keys: Arc<Vec<Regex>>, router: Arc<Router>) -> Result<(), Box<dyn std::error::Error>>
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
	let mut maximum_path_segments: usize = 0;
	let mut selected_route: Option<String> = None;
	keys.iter().for_each(|regex_entry|{
		if regex_entry.is_match(&path){
			//regex matched
			if let Some(route) = router.routes.get(&regex_entry.as_str().to_string()).unwrap().get(&method) {
				let current_segments_count = route.path.split("/").count();
				if current_segments_count > maximum_path_segments {
					maximum_path_segments = current_segments_count;
					selected_route = Some(regex_entry.as_str().to_string());
				}
			}
		}
	});
	match selected_route {
		None => {
			//resolve using the database entries prefixes
			println!("return 404")	
		},
		Some(path) => {
			if let Some(route) = router.routes.get(&path).unwrap().get(&method){
				let regex = &route.regex;
				let mut parameter_hash_map = HashMap::<String, String>::new();
				
				regex.captures_iter(req.path.unwrap()).into_iter().for_each(|c| {
					route.parameters.iter().enumerate().for_each(|(index, key)|{
						parameter_hash_map.insert( key.clone(), c[key.as_str()].to_string());
					});			
				});
				let req: Request = Request {
					method,
					body: body.to_vec(),
					parameters: parameter_hash_map,
					headers: header_map,
					path,
				};
				let handler = route.handler.deref();
				let _ = handler(req, stream).await;
				// let _ = (route.handler)(req,stream).await;
				
			}
		},
		
	}
	return Ok(());


}
