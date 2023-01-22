use std::io;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let (res, port) = tokio::select! {
        x = tokio::spawn(run_server(9876)) => (x, 9876),
        x = tokio::spawn(run_server(9877)) => (x, 9877),
        x = tokio::spawn(run_server(9878)) => (x, 9878),
    };
    res.expect(&format!("port {port} join error"))
        .expect(&format!("port {port} server error"))
}

async fn run_server(port: u16) -> io::Result<()> {
    let sock = UdpSocket::bind(format!("0.0.0.0:{port}")).await?;
    println!("listening on {port}");

    let mut buf = [0u8; 256];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("port {port}: {len} bytes recieved from {addr}");
        println!(
            "port {port}: buffer contents: {}",
            String::from_utf8_lossy(&buf)
        );
    }
}
