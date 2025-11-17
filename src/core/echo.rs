use std::net::SocketAddr;

use crate::core::{
    crypto::KeyPair,
    protocol::HandshakeInitiation,
    tun::Tun,
    udp::{self, UdpTransport},
};

// this sets up the Tun -> Udp and Udp -> Tun
pub fn run_echo(server_addr: SocketAddr) -> std::io::Result<()> {
    let tun = Tun::new()?;
    println!("TUN {} created (fd={})", tun.name, tun.fd);
    let mut udp = UdpTransport::new("0.0.0.0:0".parse().unwrap(), server_addr)?;
    println!("UDP bound and connected to {}", server_addr);

    let keypair = KeyPair::generate();
    let sender_index = 12345; // In real: random u32

    let handshake = HandshakeInitiation::new(sender_index, keypair.public_key);
    let handshake_bytes = handshake.to_bytes();
    udp.send(&handshake_bytes)?;
    println!("Sent handshake ({} bytes)", handshake_bytes.len());
    let mut buf = vec![0; 1024];

    println!("echo tunnel started try to ping at 10.0.0.1");
    loop {
        // read from tun
        match tun.read(&mut buf) {
            Ok(packet) => {
                println!("packet recieved from tun : {} bytes", packet.len());
                udp.send(packet)?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        match udp.recv(&mut buf) {
            Ok(Some(n)) => {
                println!("udp to tun {} bytes", n);
                tun.write(&buf[..n])?;
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }
    }
}
