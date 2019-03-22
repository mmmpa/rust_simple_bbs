use serde::{Deserialize, Serialize};
use crate::simple_board_api::Context;

extern crate bodyparser;

use iron::prelude::*;
use iron::status;

pub struct RouteAction;

#[derive(Debug, Clone, Deserialize)]
struct BoardCreationParams {
    title: String,
}

impl RouteAction {
    pub fn create_board(c: &mut Context, req: &mut Request) -> IronResult<Response> {
        let body = match req.get::<bodyparser::Struct<BoardCreationParams>>() {
            Ok(body) => body.unwrap(),
            Err(m) => return invalid_params()
        };

        match c.gateway.write().unwrap().create_board(&body.title) {
            Ok(id) => Ok(Response::with((status::Ok, id))),
            Err(m) => bad_params(m)
        }
    }

    pub fn show_board(c: &mut Context, req: &mut Request) -> IronResult<Response> {
        let body = match req.get::<bodyparser::Struct<BoardCreationParams>>() {
            Ok(body) => body.unwrap(),
            Err(m) => return invalid_params()
        };

        println!("{:?}", body);

        match c.gateway.write().unwrap().create_board(&body.title) {
            Ok(id) => Ok(Response::with((status::Ok, id))),
            Err(m) => bad_params(m)
        }
    }
}

fn invalid_params() -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, "invalid params format")))
}

fn bad_params(message: String) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, message)))
}
