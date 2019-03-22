use iron;

use iron::prelude::*;
use iron::status;
use crate::data_gateway::DataGateway;
use self::iron::Handler;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use self::iron::method::Method;
use crate::url_separation::Matcher;
use std::hash::Hash;

type Api = Arc<RwLock<DataGateway>>;

type Routes<T> = HashMap<
    Method,
    HashMap<
        T,
        Box<dyn CustomHandler>
    >,
>;

type Params = HashMap<String, String>;

pub struct Router<T: 'static + Clone + Send + Sync + Hash + Eq> {
    api: Api,
    routes: Routes<T>,
    matcher: Matcher<T>,
}

pub struct RouteContext {
    pub api: Api,
    pub params: Params,
}

impl<T: 'static + Clone + Send + Sync + Hash + Eq> Router<T> {
    pub fn new(gateway: DataGateway) -> Self {
        let mut routes = HashMap::new();
        routes.insert(Method::Get, HashMap::new());
        routes.insert(Method::Post, HashMap::new());
        routes.insert(Method::Patch, HashMap::new());
        routes.insert(Method::Delete, HashMap::new());

        Router {
            api: Arc::new(RwLock::new(gateway)),
            routes,
            matcher: Matcher::new(None),
        }
    }

    pub fn add_route<H: CustomHandler>(&mut self, method: Method, matching: T, path: &str, handler: H) {
        let path_routes = self.routes.get_mut(&method).unwrap();
        path_routes.insert(matching.clone(), Box::new(handler));
        self.matcher.add(path, matching.clone());
    }
}

impl<T: 'static + Clone + Send + Sync + Hash + Eq> Handler for Router<T> {
    fn handle(&self, req: &mut Request<'_, '_>) -> IronResult<Response> {
        match self.matcher.pick(&req.url.path()) {
            None => Ok(Response::with(status::NotFound)),
            Some((key, params)) => {
                match self.routes.get(&req.method).unwrap().get(&key) {
                    None => Ok(Response::with(status::NotFound)),
                    Some(handler) => {
                        let api = self.api.clone();
                        let mut context = RouteContext { params, api };

                        handler.handle(&mut context, req)
                    },
                }
            }
        }
    }
}

pub trait CustomHandler: Send + Sync + 'static {
    fn handle(&self, c: &mut RouteContext, req: &mut Request<'_, '_>) -> IronResult<Response>;
}

impl<F> CustomHandler for F where F: Send + Sync + 'static + Fn(&mut RouteContext, &mut Request<'_, '_>) -> IronResult<Response> {
    fn handle(&self, c: &mut RouteContext, req: &mut Request<'_, '_>) -> IronResult<Response> {
        (*self)(c, req)
    }
}
