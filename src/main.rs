use crate::uvd::{install, reinstall, search, uninstall};
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
        .subcommand(
            Command::new("reinstall")
                .about("Reinstall a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to reinstall"),
                ),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to uninstall"),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Search for a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to search"),
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
    if let Some(sub) = matches.subcommand_matches("reinstall") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return reinstall(uvd).await;
    }
    if let Some(sub) = matches.subcommand_matches("uninstall") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return uninstall(uvd).await;
    }
    if let Some(sub) = matches.subcommand_matches("search") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return search(uvd).await;
    }
    Ok(())
}
