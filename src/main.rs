mod dns;
mod server;

use dns::zone::Zone;
use server::Server;
use std::error::Error;
use std::net::{SocketAddrV4};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"), 
    version = env!("CARGO_PKG_VERSION"), 
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Opt {
    /// Listening address
    #[structopt(short, long, default_value = "0.0.0.0:5553")]
    address: SocketAddrV4,

    /// Zone names
    #[structopt(short, long = "name")]
    names: Vec<String>,

    /// This server's FQDN
    fqdn: String,

    /// This server's administrator's email address
    email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let email = opt
        .email
        .as_ref()
        .cloned()
        .unwrap_or_else(|| String::from("hostmaster.") + &opt.fqdn);

    let zones: Vec<Zone> = opt
        .names
        .iter()
        .map(|name| Zone::new(name, &opt.fqdn, &email))
        .collect();

    let server = Server::new(&opt.address, zones).await?;

    server.run().await?;

    Ok(())
}
