use tokio::{net::TcpListener, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};
use std::io::Result;
#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("localhost:8000").await?;
    match listener.accept().await {
        Ok((mut socket, addr)) => {
            println!("Connected! {}", addr);
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut msg = String::new();
            loop {
                let bytes_read = reader.read_line(&mut msg).await?;
                println!("{}", bytes_read);
                if bytes_read == 2 {
                    break;
                }
                writer.write_all(msg.as_bytes()).await?;
                msg.clear();
            }
        },

        Err(err) => println!("Error while connecting. {}", err)
    }

    Ok(())
}
