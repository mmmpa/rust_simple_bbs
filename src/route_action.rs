use serde::{Deserialize, Serialize};

extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use crate::router::RouteContext;

pub struct RouteAction;

#[derive(Debug, Clone, Deserialize)]
struct BoardCreationParams {
    title: String,
}

impl RouteAction {
    pub fn create_board(c: &mut RouteContext, req: &mut Request) -> IronResult<Response> {
        let body = match req.get::<bodyparser::Struct<BoardCreationParams>>() {
            Ok(body) => body.unwrap(),
            Err(m) => return invalid_params()
        };

        match c.api.write().unwrap().create_board(&body.title) {
            Ok(id) => Ok(Response::with((status::Ok, id))),
            Err(m) => bad_params(m)
        }
    }

    pub fn show_board(c: &mut RouteContext, req: &mut Request) -> IronResult<Response> {
        let board_id = c.params.get("board_id").unwrap();

        match c.api.read().unwrap().show_board(board_id) {
            Err(m) => bad_params(m),
            Ok(board) => {
                Ok(Response::with((status::Ok, format!("{:?}", board))))
            },
        }
    }
}

fn invalid_params() -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, "invalid params format")))
}

fn bad_params(message: String) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, message)))
}
