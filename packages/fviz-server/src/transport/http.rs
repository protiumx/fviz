use super::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};

/// Represent a client handshake request.
///
/// Clients should be known in the system as `actors`.
#[derive(Deserialize, Debug)]
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
pub async fn handshake_handler(body: HandshakeRequest, clients: Clients) -> Result<impl Reply> {
  let client = body.client;
  // TODO: validate client format
  let session_uuid = Uuid::new_v4().simple().to_string();

  register_client(session_uuid.clone(), client, clients).await;
  Ok(json(&HandshakeResponse { session_uuid }))
}

async fn register_client(uuid: String, client: String, clients: Clients) {
  // We need to await the tokio asynchronous mutex
  // TODO: check if the client is already registered
  clients.write().await.insert(
    uuid,
    Client {
      client,
      sender: None,
    },
  );
}

/// Upgrades a client connection to WS if the client exists
pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
  let client = clients.read().await.get(&id).cloned();
  match client {
    Some(c) => Ok(
      // Pass connection to the ws handler from warp filter
      ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c)),
    ),
    None => Err(warp::reject::not_found()),
  }
}

/// Could be used for kubernetes liveness and readiness probes
pub async fn health_handler() -> Result<impl Reply> {
  Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  #[ignore]
  fn test_register_client() {}

  #[test]
  #[ignore]
  fn test_upgrade_connection() {}

  #[test]
  #[ignore]
  fn test_health() {}
}
