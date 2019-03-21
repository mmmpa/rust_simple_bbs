extern crate iron;

use iron::prelude::*;
use iron::status;
use crate::data_gateway::DataGateway;
use iron::headers::ContentType;
use self::iron::Handler;
use std::collections::HashMap;
use hyper::StatusCode;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub struct SimpleBoardApi {
    gateway: DataGateway,
}

impl SimpleBoardApi {
    pub fn start(gateway: Arc<RwLock<DataGateway>>) {
        let mut router = Router::new(gateway);

        router.add_route("hello".to_string(), |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello world !")))
        });

        router.add_route("hello/again".to_string(), |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello again !")))
        });

        router.add_route("error".to_string(), |_: &mut Request| {
            Ok(Response::with(status::BadRequest))
        });

        Iron::new(router).http("localhost:3000");
    }
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let content = "true";
    Ok(Response::with((ContentType::json().0, status::Ok, content)))
}

struct Router {
    gateway: Arc<RwLock<DataGateway>>,
    routes: HashMap<String, Box<Handler>>,
}

struct Context {
    gateway: Arc<RwLock<DataGateway>>,
}

impl Router {
    fn new(gateway: Arc<RwLock<DataGateway>>) -> Self {
        Router {
            gateway: gateway.clone(),
            routes: HashMap::new(),
        }
    }

    fn add_route<H>(&mut self, path: String, handler: fn(Context) -> H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler(Context { gateway: self.gateway.clone() }).handle(req),
            None => Ok(Response::with((status::NotFound, "not found"))),
        }
    }
}
