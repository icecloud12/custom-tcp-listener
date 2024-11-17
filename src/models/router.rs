use std::{collections::HashMap, slice::Iter, sync::Arc};
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


pub struct Route {
	pub method: ERouterMethod,
	pub path: String,
	pub closure: fn(),
}



impl ERouterMethod {
	fn as_str(&self) -> &'static str {
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
pub struct Router {
	routes : Arc<HashMap<String, HashMap<String, Option<fn()>>>>
}
impl Router{
	fn new(&self) -> Router{
		let mut hm: HashMap<String, HashMap<String, Option<fn()>>> = HashMap::new();
		Router {
    		routes: Arc::new(hm ),
		}		
		// let mut hm = HashMap::new();
		// ERouterMethod::as_slice().iter().for_each( |router_method| {
		// 	hm.insert(router_method.as_str(), Arc::new(
		// 		Vec::<Route>::new()
		// 	));
		// });
	}
	fn route(self, method: ERouterMethod, path:String, closure: fn()){
		let mut hm = Arc::try_unwrap(self.routes).unwrap();
		let method_closure_pairs:&mut HashMap<String, Option<fn()>>  = match hm.get_mut(&path) {
			Some(p) => p,
			None => {
				let new_val = HashMap::<String, Option<fn()>>::new();
				hm.insert(path.clone(), new_val);
				hm.get_mut(&path).unwrap()
			}
		};
		method_closure_pairs.insert(method.as_str().to_string(), Some(closure));
	}
	// 	// match Arc::try_unwrap(self.routes){
	// 	// 	Ok( _v ) => {
	// 	// 		let mut v = _v;
	// 	// 		v.push(Route { method, path, closure });
	// 	// 		Router {
	// 	// 			routes : Arc::new(v)
	// 	// 		}
	// 	// 	},
	// 	// 	Err( _ ) => {
	// 	// 		panic!("Cannot unwrap arc route")
	// 	// 	}
	// 	// }
	// }
}

