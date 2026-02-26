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

async fn handle_command(line: &str, store: &Store) -> Result<String, String> {
    // ping, set key, get key, del key
    let mut parts = line.split_whitespace();
    let op = parts.next().ok_or_else(|| "ERR empty command".to_string())?;
    let op_upper = op.to_ascii_uppercase();
    
    match op_upper.as_str() {
        "PING" => Ok("PONG".to_string()),
        
        "SET" => {
            let key = parts.next().ok_or_else(|| "Err empty command".to_string())?;
            
            let prefix = format!("{} {}", op, key);
            
            let value = line.strip_prefix(&prefix).or_else(|| line.strip_prefix(&prefix.to_ascii_uppercase())).unwrap_or("").trim_start();
            
            if value.is_empty() {
                return Err("ERR SET needs value".to_string())
            }
            store.set(key.as_bytes().to_vec(), value.as_bytes().to_vec()).await?;
            Ok("OK".to_string())
        }

        "GET" => {
            let key = parts.next().ok_or_else(|| "ERR GET needs key".to_string())?;
            match store.get(key.as_bytes()).await {
                Some(v) => Ok(String::from_utf8_lossy(&v).to_string()),
                None => Ok("(nil)".to_string()),
            }
        }

        "DEL" => {
            let key = parts.next().ok_or_else(|| "ERR DEL needs key".to_string())?;
            let removed = store.del(key.as_bytes()).await;
            OK(if removed {
                "1"
            } else {
                "0"
            }.to_string())
        }

        _ => Err(format!("ERR unknown command '{}'", op)),
    }

}


