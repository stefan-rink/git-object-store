mod objects;
mod config;

use std::net::Ipv4Addr;

use objects::object_service::ObjectService;
use objects::object_filter::object_filter;
use config::environment;

#[tokio::main]
async fn main() {
    let object_service = ObjectService::new();

    let api = object_filter(object_service);

    // Load env config
    let env = environment();

    // Run web server with host and port from .env
    let host: Ipv4Addr = env.host.parse().expect("invalid IPv4 address");
    warp::serve(api).run((host.octets(), env.port)).await;
}
