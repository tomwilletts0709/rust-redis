use rust_redis::{parse_resp_array, Storage};
use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    let storage = Arc::new(Mutex::new(Storage::new()));

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream, storage.clone()));
            }
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                let mut index = 0;
                let parsed = match parse_resp_array(&buffer[..size], &mut index) {
                    Ok(p) => p,
                    Err(e) => {
                        let err_resp = format!("-ERR parse error: {}\r\n", e);
                        let _ = stream.write_all(err_resp.as_bytes()).await;
                        let _ = stream.flush().await;
                        continue;
                    }
                };

                let response = match {
                    let mut guard = storage.lock().unwrap();
                    guard.process_command(&parsed)
                } {
                    Ok(r) => r,
                    Err(e) => {
                        let err_resp = format!("-ERR {}\r\n", e);
                        let _ = stream.write_all(err_resp.as_bytes()).await;
                        let _ = stream.flush().await;
                        continue;
                    }
                };

                if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                    eprintln!("error writing response: {}", e);
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("error flushing stream: {}", e);
                }
            }
            Ok(_) => {
                eprintln!("connection closed");
                break;
            }
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        }
    }
}