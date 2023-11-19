use clap::Arg;
use clap::{arg, command, value_parser, ArgAction, Command};
use wrc::WrcClient;
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
let client = WrcClient::default();

    tokio::spawn(async move {
        client.run(&address).await;

}).await.expect_err("Error joining thread.");
    Ok(())
}
