use tokio::{net::TcpListener, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:8000").await?;
    // Listening on multiple clients 
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("Connected! {}", addr);
                // To handle multiple clients, we need multi-thread to handle each task 
                tokio::spawn(async move {
                    let (reader, mut writer) = socket.split();
                    let mut reader = BufReader::new(reader);
                    let mut msg = String::new();
                    loop {
                        match reader.read_line(&mut msg).await {
                            Ok(bytes_size) => {
                                println!("{}", bytes_size);
                                if bytes_size == 2 {
                                    break;
                                }
                                match writer.write_all(msg.as_bytes()).await {
                                    Ok(()) => (),
                                    Err(err) => println!("Error writing {}", err)
                                }
                                msg.clear();
                            },
                            Err(err) => println!("error reading bytes {}", err)
                        }
                    }
                });
            },
    
            Err(err) => println!("Error while connecting. {}", err)
        }
    }
}
