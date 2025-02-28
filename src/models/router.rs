use super::route::{ERouterMethod, Route, RouteHandler};
use http::Response;
use regex::{Regex, RegexBuilder};
use std::{collections::HashMap, future::Future, ops::Range, pin::Pin, sync::Arc};
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub fn response_to_bytes(response: Response<Vec<u8>>) -> Vec<u8> {
    let (parts, body) = response.into_parts();
    let mut _response_line = format!("{:#?} {}\r\n", parts.version, parts.status);
    parts
        .headers
        .iter()
        .for_each(|(header_name, header_value)| {
            _response_line = _response_line.clone()
                + &format!(
                    "{}:{}\r\n",
                    header_name.as_str(),
                    header_value.to_str().unwrap()
                );
        });
    let formatted_response = format!(
        "{}\r\n{}",
        _response_line,
        String::from_utf8(body.to_vec()).unwrap()
    );
    formatted_response.as_bytes().to_vec()
}

pub struct Router<K>
where
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    pub routes: HashMap<String, HashMap<String, Route<K>>>,
    pub keys: Vec<String>,
}
impl<K> Router<K>
where
    K: 'static + Send,
    Arc<K>: 'static + Send,
{
    pub fn new() -> Router<K> {
        let mut hm: HashMap<String, HashMap<String, Route<K>>> = HashMap::new();
        Router {
            routes: hm,
            keys: vec![],
        }
    }
    pub fn route(self, path: String, route_helper: (ERouterMethod, RouteHandler<K>)) -> Router<K> {
        let mut hm = self.routes;
        let mut keys = self.keys;
        let (regex_path, path_parameters) = Router::extract_path_parameter(&path);

        let method_route_pairs = match hm.get_mut(&regex_path.as_str().to_string()) {
            Some(p) => p,
            None => {
                let new_val = HashMap::<String, Route<K>>::new();
                hm.insert(regex_path.as_str().to_string(), new_val);
                hm.get_mut(&regex_path.as_str().to_string()).unwrap()
            }
        };
        if !keys.contains(&regex_path.as_str().to_string()) {
            keys.push(regex_path.as_str().to_string());
        }

        method_route_pairs.insert(
            route_helper.0.as_str().to_string(),
            Route {
                method: route_helper.0,
                path,
                handler: route_helper.1,
                parameters: path_parameters,
                regex: regex_path.clone(),
            },
        );

        Router { routes: hm, keys }
    }
    fn extract_path_parameter(path: &String) -> (Regex, Vec<String>) {
        let re = Regex::new(r":([^\/]*):").unwrap();
        let mut route_regex: String = path.clone().replace("/", "\\/");
        let mut ranges: Vec<Range<usize>> = vec![];
        //let mut path_parameter_map: HashMap<&String, String> = HashMap::new();
        let path_parameters = re
            .captures_iter(&route_regex.as_str())
            .filter_map(|fm| {
                fm.get(1).map(|m| {
                    ranges.push(m.range());
                    m.as_str().to_string()
                })
            })
            .collect::<Vec<String>>();
        ranges.iter().enumerate().rev().for_each(|(index, r)| {
            route_regex.replace_range(
                r.start - 1..r.end + 1,
                format!("(?<{}>[^\\/]*)", path_parameters[index]).as_str(),
            );
        });

        if route_regex.ends_with("\\/*") {
            let len = route_regex.len();
            route_regex.replace_range(len - 1..len, "(.+)");
        }
        let created_regex = RegexBuilder::new(route_regex.as_str()).build().unwrap();
        (created_regex, path_parameters)
    }
}
