extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, Request, IronResult, IronError, AfterMiddleware, Response, typemap};
use unicase::UniCase;
use iron::headers::{AccessControlAllowOrigin, AccessControlAllowHeaders};
use time::precise_time_ns;

pub struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        set_headers(&mut res);
        Ok(res)
    }
}

fn set_headers(res: &mut Response) {
    res.headers.set(AccessControlAllowOrigin::Any);
    res.headers.set(
        AccessControlAllowHeaders(vec![
            UniCase("Content-Type".to_owned()),
            UniCase("date".to_owned()),
        ])
    );
}

pub struct Logger;

impl typemap::Key for Logger { type Value = u64; }

impl BeforeMiddleware for Logger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Logger>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for Logger {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Logger>().unwrap();
        println!("{} ms - {:?}: {:?}", (delta as f64) / 1000000.0, req.method, req.url);
        Ok(res)
    }
}
