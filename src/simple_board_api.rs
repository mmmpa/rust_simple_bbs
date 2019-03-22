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
use self::iron::method::Method;
use crate::route_action::RouteAction;
use crate::url_separation::Matcher;

pub struct SimpleBoardApi {
    gateway: DataGateway,
}

impl SimpleBoardApi {
    pub fn start(gateway: DataGateway) {
        let mut router: Router<Arc<RwLock<CustomHandler>>> = Router::new(gateway);

        router.add_route(
            Method::Get,
            "ello",
            |c: &mut Context, req: &mut Request| {
                println!("{:?}", req.get::<bodyparser::Raw>());
                Ok(Response::with((status::Ok, "Hello world !")))
            }
        );

        router.add_route(Method::Post, "boards", RouteAction::create_board);

        Iron::new(router).http("localhost:3000");
    }
}

trait CustomHandler: Send + Sync + 'static {
    fn handle(&self, c: &mut Context, req: &mut Request) -> IronResult<Response>;
}

impl<F> CustomHandler for F where F: Send + Sync + 'static + Fn(&mut Context, &mut Request) -> IronResult<Response> {
    fn handle(&self, c: &mut Context, req: &mut Request) -> IronResult<Response> {
        (*self)(c, req)
    }
}

type RouteMap = HashMap<
    Method,
    HashMap<
        String,
        Box<CustomHandler>
    >,
>;

type PassedGateway = Arc<RwLock<DataGateway>>;

struct Router<T: 'static + Clone + Send + Sync> {
    gateway: PassedGateway,
    routes: RouteMap,
    matcher: Matcher<T>,
}

pub struct Context {
    pub gateway: PassedGateway,
    pub route_params: HashMap<String, String>,
}

impl<T: 'static + Clone + Send + Sync> Router<T> {
    fn new(gateway: DataGateway) -> Self {
        let mut routes = HashMap::new();
        routes.insert(Method::Get, HashMap::new());
        routes.insert(Method::Post, HashMap::new());
        routes.insert(Method::Patch, HashMap::new());
        routes.insert(Method::Delete, HashMap::new());

        Router {
            gateway: Arc::new(RwLock::new(gateway)),
            routes,
            matcher: Matcher::new(Some(Arc::new(RwLock::new("".to_string())))),
        }
    }

    fn add_route<H: CustomHandler>(&mut self, method: Method, path: &str, handler: H) {
        let path_routes = self.routes.get_mut(&method).unwrap();
        path_routes.insert(path.to_string(), Box::new(handler));
    }
}

impl<T: 'static + Clone + Send + Sync> Handler for Router<T> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let route_params = HashMap::new();

        match self.routes.get(&req.method) {
            None => Ok(Response::with(status::NotFound)),
            Some(path_routes) => {
                match path_routes.get(&req.url.path().join("/")) {
                    None => Ok(Response::with(status::NotFound)),
                    Some(handler) => {
                        handler.handle(&mut Context { route_params, gateway: self.gateway.clone() }, req)
                    },
                }
            },
        }
    }
}
