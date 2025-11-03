use clap::{Arg, Command};

pub mod output;
pub mod uvd;
async fn install(uvd: &str) {}

fn cli() -> clap::ArgMatches {
    Command::new("uvd")
        .version("0.1.0")
        .author("Willy Micieli <dev@hackia.org>")
        .about("An Universal Verified Disc management toolkit")
        .subcommand(
            Command::new("install").about("Installs a UVD").arg(
                Arg::new("uvd")
                    .required(true)
                    .index(1)
                    .help("The UVD to install"),
            ),
        )
        .get_matches()
}
#[tokio::main]
async fn main() {
    let matches = cli();
    if let Some(sub) = matches.subcommand_matches("install") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
    }
    println!("Hello, world!");
}
