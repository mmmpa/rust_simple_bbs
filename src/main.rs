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

fn main() {
    let adapter = JsonAdapter::new("real_run", false);
    let gateway = DataGateway::new(Box::new(adapter));

    Server::start(gateway);
}

#[cfg(test)]
mod tests {
    use std::thread::spawn;
    use crate::server::Server;
    use crate::data_gateway::DataGateway;
    use crate::json_adapter::JsonAdapter;
    use std::{thread, time};
    use std::collections::HashMap;
    use crate::board::Board;
    use crate::board_thread::BoardThread;

    const HOST: &str = "http://localhost:3000";
    const BOARD: &str = "http://localhost:3000/b";

    #[test]
    fn test_e2e() {
        spawn(|| {
            let adapter = JsonAdapter::new("e2e_test", true);
            let gateway = DataGateway::new(Box::new(adapter));

            Server::start(gateway);
        });

        let res = reqwest::get(HOST).unwrap().text().unwrap();
        assert_eq!(res, "root");

        let board_id = create_board("new board");

        let board: Board = get_board(&board_id);
        assert_eq!(board.title, "new board");

        let thread_id = create_thread(&board_id, "new thread", "hello new thread");

        let thread = get_thread(&board_id, &thread_id);
        assert_eq!(thread.title, "new thread");

        println!("{:?}", res)
    }

    fn get_board(board_id: &str) -> Board {
        let res = reqwest::get(&build_board_url(&board_id)).unwrap().text().unwrap();
        serde_json::from_str(&res).unwrap()
    }

    fn get_thread(board_id: &str, thread_id: &str) -> BoardThread {
        let res = reqwest::get(&build_thread_url(&board_id, &thread_id)).unwrap().text().unwrap();
        serde_json::from_str(&res).unwrap()
    }

    fn create_board(title: &str) -> String {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("title", title);
        let res = client.post(BOARD).json(&map).send();

        res.unwrap().text().unwrap()
    }

    fn create_thread(board_id: &str, title: &str, message: &str) -> String {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("title", title);
        map.insert("message", message);
        let res = client.post(&build_board_thread_url(&board_id)).json(&map).send();

        res.unwrap().text().unwrap()
    }

    fn build_board_url(board_id: &str) -> String {
        format!("{}/{}", BOARD, board_id)
    }

    fn build_board_thread_url(board_id: &str) -> String {
        format!("{}/{}/t", BOARD, board_id)
    }

    fn build_thread_url(board_id: &str, thread_id: &str) -> String {
        format!("{}/{}/t/{}", BOARD, board_id, thread_id)
    }

    fn build_thread_message_url(board_id: &str, thread_id: &str) -> String {
        format!("{}/{}/t/{}/m", BOARD, board_id, thread_id)
    }
}
