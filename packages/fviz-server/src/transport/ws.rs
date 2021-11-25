use futures::{FutureExt, StreamExt};
use std::collections::HashMap;
// Atomic smart pointer to pass mutex safely between threads
use std::sync::Arc;
// multiple producer, single consumer
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

/// Client representation
#[derive(Debug, Clone)]
pub struct Client {
  pub client: String,
  /// Used to forward messages
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

/// Spawn a new tokio task for the client ws connection. Acts as Stream and Sink (sender and a receiver)
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
  // split into ender and receiver
  let (client_ws_sender, mut client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      eprintln!("error sending websocket msg: {}", e);
    }
  }));

  // update clients sender channel
  client.sender = Some(client_sender);
  clients.write().await.insert(id.clone(), client);

  println!("{} connected", id);

  // Await for client streams
  while let Some(result) = client_ws_rcv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
        // TODO: disconnect WS
        break;
      }
    };
    process_client_message(&id, msg, &clients).await;
  }

  clients.write().await.remove(&id);
  println!("{} disconnected", id);
}

async fn process_client_message(id: &str, msg: Message, _: &Clients) {
  println!("received message from {}: {:?}", id, msg);
  let message = match msg.to_str() {
    Ok(v) => v,
    Err(_) => return,
  };

  println!("{}", message);

  // TODO: use regex
  if message == "ping" || message == "ping\n" {
    // TODO: reply with "pong"
    return;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  #[ignore]
  fn test_client_connection() {}
}
