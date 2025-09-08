use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::Arc,
    thread::{self, Builder},
};

struct ClientState {
    encryption_key: [u8; 20],
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let socket = Arc::new(socket);
    // Connection state like in TCP using client ip and client state
    let mut connection_state: HashMap<SocketAddr, ClientState> = HashMap::new();
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let data = buf[..size].to_vec();
                //TODO
                //handle timeout for client
                // Dup socket for concurrency
                let socket = Arc::clone(&socket);
                //TODO
                // iterate over hashmap to find existing client, if no client, create thread and
                // add client to hashmap
                if !connection_state.iter().any(|c| c.0 == &addr) {
                    let client_ste: ClientState = ClientState {
                        encryption_key: [0u8; 20],
                    };
                    connection_state.insert(addr, client_ste);
                }
                // Use builder to assign thread a name.
                let handler_name: String = format!("{:?}", addr);
                let _ = Builder::new()
                    .name(handler_name.clone())
                    .spawn(move || {
                        println!("Handling {} bytes from {}", size, addr);
                        println!("{:?}", handler_name);
                        // response to client
                        let response: &[u8] = "Receive datagram :)".as_bytes();
                        let _ = socket.send_to(response, addr);
                    })
                    .expect("Failed to spawn thread")
                    .join();
            }
            Err(e) => {
                eprintln!("error from recv_from: {e}");
                continue;
            }
        }
    }
}
