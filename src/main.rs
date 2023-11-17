use bincode;
use std::io;
use std::mem;
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use tracing::{debug, error, info};
use wrc::*;
#[tokio::main]
async fn main() -> io::Result<()> {
    //let mut interval = time::interval(Duration::from_millis(500));

    let sub = tracing_subscriber::FmtSubscriber::default();
    if let Err(err) = tracing::subscriber::set_global_default(sub) {
        println!("{err}")
    }
    const BUF_SIZE: usize = mem::size_of::<wrc::Packet>();
    let sock = UdpSocket::bind("0.0.0.0:6969").await?;
    let mut buf = [0; BUF_SIZE];

    loop {
      //  interval.tick().await;
        let (len, _) = sock.recv_from(&mut buf).await?;

        let packet: wrc::Packet = wrc::Packet::try_from(&buf).expect("Error deserializing");

        info!("Read {:?} bytes from UDP stream.", len);
        if let Ok(res) = serde_json::to_string_pretty(&packet) {
            info!("{res}")
        }
    }
}
