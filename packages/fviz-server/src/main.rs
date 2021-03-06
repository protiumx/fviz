use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use transport::{http, ws};
use warp::Filter;

mod transport;

extern crate dotenv;

/// For testing purposes. Devices should be loaded from a DB and verified.
static DEVICES: &[&str] = &["63bbbb82-c6ce-430f-ac19-f6479cee4ab7"];

#[tokio::main]
async fn main() {
  let path = std::path::Path::new("../.env");
  dotenv::from_path(path).ok();
  let clients: ws::Clients = Arc::new(RwLock::new(HashMap::new()));
  load_devices(&clients).await;
  let addr = std::env::var("HOST_PORT")
    .ok()
    .and_then(|string| SocketAddr::from_str(&string).ok())
    // Default to port 9000
    .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:9000").unwrap());
  run_server(addr, &clients).await;
}

async fn load_devices(clients: &ws::Clients) {
  for item in DEVICES.iter().enumerate() {
    let (i, duid): (usize, &&str) = item;
    clients.write().await.insert(
      duid.to_string(),
      ws::Client {
        name: format!("DEVICE-{:03}", i),
        sender: None,
      },
    );
  }
  println!("Devices loaded");
}

async fn run_server(addr: SocketAddr, clients: &ws::Clients) {
  let routes = http::filters::health()
    .or(http::filters::handshake(clients.clone()))
    .or(http::filters::websockets(clients.clone()))
    // enable CORS
    .with(
      warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"])
        .allow_headers(vec![
          "Accept",
          "User-Agent",
          "Content-Type",
          "Sec-Fetch-Mode",
          "Referer",
          "Origin",
          "Access-Control-Allow-Origin",
          "Access-Control-Request-Method",
          "Access-Control-Request-Headers",
        ]),
    );
  println!("Server running at: {}", addr);
  warp::serve(routes).run(addr).await;
}
