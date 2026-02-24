use tokio::{
    io::{AsyncBufRead, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::store::Store;

pub async fn handle_connection(socket: TcpStream, store: Store) -> anyhow::Result<()> {
    let (read_half, mut write_half) = socket.into_split();

    let mut reader = BufReader::new(read_half);
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            return Ok(());
        }
        
        let cmdline = line.trim_end_matches(&['\r', '\n'][..]);
        
        if cmdline.is_empty() {
            continue;
        }
        
        match handle_command(cmdline, &store).await {
            Ok(resp) => {
                write_half.write_all(resp.as_bytes()).await?;
                write_half.write_all(b"\n").await?;
            }
            Err(err_msg) => {
                write_half.write_all(err_msg.as_bytes()).await?;
                write_half.write_all(b"\n").await?;
            }
        }
    }
}

async fn handle_command(Line: &str, store: &Store) -> Result<String, String> {
    // stub
    Ok("hello".to_string())
}
