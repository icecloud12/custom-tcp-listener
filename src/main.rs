
use std::{error::Error, future::Future, pin::Pin, process::Output};

use http::{request, response, Response};
use regex::Regex;
use tokio::{self, io::AsyncWriteExt, net::TcpStream};
use dotenv::dotenv;

use custom_lib::models::{
	listener, router :: {
		get, ERouterMethod, Route, Router
	}, types::Request
};

async fn hello_world(request: Request, mut tcpstream: TcpStream){
	
	let response_body = b"Hello World!";
	let response_builder = Response::builder().status(200);
	let response = response_builder.header("content-Length", response_body.len())
		.header("Content-Type", "text/json")
		.body(response_body).unwrap();

	let (parts, body) = response.into_parts();
	let mut response_line = format!("{:#?} {}", parts.version, parts.status);
	parts.headers.iter().for_each(
		|(header_name, header_value)|{
			response_line = response_line.clone() + "\r\n"+ &format!("{}:{}", header_name.as_str(), header_value.to_str().unwrap());
		}		 
	);
	response_line = format!("{}\r\n\r\n{}", response_line,String::from_utf8(body.to_vec()).unwrap());
	let _stream_write = tcpstream.write_all(response_line.as_bytes()).await;
	let _stream_flush = tcpstream.flush().await;

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	dotenv().ok();
	let address: &str = "0.0.0.0:5000";
	let prefix: &str = "/orchestrator/api";

	

	let mut router = Router::new();
	router = router.route(format!("{prefix}/handshake/:text:/*"), get(hello_world));
	
	listener::bind(router, address).await?;
	Ok(())
}