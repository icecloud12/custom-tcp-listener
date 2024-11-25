
use std::{error::Error, future::Future, pin::Pin, process::Output};

use http::request;
use regex::Regex;
use tokio;
use dotenv::dotenv;

use custom_lib::models::{
	listener, router :: {
		get, ERouterMethod, Route, Router
	}, types::Request
};

pub struct ActionManager<F, O>
where
    F: Fn(Request) -> O + std::marker::Sync + std::marker::Send + 'static,
	O: Future<Output = ()> +std::marker::Send  + 'static
{
    pub action: F,
}

async fn test(request: Request){
	println!("in test async function");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	dotenv().ok();
	let address: &str = "192.168.254.106:5000";
	let prefix: &str = "/orchestrator/api";

	let am = ActionManager{
		action : test
	};

	let mut router = Router::new();
	router = router.route(format!("{prefix}/handshake/:text:/*"), get(test));
	
	listener::bind(router, address).await?;
	

	Ok(())
}