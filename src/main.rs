use crate::server::Server;
use crate::data_gateway::DataGateway;
use crate::json_adapter::JsonAdapter;

mod thread_message;
mod board;
mod board_thread;
mod message_arrangement;
mod data_gateway;
mod data_gateway_adapter;
mod test_adapter;
mod json_adapter;
mod server;
mod route_action;
mod router;
mod url_separation;
mod common_error;
mod to_json;

#[cfg(test)]
mod e2e_test;

fn main() {
    let adapter = JsonAdapter::new("real_run", false);
    let gateway = DataGateway::new(Box::new(adapter));

    Server::start(3000, gateway);
}
