use std::{error::Error, path::PathBuf};

use custom_tcp_listener::models::{listener::bind, route::get, router::Router, types::Request};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn hello_world(_request: Request, _tcp_stream: TlsStream<TcpStream>) -> Result<(), Box<dyn Error>> {
    println!("entering hello_world");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new().route("/*".to_string(), get(hello_world));
    let _bind_result = bind(
        router,
        format!("{}:{}", "192.168.254.106", 3000).as_str(),
        PathBuf::from("./src/certificates/cert.crt"),
        PathBuf::from("./src/certificates/cert.key"),
    )
    .await;
    println!("{:#?}", _bind_result);
    Ok(())
}
