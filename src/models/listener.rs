use std::str::FromStr;
use httparse;
use http::{HeaderMap, HeaderName, HeaderValue};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};

use super::types::Request;


pub async fn bind(address:&str) -> Result<(), Box<dyn std::error::Error>>{
	let listener = TcpListener::bind(address).await?;
	loop {
		let (stream, _addr) = listener.accept().await?;
		tokio::spawn(async move {
			let _listen_result = listen(stream).await;
		});
	}
	return Ok(())
}

async fn listen( mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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

	let body: &[u8] = &buffer[ byte_offset.unwrap()..(byte_offset.unwrap() + content_length)];
	let body_str: std::borrow::Cow<str> = String::from_utf8_lossy(body);
	let mut header_map: HeaderMap<HeaderValue> = HeaderMap::new();
	req.headers.iter().for_each(|header_item|{
		let header_name = HeaderName::from_str(header_item.name).unwrap();
		let header_value = HeaderValue::from_bytes(header_item.value).unwrap();
		header_map.insert(header_name, header_value);
	});

	let path = req.path.unwrap().to_string();
	//using the said path use it against the router
	

	return Ok(())
}
