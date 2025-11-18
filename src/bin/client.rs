use vpn::core::echo;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_addr = "203.0.113.1:51820".parse().unwrap();
    echo::run_echo(server_addr)
}
