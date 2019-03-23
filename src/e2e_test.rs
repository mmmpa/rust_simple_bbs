#[cfg(test)]
mod tests {
    use std::thread::spawn;
    use crate::server::Server;
    use crate::data_gateway::DataGateway;
    use crate::json_adapter::JsonAdapter;
    use std::collections::HashMap;
    use crate::board::Board;
    use crate::board_thread::BoardThread;

    #[test]
    fn test_e2e_normal_creation() {
        let port = 3830;

        spawn(move || {
            let adapter = JsonAdapter::new("test_e2e_normal_creation", true);
            let gateway = DataGateway::new(Box::new(adapter));

            Server::start(port, gateway);
        });

        while reqwest::get(&host_url(port)).unwrap().text().unwrap() != "root" {}

        let res = reqwest::get(&host_url(port)).unwrap().text().unwrap();
        assert_eq!(res, "root");

        let board_id = create_board(port, "new board");
        let board: Board = get_board(port, &board_id);
        assert_eq!(board.title, "new board");

        let thread_id = create_thread(port, &board_id, "new thread", "hello new thread");
        let thread = get_thread(port, &board_id, &thread_id);
        assert_eq!(thread.title, "new thread");

        create_message(port, &board_id, &thread_id, "hello");

        let thread = get_thread(port, &board_id, &thread_id);
        assert_eq!(thread.messages.len(), 2);
    }

    #[test]
    fn test_e2e_huge_threads_posting() {
        let port = 3831;

        spawn(move || {
            let adapter = JsonAdapter::new("test_e2e_huge_threads_posting", true);
            let gateway = DataGateway::new(Box::new(adapter));

            Server::start(port, gateway);
        });

        while reqwest::get(&host_url(port)).unwrap().text().unwrap() != "root" {}

        let res = reqwest::get(&host_url(port)).unwrap().text().unwrap();
        assert_eq!(res, "root");

        let board_id = create_board(port, "new board");

        let workers_num = 10;
        let mut workers = Vec::with_capacity(workers_num);
        for i in 0..workers_num {
            let b_id = board_id.clone();
            workers.push(spawn(move || {
                for n in 0..100 {
                    create_thread(port, &b_id, &format!("{} {} {}", &b_id, i, n), &format!("hello {} {} {}", &b_id, i, n));
                }
            }));
        }

        for worker in workers {
            worker.join().unwrap();
        }

        let board: Board = get_board(port, &board_id);
        assert_eq!(board.summaries.len(), 1000);
    }

    #[test]
    fn test_e2e_huge_messages_posting() {
        let port = 3832;

        spawn(move || {
            let adapter = JsonAdapter::new("test_e2e_huge_messages_posting", true);
            let gateway = DataGateway::new(Box::new(adapter));

            Server::start(port, gateway);
        });

        while reqwest::get(&host_url(port)).unwrap().text().unwrap() != "root" {}

        let res = reqwest::get(&host_url(port)).unwrap().text().unwrap();
        assert_eq!(res, "root");

        let board_id = create_board(port, "new board");
        let thread_id = create_thread(port, &board_id, "new thread", "hello new thread");

        let workers_num = 10;
        let mut workers = Vec::with_capacity(workers_num);
        for i in 0..workers_num {
            let b_id = board_id.clone();
            let t_id = thread_id.clone();
            workers.push(spawn(move || {
                for n in 0..100 {
                    create_message(port, &b_id, &t_id, &format!("{} - {}", i, n));
                }
            }));
        }

        for worker in workers {
            worker.join().unwrap();
        }

        let thread = get_thread(port, &board_id, &thread_id);
        assert_eq!(thread.messages.len(), 1001);
    }

    fn get_board(port: u16, board_id: &str) -> Board {
        let res = reqwest::get(&build_board_url(port, &board_id)).unwrap().text().unwrap();
        serde_json::from_str(&res).unwrap()
    }

    fn get_thread(port: u16, board_id: &str, thread_id: &str) -> BoardThread {
        let res = reqwest::get(&build_thread_url(port, &board_id, &thread_id)).unwrap().text().unwrap();
        serde_json::from_str(&res).unwrap()
    }

    fn create_board(port: u16, title: &str) -> String {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("title", title);
        let res = client.post(&board_url(port)).json(&map).send();

        res.unwrap().text().unwrap()
    }

    fn create_thread(port: u16, board_id: &str, title: &str, message: &str) -> String {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("title", title);
        map.insert("message", message);
        let res = client.post(&build_board_thread_url(port, &board_id)).json(&map).send();

        res.unwrap().text().unwrap()
    }

    fn create_message(port: u16, board_id: &str, thread_id: &str, message: &str) -> () {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("message", message);
        let res = client.post(&build_thread_message_url(port, board_id, thread_id)).json(&map).send();

        res.unwrap().text().unwrap();
    }

    const HOST: &str = "http://localhost";

    fn host_url(port: u16) -> String {
        format!("{}:{}", HOST, port)
    }

    fn board_url(port: u16) -> String {
        format!("{}/api/b", host_url(port))
    }

    fn build_board_url(port: u16, board_id: &str) -> String {
        format!("{}/api/b/{}", host_url(port), board_id)
    }

    fn build_board_thread_url(port: u16, board_id: &str) -> String {
        format!("{}/api/b/{}/t", host_url(port), board_id)
    }

    fn build_thread_url(port: u16, board_id: &str, thread_id: &str) -> String {
        format!("{}/api/b/{}/t/{}", host_url(port), board_id, thread_id)
    }

    fn build_thread_message_url(port: u16, board_id: &str, thread_id: &str) -> String {
        format!("{}/api/b/{}/t/{}/m", host_url(port), board_id, thread_id)
    }
}
