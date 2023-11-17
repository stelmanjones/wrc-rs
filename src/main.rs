use clap::Arg;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::io;
use std::mem;
use tokio::net::UdpSocket;
use tracing::{debug, error, info, Level};

#[tokio::main]
async fn main() -> io::Result<()> {
    //let mut interval = time::interval(Duration::from_millis(500));

    let app = command!()
        .arg(arg!([addr] "Set the UDP socket address to bind."))
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("If set, changes tracing level to debug.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let address: String;
    let level: tracing::Level;

    if let Some(dbg) = app.get_one::<bool>("debug") {
        match dbg {
            true => level = Level::DEBUG,
            false => level = Level::INFO,
        }
    } else {
        level = Level::INFO;
    }
    let sub = tracing_subscriber::FmtSubscriber::builder()
        .with_line_number(true)
        .with_ansi(true)
        .with_max_level(level)
        .pretty()
        .finish();

    if let Err(err) = tracing::subscriber::set_global_default(sub) {
        println!("{err}")
    }

    if let Some(addr) = app.get_one::<String>("addr") {
        address = addr.to_string();
        debug!("{address}")
    } else {
        address = "0.0.0.0:6969".to_string();
        debug!("{address}");
    }

    let sock = UdpSocket::bind(address).await?;
    info!("WRC Telemetry server running!");
    let mut buf = [0; wrc::PACKET_SIZE];

    loop {
        //  interval.tick().await;
        let (len, _) = sock.recv_from(&mut buf).await?;

        let packet: wrc::Packet = wrc::Packet::try_from(&buf).expect("Error deserializing");

        debug!("Read {:?} bytes from UDP stream.", len);
        if let Ok(res) = serde_json::to_string_pretty(&packet) {
            info!("{res}")
        } else {
            error!("Could not perform JSON serialization.")
        }
    }
}
