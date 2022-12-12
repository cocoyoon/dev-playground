use tokio::{net::TcpListener, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::broadcast};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:8000").await?;
    // Channel buffer size for clients
    let (tx, _rx) = broadcast::channel(10);
    // Listening on multiple clients 
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("Connected! Client address: {}", addr);
                // Cloning (tx, rx) for each client 
                let tx = tx.clone();
                let mut rx = tx.subscribe();
                // To handle multiple clients, we need multi-thread to handle each task 
                tokio::spawn(async move {
                    // Create reader/writer that can read/write between server and each client that connects
                    let (reader, mut writer) = socket.split();
                    let mut reader = BufReader::new(reader);
                    let mut msg = String::new(); // Create buffer that can store message from client 
                    // To handle multiple messages from each client
                    loop {
                        tokio::select! {
                            result = reader.read_line(&mut msg) => {
                                match result {
                                    Ok(size) => {
                                        if size == 2 {
                                            break;
                                        }
                                        tx.send((msg.clone(), addr)).unwrap();
                                    }
                                    Err(err) => println!("Error reading bytes {}", err)
                                }
                            }

                            result = rx.recv() => {
                                let (message, recv_addr) = result.unwrap();
                                if addr != recv_addr {
                                    match writer.write_all(message.as_bytes()).await {
                                        Ok(()) => (),
                                        Err(err) => println!("Error writing {}", err),
                                    }
                                }
                            }
                        }
                    }
                });
            },
    
            Err(err) => println!("Error while connecting. {}", err)
        }
    }
}
