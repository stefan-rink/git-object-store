mod config;
mod git_storage;
mod objects;

use std::net::Ipv4Addr;

use anyhow::Context;
use config::environment;
use git_storage::GitStorage;
use objects::object_filter::object_filter;
use objects::object_service::ObjectService;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let object_service = ObjectService::new();

    let api = object_filter(object_service);

    // Load env config
    let env = environment()?;

    let git_storage_handler = GitStorage::new(&env.git_storage_path);

    let repo = git_storage_handler.get_repo_storage();

    // Run web server with host and port from .env
    let host: Ipv4Addr = env.host.parse().context("invalid IPv4 address")?;
    warp::serve(api).run((host.octets(), env.port)).await;

    Ok(())
}
