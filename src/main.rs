use tokio::net::TcpListener;

mod net;
mod store;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    let store = store::Store::new();

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("client connected: {}", peer);

        let store = store.clone();
        tokio::spawn(async move {
            if let Err(e) = net::handle_connection(socket, store).await {
                eprintln!("connection error ({}): {}", peer, e);
            }
        });
    }
}
