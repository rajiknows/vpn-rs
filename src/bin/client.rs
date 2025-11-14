use vpn::core::echo;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let tun = Tun::new("tun0".to_string())?;
    // let mut udp = UdpTransport::new(
    //     "0.0.0.0:0".parse().unwrap(),
    //     "203.0.113.1:51820".parse().unwrap(),
    // )?;
    // let mut buf = vec![0u8; 2048];
    //
    // loop {
    //     if let Ok(packet) = tun.read(&mut buf) {
    //         udp.send(packet)?;
    //     }
    //     if let Ok(Some(n)) = udp.recv(&mut buf) {
    //         tun.write(&buf[..n])?;
    //     }
    // }
    let server_addr = "203.0.113.1:51820".parse().unwrap();
    echo::run_echo(server_addr)
}
