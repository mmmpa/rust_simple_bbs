use crate::simple_board_api::SimpleBoardApi;
use crate::data_gateway::DataGateway;
use crate::test_adapter::TestAdapter;
use std::sync::{Arc, RwLock};
use crate::json_adapter::JsonAdapter;

mod thread_message;
mod board;
mod board_thread;
mod message_arrangement;
mod data_gateway;
mod data_gateway_adapter;
mod test_adapter;
mod json_adapter;
mod simple_board_api;
mod route_action;
mod router;
mod url_separation;

fn main() {
    let g = DataGateway::new(
        Box::new(JsonAdapter::new("real_run", false))
    );

    SimpleBoardApi::start(g);
}
