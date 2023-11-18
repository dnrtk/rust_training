use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration, sleep};
use std::error::Error;
use std::sync::Arc;

pub async fn send_msearch(socket: &UdpSocket) -> Result<(), Box<dyn Error>> {
   let message = "M-SEARCH * HTTP/1.1\r\nHost:239.255.255.250:1900\r\nST:ssdp:all\r\nMan:\"ssdp:discover\"\r\nMX:3\r\n\r\n";
   socket.send_to(message.as_bytes(), "239.255.255.250:1900").await?;
   Ok(())
}

fn extract_value(data: &str, prefix: &str) -> String {
    data.lines().filter_map(|line| {
        if line.starts_with(prefix) {
            Some(line.split_at(prefix.len()).1.trim().to_string())
        } else {
            None
        }
    }).collect()
}

pub async fn receive_and_process(socket: &UdpSocket, process_func: fn(String, String, String, &std::net::SocketAddr) -> std::io::Result<()>) -> Result<(), Box<dyn Error>> {
   let mut buf = [0; 1024];
   let timeout_duration = Duration::from_secs(10);
   loop {
       let recv_future = socket.recv_from(&mut buf);
       match timeout(timeout_duration, recv_future).await {
           Ok(Ok((amt, src))) => {
               let data = String::from_utf8_lossy(&buf[..amt]).to_string();
               let usn = extract_value(&data, "USN: ");
               let location = extract_value(&data, "LOCATION: ");
               let server = extract_value(&data, "SERVER: ");
               process_func(usn, location, server, &src)?;
           }
           Ok(Err(e)) => return Err(Box::new(e)),
           Err(_) => {
               println!("T.O.");
               break;
           }
       }
   }
   Ok(())
}

fn process_response(usn: String, location: String, server: String, addr: &std::net::SocketAddr) -> std::io::Result<()> {
   println!("Received USN: {}\nLOCATION: {}\nSERVER: {}\nfrom {:?}", usn, location, server, addr);
   Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_addr = "0.0.0.0:0";
    let socket = Arc::new(UdpSocket::bind(local_addr).await?);

    let socket_clone = Arc::clone(&socket);
    tokio::spawn(async move {
        loop {
            send_msearch(&*socket_clone).await.unwrap();
            receive_and_process(&*socket_clone, process_response).await.unwrap();
            sleep(Duration::from_secs(3600-10)).await;
        }
    });
    sleep(Duration::from_secs(30)).await;
    Ok(())
}
