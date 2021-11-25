use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use transport::{http, ws};
use warp::Filter;

mod transport;

extern crate dotenv;

#[tokio::main]
async fn main() {
  let path = std::path::Path::new("../.env");
  dotenv::from_path(path).ok();
  let clients: ws::Clients = Arc::new(RwLock::new(HashMap::new()));

  let addr = std::env::var("HOST_PORT")
    .ok()
    .and_then(|string| SocketAddr::from_str(&string).ok())
    // Default to port 9000
    .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:9000").unwrap());
  run_server(addr, &clients).await;
}

async fn run_server(addr: SocketAddr, clients: &ws::Clients) {
  let health_route = warp::path!("health").and_then(http::health_handler);

  let handshake = warp::path("handshake");
  let handshake_routes = handshake
    .and(warp::post())
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(http::handshake_handler);

  let ws_route = warp::path("ws")
    // use warp ws filter to upgrade connection
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(http::ws_handler);

  let routes = health_route
    .or(handshake_routes)
    .or(ws_route)
    // enable CORS
    .with(warp::cors().allow_any_origin());
  println!("Running server at: {}", addr);
  warp::serve(routes).run(addr).await;
}

/// Clone clients
fn with_clients(
  clients: ws::Clients,
) -> impl Filter<Extract = (ws::Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}
