use regex::Regex;
use std::error::Error;
use std::sync::Arc;
use std::{future::Future, pin::Pin};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

use super::types::Request;

//#region ERouterMethod
pub enum ERouterMethod {
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

pub type RouteHandler<K> = Box<
    dyn Fn(
            Request,
            TlsStream<TcpStream>,
            Arc<K>,
        ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send>>
        + Send
        + Sync,
>;

//#region

pub struct Route<K> {
    pub method: ERouterMethod,
    pub path: String,
    pub handler: RouteHandler<K>,
    pub parameters: Vec<String>,
    pub regex: Regex,
}

pub fn connect<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::CONNECT,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}

pub fn get<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::GET,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}

pub fn delete<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::DELETE,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}
pub fn head<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::HEAD,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}
pub fn option<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::OPTIONS,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}
pub fn patch<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::PATCH,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}
pub fn post<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::POST,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}
pub fn put<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::PUT,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
}

pub fn trace<T, K>(
    handler: fn(Request, TlsStream<TcpStream>, Arc<K>) -> T,
) -> (ERouterMethod, RouteHandler<K>)
where
    T: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    (
        ERouterMethod::TRACE,
        Box::new(move |request, tcp_stream, decoration| {
            Box::pin(handler(request, tcp_stream, decoration))
        }),
    )
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
    fn as_slice() -> [ERouterMethod; 9] {
        [
            ERouterMethod::CONNECT,
            ERouterMethod::DELETE,
            ERouterMethod::GET,
            ERouterMethod::HEAD,
            ERouterMethod::OPTIONS,
            ERouterMethod::PATCH,
            ERouterMethod::POST,
            ERouterMethod::PUT,
            ERouterMethod::TRACE,
        ]
    }
}
