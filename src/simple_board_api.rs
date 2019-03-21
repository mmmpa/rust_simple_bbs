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
use std::sync::mpsc::channel;

pub struct SimpleBoardApi {
    gateway: DataGateway,
}

impl SimpleBoardApi {
    pub fn start(gateway: DataGateway) {
        let mut router = Router::new(gateway);

        router.add_route("hello".to_string(), |c: &Context, _: &mut Request| {
            Ok(Response::with((status::Ok, "Hello world !")))
        });

        Iron::new(router).http("localhost:3000");
    }
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let content = "true";
    Ok(Response::with((ContentType::json().0, status::Ok, content)))
}

trait CustomHandler: Send + Sync + 'static {
    fn handle(&self, c: &Context, req: &mut Request) -> IronResult<Response>;
}

impl<F> CustomHandler for F
    where F: Send + Sync + 'static + Fn(&Context, &mut Request) -> IronResult<Response>
{
    fn handle(&self, c: &Context, req: &mut Request) -> IronResult<Response> {
        (*self)(c, req)
    }
}

struct Router {
    gateway: DataGateway,
    routes: HashMap<String, Box<CustomHandler>>,
}

struct Context {}

impl Router {
    fn new(gateway: DataGateway) -> Self {
        Router {
            gateway: gateway,
            routes: HashMap::new(),
        }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
        where H: CustomHandler
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(&Context {}, req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}
