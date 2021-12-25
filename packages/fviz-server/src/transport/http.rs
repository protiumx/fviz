use super::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};

/// Represent a client handshake request.
///
/// Clients should be known in the system as `actors`.
#[derive(Serialize, Deserialize, Debug)]
pub struct HandshakeRequest {
  client: String,
}

/// Represents a handshake response.
///
/// The `session_uuid` must be used to stablish connection with the WS server.
#[derive(Serialize, Debug)]
pub struct HandshakeResponse {
  session_uuid: String,
}

/// Handles handshake with a new client.
///
/// It assigns a new uuid v4 to the client connection.
pub async fn handshake_handler(
  body: HandshakeRequest,
  clients: ws::Clients,
) -> ws::Result<impl Reply> {
  let client = body.client;
  // TODO: validate client format
  let session_uuid = Uuid::new_v4().simple().to_string();

  register_client(session_uuid.clone(), client, clients).await;
  Ok(json(&HandshakeResponse { session_uuid }))
}

async fn register_client(uuid: String, name: String, clients: ws::Clients) {
  // We need to await the tokio asynchronous mutex
  // TODO: check if the client is already registered
  clients
    .write()
    .await
    .insert(uuid, ws::Client { name, sender: None });
}

/// Upgrades a client connection to WS if the client exists
pub async fn ws_handler(
  ws: warp::ws::Ws,
  session_uuid: String,
  clients: ws::Clients,
) -> ws::Result<impl Reply> {
  println!("session ws {}", session_uuid);
  let client = clients.read().await.get(&session_uuid).cloned();
  match client {
    Some(client) => Ok(
      // Pass connection to the ws handler from warp filter
      ws.on_upgrade(move |socket| ws::client_connection(socket, session_uuid, clients, client)),
    ),
    None => Err(warp::reject::not_found()),
  }
}

/// Could be used for kubernetes liveness and readiness probes
pub async fn health_handler() -> ws::Result<impl Reply> {
  Ok(StatusCode::OK)
}

pub mod filters {
  use super::*;
  use std::convert::Infallible;
  use warp::Filter;

  pub fn handshake(
    clients: ws::Clients,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("handshake")
      .and(warp::post())
      .and(warp::body::json())
      .and(with_clients(clients))
      .and_then(handshake_handler)
  }

  pub fn websockets(
    clients: ws::Clients,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
      // use warp ws filter to upgrade connection
      .and(warp::ws())
      .and(warp::path::param())
      .and(with_clients(clients))
      .and_then(ws_handler)
  }

  pub fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health").and_then(health_handler)
  }

  fn with_clients(
    clients: ws::Clients,
  ) -> impl Filter<Extract = (ws::Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
  }
}

#[cfg(test)]
mod should {
  use super::{filters, ws, HandshakeRequest};
  use std::collections::HashMap;
  use std::sync::Arc;
  use tokio::sync::RwLock;
  use warp::test::request;

  #[tokio::test]
  async fn return_health_status() {
    let api = filters::health();
    let resp = request().method("GET").path("/health").reply(&api).await;
    assert_eq!(resp.status(), 200);
  }

  #[tokio::test]
  async fn register_new_client() {
    let clients: ws::Clients = Arc::new(RwLock::new(HashMap::new()));
    let api = filters::handshake(clients);
    let resp = request()
      .method("POST")
      .path("/handshake")
      .json(&HandshakeRequest {
        client: "FVIZ-TEST".to_string(),
      })
      .reply(&api)
      .await;
    let result: Vec<u8> = resp.into_body().into_iter().collect();
    // result
  }
}
