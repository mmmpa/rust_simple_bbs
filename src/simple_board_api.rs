extern crate iron;

use crate::router::Router;
use crate::data_gateway::DataGateway;
use self::iron::method::Method;
use crate::route_action::RouteAction;
use self::iron::prelude::Iron;

pub struct SimpleBoardApi {
    gateway: DataGateway,
}

impl SimpleBoardApi {
    pub fn start(gateway: DataGateway) {
        let mut router: Router<&str> = Router::new(gateway);

        router.add_route(Method::Post, "create board", "boards", RouteAction::create_board);
        router.add_route(Method::Get, "show board", "boards/:board_id", RouteAction::show_board);

        Iron::new(router).http("localhost:3000");
    }
}

