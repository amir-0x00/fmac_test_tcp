use actix_rt::net::{TcpListener, TcpStream};
use colored::Colorize;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use std::error::Error;
use tokio::io::AsyncReadExt;

pub const DEFAULT_IP: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 6379;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("server!");
    let _ = start_tcp(DEFAULT_IP, 6379).await;
    Ok(())
}

static CLIENTS_LIST: Lazy<RwLock<Vec<Arc<RwLock<TcpClient>>>>> = Lazy::new(|| {
    let list = Vec::new();
    RwLock::new(list)
});

// ----------------------------------------------------------------
#[derive(Debug)]
struct TcpClient {
    pub stream: TcpStream,
    pub addr: SocketAddr,
}
impl TcpClient {
    pub async fn add(stream: TcpStream, addr: SocketAddr) {
        let client = Arc::new(RwLock::new(TcpClient { addr, stream }));
        TcpClient::start_listener(client.clone(), addr).await;
        // tokio::spawn(async move {
        CLIENTS_LIST.write().await.push(client);
        // });
        // process(&mut client.stream, addr).await;
    }

    async fn start_listener(client: Arc<RwLock<TcpClient>>, addr: SocketAddr) {
        println!("{}: {:?}", "start".green(), addr);
        let mut buffer = [0; 1024];
        // let closed = false;
        // tokio::spawn(async {
        // let _ = stream.write(message.as_bytes()).await;

        loop {
            let mut client = client.write().await;
            let len = if let Ok(len) = client.stream.read(&mut buffer).await {
                len
            } else {
                0
            };
            if len > 0 {
                // let message = String::from_utf8_lossy(&buffer[..len]);
                let len_buffer: [u8; 4] = buffer[..4].try_into().unwrap();
                println!("len_buffer : {:?} -- addr : {:?}", len_buffer, addr);
                let len = u32::from_be_bytes(len_buffer);
                println!("len: {:?}", len);
                // let message = u16::from_be_bytes(len_buffer); // println!("message : {:?}", message.as_bytes());
                // let _ = stream.write(message.as_bytes()).await;
                // let _ = stream.flush().await;
            } else {
                println!("{}: {:?}", "closed".red(), addr);
                // stream has reached EOF
                break;
            }
        }
        // });
    }
    // pub fn ssss(&self) {
    //     self.ssss();
    // }
}

// ----------------------------------------------------------------

pub async fn start_tcp(ip: &str, port: u16) {
    // Bind the listener to the address
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    loop {
        // The second item contains the IP and port of the new connection.
        let (stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            TcpClient::add(stream, addr).await;
            //     // process(socket, socket_addr).await;
        });
    }
}

// async fn process(stream: &mut TcpStream, socket_addr: SocketAddr) {
//     println!("start: {:?}", socket_addr);
//     let mut buffer = [0; 1024];
//     // let closed = false;
//     loop {
//         let len = if let Ok(len) = stream.read(&mut buffer).await {
//             len
//         } else {
//             0
//         };
//         if len > 0 {
//             // let message = String::from_utf8_lossy(&buffer[..len]);
//             let len_buffer: [u8; 4] = buffer[..4].try_into().unwrap();
//             println!("len_buffer : {:?} -- addr : {:?}", len_buffer, socket_addr);
//             let len = u32::from_be_bytes(len_buffer);
//             // println!("len: {:?}", len);
//             // let message = u16::from_be_bytes(len_buffer);
//             // println!("message : {:?}", message.as_bytes());
//             // let _ = stream.write(message.as_bytes()).await;
//             // let _ = stream.flush().await;
//         } else {
//             println!("closed: {:?}", socket_addr);
//             // stream has reached EOF
//             break;
//         }
//     }

//     // stream.read(buf)

//     // if let Some(frame) =  {
//     //     println!("GOT: {:?}", frame);

//     //     // Respond with an error
//     //     // let response = Frame::Error("unimplemented".to_string());
//     //     // connection.write_frame(&response).await.unwrap();
//     // }

//     // The `Connection` lets us read/write redis **frames** instead of
//     // byte streams. The `Connection` type is defined by mini-redis.
//     // let mut connection = Connection::new(socket);

//     // if let Some(frame) = connection.read_frame().await.unwrap() {
//     //     println!("GOT: {:?}", frame);

//     //     // Respond with an error
//     //     let response = Frame::Error("unimplemented".to_string());
//     //     connection.write_frame(&response).await.unwrap();
//     // }
// }
