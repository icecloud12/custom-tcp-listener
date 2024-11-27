use std::sync::{Arc, OnceLock};

use http::{Response, StatusCode};
use tokio::{self, io::AsyncWriteExt, net::TcpStream, sync::Mutex};
use dotenv::dotenv;
use custom_lib::models::{
	listener, router :: {
		response_to_bytes, Router, Route
	}, types::Request
};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

static STREAMQUEUE:OnceLock<Arc<Mutex<Vec<TcpStream>>>> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
struct HelloWorldBody {
	param1: Option<String>
}

async fn hello_world(request: Request, mut tcpstream: TcpStream){

	
	let body= serde_json::from_slice::<serde_json::Value>(request.body.as_slice());
	println!("Recieved body{:#?}", body);
	//response here
	let response_body: &[u8] = b"Hello World!";
	let response_builder = Response::builder().status(StatusCode::OK);
	let response: Response<&[u8]> = response_builder.header("content-Length", response_body.len())
		.header("Content-Type", "text/json")
		.body(response_body).unwrap();
	let response_bytes = response_to_bytes(response);	
	// let _stream_write = tcpstream.write_all(&response_bytes.as_slice()).await;
	// let _stream_flush = tcpstream.flush().await;
	let mut guard = STREAMQUEUE.get().unwrap().lock().await;
	guard.push(tcpstream);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	dotenv().ok();
	let address: &str = "0.0.0.0:5000";
	let prefix: &str = "/orchestrator/api";
	STREAMQUEUE.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
	let mut router = Router::new();
	router = router.route(format!("{prefix}/handshake/:text:/*"), Route::get(hello_world));
	
	listener::bind(router, address).await?;
	Ok(())
}