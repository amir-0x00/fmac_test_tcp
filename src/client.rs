use std::time::Duration;

use colored::Colorize;
use once_cell::sync::Lazy;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub const DEFAULT_IP: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 6379;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("client!");
    // sleep(Duration::from_secs(1)).await;

    // let _ = start_tcp_client(DEFAULT_IP, DEFAULT_PORT).await?;
    TcpServer::start(DEFAULT_IP, DEFAULT_PORT).await?;

    Ok(())
}

static TCP_SERVER_STREAM: Lazy<Mutex<TcpServer>> = Lazy::new(|| Mutex::new(TcpServer::empty()));

/// connection to the server
#[derive(Debug)]
pub struct TcpServer {
    stream: Option<TcpStream>,
}

impl TcpServer {
    fn empty() -> Self {
        Self { stream: None }
    }
    // fn new(stream: TcpStream) -> Self {
    //     Self { stream }
    // }
    pub async fn start(ip: &str, port: u16) -> std::io::Result<()> {
        loop {
            let stream = Self::connect(ip, port).await;
            TCP_SERVER_STREAM.lock().await.stream = Some(stream);
            Self::start_listener().await?;
        }
        // Ok(())
    }
    async fn start_listener() -> std::io::Result<()> {
        println!("start_listener");
        tokio::spawn(async move {
            println!("tokio::spawn");
            let mut buffer = [0; 1024];
            loop {
                let mut server = TCP_SERVER_STREAM.lock().await;
                if let Some(stream) = server.stream.as_mut() {
                    let len = if let Ok(len) = stream.read(&mut buffer).await {
                        len
                    } else {
                        0
                    };
                    if len > 0 {
                        // let message = String::from_utf8_lossy(&buffer[..len]);
                        let len_buffer: [u8; 4] = buffer[..4].try_into().unwrap();
                        println!("len_buffer : {:?}", len_buffer);
                        let len = u32::from_be_bytes(len_buffer);
                        println!("len: {:?}", len);
                        // let message = u16::from_be_bytes(len_buffer); // println!("message : {:?}", message.as_bytes());
                        // let _ = stream.write(message.as_bytes()).await;
                        // let _ = stream.flush().await;
                    } else {
                        println!("{}", "closed".red());
                        // stream has reached EOF
                        break;
                    }
                } else {
                    break;
                }
                // println!("server : {:?}", stream.peer_addr());

                // tokio::select! {
                //   _ = timeout.as_mut() =>{
                //     // let message = math_rand_alpha(15);
                //     // println!("Sending '{}'", message);
                //     // result = d2.send_text(message).await.map_err(Into::into);
                //     let _ = stream.write_all(b"2222").await;
                //     println!("sent : {:?}", stream.local_addr());
                //   }
            }
        })
        .await?;
        Ok(())
    }
    async fn connect(ip: &str, port: u16) -> TcpStream {
        let mut try_count = 0;
        loop {
            let millis = (200 * try_count).min(2500);
            println!("retry : {:?}", millis);
            let duration = Duration::from_millis(millis);
            tokio::time::sleep(duration).await;
            let stream = TcpStream::connect(format!("{}:{}", ip, port)).await;
            if let Ok(stream) = stream {
                return stream;
            } else {
                try_count += 1;
            }
        }
    }
}

pub async fn start_tcp_client(ip: &str, port: u16) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
    // .expect("can't connect to server");
    println!("server : {:?}", stream.local_addr());
    let timeout = tokio::time::sleep(Duration::from_secs(2));
    tokio::pin!(timeout);
    loop {
        // println!("server : {:?}", stream.peer_addr());

        tokio::select! {
          _ = timeout.as_mut() =>{
            // let message = math_rand_alpha(15);
            // println!("Sending '{}'", message);
            // result = d2.send_text(message).await.map_err(Into::into);
            let _ = stream.write_all(b"2222").await;
            println!("sent : {:?}", stream.local_addr());
            }
        };
    }

    // Ok(())
}
