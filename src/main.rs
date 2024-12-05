use std::{convert::Infallible, error::Error, future::Future, pin::Pin, sync::{Arc, OnceLock}};

use http::{request, Response, StatusCode};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use tokio::{self, io::AsyncWriteExt, net::{tcp, TcpStream}, sync::Mutex};
use dotenv::dotenv;
use custom_tcp_listener::models::{
	 listener, route::{get, post, ERouterMethod, Route, RouteHandler}, router :: {
		response_to_bytes, Router,
	}, types::Request
};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

static STREAMQUEUE:OnceLock<Arc<Mutex<Vec<TcpStream>>>> = OnceLock::new();

// async fn hello_world(request: Request, mut tcpstream: TcpStream){

	
// 	let body= serde_json::from_slice::<serde_json::Value>(request.body.as_slice());
// 	println!("Recieved body{:#?}", body);
// 	//response here
// 	let response_body: &[u8] = b"Hello World!";
// 	let response_builder = Response::builder().status(StatusCode::OK);
// 	let response: Response<&[u8]> = response_builder.header("content-Length", response_body.len())
// 		.header("Content-Type", "text/json")
// 		.body(response_body).unwrap();
// 	let response_bytes = response_to_bytes(response);	
// 	// let _stream_write = tcpstream.write_all(&response_bytes.as_slice()).await;
// 	// let _stream_flush = tcpstream.flush().await;
// 	let mut guard = STREAMQUEUE.get().unwrap().lock().await;
// 	guard.push(tcpstream);
	
// }

async fn hello_world1(request: Request, mut tcpstream: TcpStream) -> Result<(), Box<dyn std::error::Error>>
{

	let body= serde_json::from_slice::<serde_json::Value>(request.body.as_slice());
	println!("Recieved body{:#?}", body);
	//response here
	let response_body: &[u8] = b"Hello World!1";
	let response_builder = Response::builder().status(StatusCode::OK);
	let response: Response<&[u8]> = response_builder.header("content-Length", response_body.len())
		.header("Content-Type", "text/json")
		.body(response_body)?;
	let response_bytes = response_to_bytes(response);	
	let _stream_write = tcpstream.write_all(&response_bytes.as_slice()).await;
	let _stream_flush = tcpstream.flush().await;
	// let mut guard = STREAMQUEUE.get().unwrap().lock().await;
	// guard.push(tcpstream);

	Ok(())
}
async fn hello_world2(request: Request, mut tcpstream: TcpStream) -> Result<(), Box<dyn std::error::Error>>
{
	
	let uri = "mongodb://localhost:27017/";
	let client = Client::with_uri_str(uri).await.unwrap();
	let database = client.database("orchestrator");
    let my_coll: Collection<Image> = database.collection("images");
    let mut my_image= my_coll.find(doc!{}).await.unwrap();
	while my_image.advance().await? {
		let a = my_image.deserialize_current();
		if a.is_ok(){
			println!("{:?}", a);
		};
	}
	
	Ok(())
}
#[derive(Debug, Deserialize)]
struct Image {
	_id: ObjectId,
	docker_image_id: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	
	dotenv().ok();
	let address: &str = "0.0.0.0:5000";
	let prefix: &str = "/orchestrator/api";
	// STREAMQUEUE.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
	let mut router = Router::new();
	router = router.route(format!("{prefix}/handshake/:text:/*"), get(hello_world1));
	router = router.route(format!("{prefix}/handshake/:text:/*"), post(hello_world2));
	listener::bind(router, address).await?;
	

	// let b = Box::pin(|request, tcp_stream| {
	// 	hello_world2(request, tcp_stream)
	// }) as Pin<Box<dyn Future<Output = ()>>>;
	
	Ok(())
}