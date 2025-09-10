use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread::Builder,
};

use openssl::rsa::Rsa;

struct ClientState {
    encryption_key: [u8; 20],
    counter: u16,
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let socket = Arc::new(socket);
    // Connection state like in TCP using client IP and client state
    let connection_state: Arc<Mutex<HashMap<SocketAddr, ClientState>>> =
        Arc::new(Mutex::new(HashMap::new()));
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
                // Arc clone for shared access to hashmap ptr
                let connection_state_clone = Arc::clone(&connection_state);
                // Check if client exist
                // Use builder to assign thread a name.
                let handler_name: String = format!("{:?}", addr);
                let _handler = Builder::new()
                    .name(handler_name.clone())
                    .spawn(move || {
                        // Generate RSA pair-keys
                        let (pub_key, priv_key) = generate_rsa_key_pairs();

                        let mut lock_map = connection_state_clone
                            .lock()
                            .expect("Failed to lock connection_state_clone");
                        if !lock_map.iter().any(|c| c.0 == &addr) {
                            let client_ste: ClientState = ClientState {
                                encryption_key: [0u8; 20],
                                counter: 0,
                            };
                            lock_map.insert(addr, client_ste);
                        }

                        println!("Handling {} bytes from {}", size, addr);
                        println!("{:?}", handler_name);
                        // Increase client counter
                        lock_map.entry(addr).and_modify(|c| c.counter += 1);
                        // response to client
                        let response: &[u8] = "Receive datagram :)".as_bytes();
                        let _ = socket.send_to(response, addr);
                    })
                    .expect("Failed to spawn thread");
            }
            Err(e) => {
                eprintln!("error from recv_from: {e}");
                continue;
            }
        }
    }
}

fn generate_rsa_key_pairs() -> (Vec<u8>, Vec<u8>) {
    let rsa = Rsa::generate(4096).unwrap();
    let public_rsa = rsa.public_key_to_der().expect("Failed to get public key");
    let private_rsa = rsa.private_key_to_der().expect("Failed to get private key");
    (public_rsa, private_rsa)
}
