use crate::remote::{add_remote, remove_remote};
use crate::uvd::{
    adding_dependency, archive, build, export, info, install, list, publish, reinstall,
    remove_dependency, search, uninstall, update, upgrade, verify,
};
use anyhow::Error;
use clap::{Arg, Command};

pub mod output;
pub mod remote;
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
        .subcommand(Command::new("list").about("List all universal verified discs"))
        .subcommand(
            Command::new("add")
                .about("add dependencies to the universal verified disc")
                .arg(Arg::new("deps").required(true).index(1)),
        )
        .subcommand(
            Command::new("rm")
                .about("remove a dependencies to the universal verified disc")
                .arg(Arg::new("deps").required(true).index(1)),
        )
        .subcommand(Command::new("login").about("Login  to the universal verified disc hub"))
        .subcommand(Command::new("logout").about("Logout from the universal verified disc hub"))
        .subcommand(Command::new("verify").about("Verify a universal verified disc"))
        .subcommand(
            Command::new("info")
                .about("Get info for a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to get info"),
                ),
        )
        .subcommand(Command::new("build").about("Build a universal verified disc from source code"))
        .subcommand(Command::new("publish").about("Publish a universal verified disc"))
        .subcommand(
            Command::new("remote")
                .about("Manage a universal verified disc remote url")
                .subcommand(
                    Command::new("add")
                        .about("Add a remote url")
                        .arg(Arg::new("name").required(true))
                        .arg(Arg::new("url").required(true)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a remote url")
                        .arg(Arg::new("name").required(true)),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to update"),
                )
                .arg(
                    Arg::new("usb")
                        .required(true)
                        .index(2)
                        .help("The universal verified disc to update"),
                ),
        )
        .subcommand(
            Command::new("export")
                .about("Export a universal verified disc")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The universal verified disc to update"),
                )
                .arg(
                    Arg::new("usb")
                        .required(true)
                        .index(2)
                        .help("The universal verified disc to update"),
                ),
        )
        .subcommand(Command::new("upgrade").about("Upgrade all universal verified discs"))
        .subcommand(Command::new("archive").about("Archive the source code"))
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
    if let Some(sub) = matches.subcommand_matches("update") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return update(uvd).await;
    }
    if let Some(_) = matches.subcommand_matches("upgrade") {
        return upgrade().await;
    }
    if let Some(_) = matches.subcommand_matches("list") {
        return list().await;
    }
    if let Some(_) = matches.subcommand_matches("login") {
        return uvd::hub::login().await;
    }
    if let Some(_) = matches.subcommand_matches("logout") {
        return uvd::hub::logout().await;
    }
    if let Some(sub) = matches.subcommand_matches("verify") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return verify(uvd).await;
    }
    if let Some(sub) = matches.subcommand_matches("info") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return info(uvd).await;
    }
    if let Some(_) = matches.subcommand_matches("publish") {
        return publish().await;
    }
    if let Some(_) = matches.subcommand_matches("build") {
        return build().await;
    }
    if let Some(sub) = matches.subcommand_matches("export") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        let usb = sub.get_one::<String>("usb").unwrap();
        return export(uvd, usb).await;
    }
    if let Some(sub) = matches.subcommand_matches("remote") {
        if let Some(sub) = sub.subcommand_matches("add") {
            let name = sub.get_one::<String>("name").unwrap();
            let url = sub.get_one::<String>("url").unwrap();
            return add_remote(name, url).await;
        }
        if let Some(sub) = sub.subcommand_matches("remove") {
            let name = sub.get_one::<String>("name").unwrap();
            return remove_remote(name).await;
        }
    }
    if let Some(sub) = matches.subcommand_matches("add") {
        let deps = sub.get_one::<String>("deps").unwrap();
        return adding_dependency(deps).await;
    }
    if let Some(sub) = matches.subcommand_matches("rm") {
        let deps = sub.get_one::<String>("deps").unwrap();
        return remove_dependency(deps).await;
    }
    if let Some(_) = matches.subcommand_matches("archive") {
        return archive().await;
    }
    Ok(())
}
