use std::{error::Error, path::PathBuf, sync::Arc};

use custom_tcp_listener::models::{listener::bind, route::get, router::Router, types::Request};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn hello_world(
    _request: Request,
    _tcp_stream: TlsStream<TcpStream>,
    _decoration: Arc<Test>,
) -> Result<(), Box<dyn Error>> {
    println!("entering hello_world");
    println!("{:#?}", _decoration);
    Ok(())
}
#[derive(Debug)]
pub struct Test {
    field: Arc<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let decorations = Test { field: Arc::new(5) };
    let router = Router::new().route("/*".to_string(), get(hello_world));
    let _bind_result = bind::<Test>(
        router,
        format!("{}:{}", "192.168.3.10", 3000).as_str(),
        PathBuf::from("./src/certs/cert.crt"),
        PathBuf::from("./src/certs/cert.key"),
        decorations,
    )
    .await;
    println!("{:#?}", _bind_result);
    Ok(())
}
