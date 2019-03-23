use iron;

use crate::router::Router;
use crate::data_gateway::DataGateway;
use self::iron::method::Method;
use crate::route_action::RouteAction;
use self::iron::prelude::Iron;

pub struct Server {}

impl Server {
    pub fn start(port: u16, gateway: DataGateway) {
        let mut router: Router<&str> = Router::new(gateway);

        router.add_route(Method::Get, "show html root", "/", RouteAction::show_root);
        router.add_route(Method::Get, "show html boards", "b", RouteAction::show_root);
        router.add_route(Method::Get, "show html board", "b/:*/", RouteAction::show_root);
        router.add_route(Method::Get, "show html thread", "b/:*/t/:*", RouteAction::show_root);
        router.add_route(Method::Get, "show html thread with range", "b/:*/t/:*/:*", RouteAction::show_root);

        router.add_route(Method::Get, "show board", "api/b/:board_id", RouteAction::show_board);
        router.add_route(Method::Get, "show thread", "api/b/:board_id/t/:thread_id", RouteAction::show_thread);
        router.add_route(Method::Get, "show thread with range", "api/b/:board_id/t/:thread_id/:range", RouteAction::show_thread);

        router.add_route(Method::Post, "create board", "api/b", RouteAction::create_board);
        router.add_route(Method::Post, "create thread", "api/b/:board_id/t", RouteAction::create_thread);
        router.add_route(Method::Post, "create message", "api/b/:board_id/t/:thread_id/m", RouteAction::create_message);

        Iron::new(router).http(format!("localhost:{}", port)).unwrap();
        println!("On 3000");
    }
}
