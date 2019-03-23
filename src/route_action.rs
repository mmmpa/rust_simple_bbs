use serde::{Deserialize};

use bodyparser;

use iron::prelude::*;
use iron::status;
use crate::router::RouteContext;
use crate::to_json::ToJson;

pub struct RouteAction;

#[derive(Debug, Clone, Deserialize)]
struct BoardCreationParams {
    title: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ThreadCreationParams {
    title: String,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MessageCreationParams {
    message: String,
}

impl RouteAction {
    pub fn show_root(_c: &mut RouteContext, _req: &mut Request<'_, '_>) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "root")))
    }

    pub fn show_board(c: &mut RouteContext, _req: &mut Request<'_, '_>) -> IronResult<Response> {
        let board_id = c.params.get("board_id").unwrap();

        match c.api.show_board(board_id) {
            Err(m) => bad_params(m),
            Ok(board) => Ok(Response::with((status::Ok, board.to_json()))),
        }
    }

    pub fn show_thread(c: &mut RouteContext, _req: &mut Request<'_, '_>) -> IronResult<Response> {
        let board_id = c.params.get("board_id").unwrap();
        let thread_id = c.params.get("thread_id").unwrap();

        match c.api.show_thread(board_id, thread_id, 0..10000) {
            Err(m) => bad_params(m),
            Ok(thread) => Ok(Response::with((status::Ok, thread.to_json()))),
        }
    }

    pub fn create_board(c: &mut RouteContext, req: &mut Request<'_, '_>) -> IronResult<Response> {
        let body = match req.get::<
            bodyparser::Struct<BoardCreationParams>
        >() {
            Ok(body) => body.unwrap(),
            Err(_m) => return invalid_params()
        };

        match c.api.create_board(&body.title) {
            Ok(id) => Ok(Response::with((status::Ok, id))),
            Err(m) => bad_params(m)
        }
    }

    pub fn create_thread(c: &mut RouteContext, req: &mut Request<'_, '_>) -> IronResult<Response> {
        let board_id = c.params.get("board_id").unwrap();

        let ThreadCreationParams {
            title,
            message,
        } = match req.get::<
            bodyparser::Struct<ThreadCreationParams>
        >() {
            Ok(body) => body.unwrap(),
            Err(_m) => return invalid_params()
        };

        match c.api.create_thread(&board_id, &title, &message) {
            Ok(id) => Ok(Response::with((status::Ok, id))),
            Err(m) => bad_params(m)
        }
    }

    pub fn create_message(c: &mut RouteContext, req: &mut Request<'_, '_>) -> IronResult<Response> {
        let board_id = c.params.get("board_id").unwrap();
        let thread_id = c.params.get("thread_id").unwrap();

        let MessageCreationParams {
            message,
        } = match req.get::<
            bodyparser::Struct<MessageCreationParams>
        >() {
            Ok(body) => body.unwrap(),
            Err(_m) => return invalid_params()
        };

        match c.api.create_message(&board_id, &thread_id, &message) {
            Ok(_) => Ok(Response::with(status::Ok)),
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
