use std::{convert::Infallible, future::Future, pin::Pin};
use regex::Regex;
use std::error::Error;
use tokio::net::TcpStream;

use super::types::Request;

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

pub type RouteHandler = Box<dyn Fn(Request, TcpStream) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send >> + Send + Sync>;

//#region

pub struct Route
{
	pub method: ERouterMethod,
	pub path: String,
	pub handler: RouteHandler,
	pub parameters: Vec<String>,
	pub regex: Regex
}


pub fn connect<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::CONNECT, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}

pub fn get<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::GET, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
pub fn delete<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::DELETE, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
pub fn head<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::HEAD, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
pub fn option<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::OPTIONS, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
pub fn patch<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::PATCH, Box::new( move |request, tcp_stream: TcpStream| Box::pin(handler(request, tcp_stream))))
}
pub fn post<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::POST, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
pub fn put<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send
{
	(ERouterMethod::PUT, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}

pub fn trace<T>(handler: fn(Request, TcpStream) -> T) -> (ERouterMethod, RouteHandler) 
where
	T: Future<Output = Result<(), Box<dyn Error>>>+ 'static + Send
{
	(ERouterMethod::TRACE, Box::new( move |request, tcp_stream| Box::pin(handler(request, tcp_stream))))
}
//#endregion

impl ERouterMethod {
	pub fn as_str(&self) -> &'static str {
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