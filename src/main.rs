use crate::uvd::install;
use anyhow::Error;
use clap::{Arg, Command};

pub mod output;

pub mod uvd;

fn cli() -> clap::ArgMatches {
    Command::new("uvd")
        .version("0.1.0")
        .author("Willy Micieli <dev@hackia.org>")
        .about("An Universal Verified Disc management toolkit")
        .subcommand(
            Command::new("install")
                .about("Install a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to install"),
                ),
        )
        .get_matches()
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli();
    if let Some(sub) = matches.subcommand_matches("install") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return install(uvd).await;
    }
    Ok(())
}
