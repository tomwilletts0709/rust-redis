use crate::resp::{bytes_to_resp, RESP};
use crate::resp_results::RESPResult;
use crate::storage::Storage;
use std::syn::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp;
mod resp_results;
mod server; 
mod storage; 
mod storage_result;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    let mut storage = Storage::new()

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream, storage));
            }
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        }
    }

async fn handle_connection(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) {
    let mut buffer = [0; 512];


    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                let response = RESP::SimpleString(String::from("PONG"));

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

        }
    }
}


fn parser_router(
    buffer: &[u8],
    index: &mut usize,
) -> Option<fn(&[u8], &mut usize) -> RESPResult<RESP>> {
    match buffer[*index] {
        b'+' => Some(parse_simple_string),
        _ => None,
    }
}

pub fn bytes_to_resp(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    match parse_router(buffer, index) {
        Some(parse_func) => {
            let result: RESP = parse_func(buffer, index)?;
            Ok(result)
        }
        None => {
            Err(RESPError::Unknown)
        }
    }
}