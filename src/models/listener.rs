use http::{HeaderMap, HeaderName, HeaderValue};
use httparse;
use regex::Regex;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

use super::router::Router;
use super::types::Request;

pub async fn bind<Decoration>(
    router: Router<Decoration>,
    address: &str,
    certificate_der: impl AsRef<std::path::Path>,
    certificate_key: impl AsRef<std::path::Path>,
    decoration: Decoration,
) -> Result<(), Box<dyn std::error::Error>>
where
    Decoration: 'static + Send,
    Arc<Decoration>: 'static + Send,
{
    let certs = CertificateDer::pem_file_iter(certificate_der)?.collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_file(certificate_key)?;
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(address).await?;
    let rc_keys: Arc<Vec<Regex>> = Arc::new(
        router
            .keys
            .iter()
            .map(|k| Regex::from_str(k).unwrap())
            .collect(),
    );
    let rc_router = Arc::new(router);
    let arc_decoration: Arc<Decoration> = Arc::new(decoration);
    loop {
        let (stream, _socket_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let stream_accept = acceptor.accept(stream).await;
        match stream_accept {
            Ok(stream) => {
                let c_rc_keys = Arc::clone(&rc_keys);
                let c_rc_router = Arc::clone(&rc_router);
                let c_arc_decoration = Arc::clone(&arc_decoration);
                tokio::spawn(async move {
                    let _listen_result =
                        listen::<Decoration>(stream, c_rc_keys, c_rc_router, c_arc_decoration)
                            .await;
                });
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}

async fn listen<Decoration>(
    mut stream: TlsStream<TcpStream>,
    keys: Arc<Vec<Regex>>,
    router: Arc<Router<Decoration>>,
    decoration: Arc<Decoration>,
) -> Result<(), Box<dyn std::error::Error>>
where
    Decoration: 'static + Send,
    Arc<Decoration>: 'static + Send,
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
    let content_length = req
        .headers
        .iter()
        .find(|h| h.name == "Content-Length")
        .and_then(|h| std::str::from_utf8(h.value).ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    //method, path, body, headers
    let method = req.method.unwrap().to_string();
    let path = req.path.unwrap().to_string();

    let body = match byte_offset {
        Ok(b) => buffer[byte_offset.unwrap()..(byte_offset.unwrap() + content_length)].to_vec(),
        Err(e) => vec![],
    };
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::new();
    req.headers.iter().for_each(|header_item| {
        let header_name = HeaderName::from_str(header_item.name).unwrap();
        let header_value = HeaderValue::from_bytes(header_item.value).unwrap();
        header_map.insert(header_name, header_value);
    });

    //resolve path
    let mut maximum_path_segments: usize = 0;
    let mut selected_route: Option<String> = None;
    keys.iter().for_each(|regex_entry| {
        if regex_entry.is_match(&path) {
            //regex matched
            if let Some(route) = router
                .routes
                .get(&regex_entry.as_str().to_string())
                .unwrap()
                .get(&method)
            {
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
        }
        Some(path) => {
            if let Some(route) = router.routes.get(&path).unwrap().get(&method) {
                let regex = &route.regex;
                let mut parameter_hash_map = HashMap::<String, String>::new();
                let request_path = req.path.unwrap();
                regex.captures_iter(request_path).into_iter().for_each(|c| {
                    route
                        .parameters
                        .iter()
                        .enumerate()
                        .for_each(|(_index, key)| {
                            parameter_hash_map.insert(key.clone(), c[key.as_str()].to_string());
                        });
                });
                let req: Request = Request {
                    method,
                    body,
                    parameters: parameter_hash_map,
                    headers: header_map,
                    path: request_path.to_string(),
                };
                let handler = route.handler.deref();
                let _ = handler(req, stream, decoration).await;
                // let _ = (route.handler)(req,stream).await;
            }
        }
    }
    return Ok(());
}
