use napi::Env;
use napi_derive::*;
use tokio::{runtime::Handle, net::{TcpListener, TcpStream}, task::JoinHandle};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};

async fn accept_connection(peer: std::net::SocketAddr, stream: TcpStream) {
  if let Err(e) = handle_connection(peer, stream).await {
    println!("Error on WS handling!")
  }
}

async fn handle_connection(peer: std::net::SocketAddr, stream: TcpStream) -> tungstenite::Result<()> {
  let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

  println!("New WS");

  while let Some(msg) = ws_stream.next().await {
      let msg = msg?;
      if msg.is_text() || msg.is_binary() {
          ws_stream.send(msg).await?;
      }
  }

  Ok(())
}

#[napi]
pub struct LycheeServer {
  handle: Option<Handle>,
  join_handle: Option<JoinHandle<()>>,
}

#[napi]
impl LycheeServer {
  #[napi(constructor)]
  pub fn new(env: Env) -> Self {
    LycheeServer { handle: None, join_handle: None }
  }

  #[napi]
  pub async fn start_server(&mut self) {
    let handle = Handle::current();
    self.handle = Some(handle);
    let join = self.handle.as_ref().unwrap().spawn(async {
      let addr = "127.0.0.1:9002";
      let listener = TcpListener::bind(&addr).await.expect("Can't listen");
      println!("Listening on: {}", addr);
  
      while let Ok((stream, _)) = listener.accept().await {
          let peer = stream.peer_addr().expect("connected streams should have a peer address");
          println!("Peer address: {}", peer);
  
          tokio::spawn(accept_connection(peer, stream));
      }
    });

    self.join_handle = Some(join);
  }

  #[napi]
  pub fn do_something_else(&self) {
    println!("Yep it still works");
  }
}