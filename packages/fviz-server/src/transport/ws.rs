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
  pub name: String,
  /// Tracks each client sender
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

/// Spawn a new tokio task for the client ws connection. Acts as Stream and Sink (sender and a receiver)
pub async fn client_connection(
  ws: WebSocket,
  session_uuid: String,
  clients: Clients,
  mut client: Client,
) {
  // split into ender and receiver
  let (tx, mut rx) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();

  tokio::task::spawn(client_rcv.forward(tx).map(|result| {
    if let Err(e) = result {
      eprintln!("error sending websocket msg: {}", e);
    }
  }));

  // update clients sender channel
  client.sender = Some(client_sender);
  clients.write().await.insert(session_uuid.clone(), client);

  println!("{} connected", session_uuid);

  // Await for client streams
  while let Some(result) = rx.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!(
          "error receiving ws message for session_uuid: {}): {}",
          session_uuid.clone(),
          e
        );
        // TODO: disconnect WS
        break;
      }
    };
    process_client_message(&session_uuid, msg, &clients).await;
  }

  clients.write().await.remove(&session_uuid);

  println!("{} disconnected", session_uuid);
}

async fn process_client_message(session_uuid: &str, msg: Message, clients: &Clients) {
  println!("received message from {}: {:?}", session_uuid, msg);

  let client = match clients.read().await.get(session_uuid).cloned() {
    Some(c) => c,
    None => return,
  };

  // if msg.is_close() {
  //   println!("Client {} has closed", session_uuid);
  //   clients.write().await.remove(session_uuid);
  // }

  let message = match msg.to_str() {
    Ok(v) => v,
    Err(_) => return,
  };
  if message.contains("ping") {
    if let Some(sender) = client.sender {
      // Ignore the error. Disconnection should be handled in another task.
      if let Err(_disconnected) = sender.send(Ok(Message::text("pong"))) {};
    }
    return;
  }

  // Broadcast to UI clients
  println!("Broadcasting to all UI clients");
  clients
    .read()
    .await
    .iter()
    .filter(|(_, client)| client.name.contains("FVIZ-"))
    .for_each(|(_, client)| {
      if let Some(sender) = &client.sender {
        let _ = sender.send(Ok(Message::text(message)));
      }
    });
}

#[cfg(test)]
mod tests {
  #[test]
  #[ignore]
  fn test_client_connection() {}
}
