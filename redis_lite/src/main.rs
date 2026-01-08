use bytes::BytesMut;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::logger::Logger;
use crate::parser::RespType;

mod command;
mod db;
mod logger;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}

async fn process_socket(mut socket: TcpStream) {
    tokio::spawn(async move {
        let mut db: db::Db = db::Db::new();
        let date_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let logger = logger::Logger {
            file_name: String::from(format!("logs/logger-{}.txt", date_time)),
        };

        let mut buf = BytesMut::with_capacity(1024);

        loop {
            let _ = match socket.read_buf(&mut buf).await {
                Ok(0) => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let command = parser::RespType::parse(&buf);
            let _ = process_query(&command, &mut socket, &mut db).await;

            let args: String = command.to_readable_string();
            let message = Logger::format_message(&args);
            match logger.write(&message) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                }
            };
            buf.clear();
        }
    });
}

async fn process_query(
    resp: &RespType,
    socket: &mut TcpStream,
    db: &mut db::Db,
) -> std::io::Result<()> {
    let command_str = resp.to_readable_string();
    let command_vec: Vec<String> = command_str.split(' ').map(|s| s.to_string()).collect();

    let command = command::Command::get_command(&command_vec[0]);

    match command {
        command::Command::Get => {
            match db.read(&command_vec) {
                Ok(value) => {
                    socket
                        .write_all(&parser::RespType::str_to_resp_byte(value))
                        .await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        command::Command::Set => {
            match db.write(&command_vec) {
                Ok(_) => {
                    socket
                        .write_all(&parser::RespType::str_to_resp_byte("OK"))
                        .await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        command::Command::Ping => {
            if command_vec.len() == 1 {
                socket
                    .write_all(&parser::RespType::str_to_resp_byte("PONG"))
                    .await?;
            } else {
                socket
                    .write_all(&parser::RespType::str_to_resp_byte(&format!(
                        "PONG {}",
                        &command_vec[1..].join(" ")
                    )))
                    .await?;
            }
            Ok(())
        }
        _ => {
            socket
                .write_all(&parser::RespType::str_to_resp_byte("Invalid Command"))
                .await?;
            Ok(())
        }
    }
}
