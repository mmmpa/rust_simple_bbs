extern crate iron;

use crate::router::Router;
use crate::data_gateway::DataGateway;
use self::iron::method::Method;
use crate::route_action::RouteAction;
use self::iron::prelude::Iron;

pub struct Server {
    gateway: DataGateway,
}

impl Server {
    pub fn start(gateway: DataGateway) {
        let mut router: Router<&str> = Router::new(gateway);

        router.add_route(Method::Get, "show board", "b/:board_id", RouteAction::show_board);
        router.add_route(Method::Get, "show thread", "b/:board_id/t/:thread_id", RouteAction::show_thread);
        router.add_route(Method::Get, "show thread with range", "b/:board_id/t/:thread_id/:range", RouteAction::show_thread);

        router.add_route(Method::Post, "create board", "b", RouteAction::create_board);
        router.add_route(Method::Post, "create thread", "b/:board_id/t", RouteAction::create_thread);
        router.add_route(Method::Post, "create message", "b/:board_id/t/:thread_id/m", RouteAction::create_message);

        Iron::new(router).http("localhost:3000");
    }
}

