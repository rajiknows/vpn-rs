use mio::{Events, Interest, Poll, Token, net::UdpSocket};
use std::collections::HashMap;
use std::net::SocketAddr;

use vpn::core::crypto::KeyPair;
use vpn::core::protocol::{HandshakeInitiation, HandshakeResponse};

const TOKEN_UDP: Token = Token(0);

#[derive(Debug)]
struct Peer {
    addr: SocketAddr,
    client_index: u32,
    client_ephem: [u8; 32],
    // keys will be added next step
}

fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("0.0.0.0:51820".parse().unwrap())?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut socket, TOKEN_UDP, Interest::READABLE)?;

    let server_keys = KeyPair::generate();
    let mut peers: HashMap<u32, Peer> = HashMap::new();

    println!("Server listening on 51820");
    println!(
        "Server public key (base64): {}",
        base64::encode(&server_keys.public_key)
    );

    let mut buf = [0u8; 2048];

    loop {
        poll.poll(&mut events, None)?;

        for _ in events.iter() {
            while let Ok((len, src)) = socket.recv_from(&mut buf) {
                let pkt = &buf[..len];

                // Handshake Initiation (type 1)
                if pkt[0] == 1 {
                    if let Some(init) = HandshakeInitiation::from_bytes(pkt) {
                        println!("New client {} from {}", init.client_id, src);

                        // Store peer
                        peers.insert(
                            init.client_id,
                            Peer {
                                addr: src,
                                client_index: init.client_id,
                                client_ephem: init.ephemeral_public_key,
                            },
                        );

                        // Send Handshake Response (type 2)
                        let resp = HandshakeResponse::new(init.client_id, &server_keys.public_key);
                        socket.send_to(&resp.to_bytes(), src)?;
                        println!("Sent type 2 response");
                    }
                }

                // Future: handle type 4 encrypted packets here
            }
        }
    }
}
