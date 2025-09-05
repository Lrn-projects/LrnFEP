use std::{
    net::UdpSocket,
    sync::Arc,
    thread::{self},
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let socket = Arc::new(socket);
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let data = buf[..size].to_vec();
                let socket = Arc::clone(&socket);
                thread::spawn(move || {
                    println!("Handling {} bytes from {}", size, addr);
                    // response to client
                    let response: &[u8] = "Receive datagram :)".as_bytes();
                    let _ = socket.send_to(response, addr);
                });
            }
            Err(e) => {
                eprintln!("error from recv_from: {e}");
                continue;
            }
        }
    }
}
