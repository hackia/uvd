use crate::remote::{add_remote, remove_remote};
use crate::uvd::{
    adding_dependency, create_usb, create_uvd, info, install, list, new, publish, reinstall,
    remove_dependency, search, uninstall, update, upgrade, verify,
};
use anyhow::Error;
use clap::{Arg, Command};

pub mod license;
pub mod output;
pub mod remote;
pub mod uvd;

#[allow(clippy::too_many_lines)]
fn cli() -> clap::ArgMatches {
    Command::new("uvd")
        .version("0.1.0")
        .author("Willy Micieli <dev@hackia.org>")
        .about("An unix virtual disk toolkit")
        .subcommand(
            Command::new("install")
                .about("Install a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to install"),
                ),
        )
        .subcommand(
            Command::new("reinstall")
                .about("Reinstall a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to reinstall"),
                ),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to uninstall"),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Search for a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to search"),
                ),
        )
        .subcommand(Command::new("list").about("List all unix virtual disks"))
        .subcommand(
            Command::new("add")
                .about("add dependencies to the unix virtual disk")
                .arg(Arg::new("deps").required(true).index(1)),
        )
        .subcommand(
            Command::new("rm")
                .about("remove a dependencies to the unix virtual disk")
                .arg(Arg::new("deps").required(true).index(1)),
        )
        .subcommand(Command::new("login").about("Login  to the unix virtual disk hub"))
        .subcommand(Command::new("logout").about("Logout from the unix virtual disk hub"))
        .subcommand(
            Command::new("verify")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to verify"),
                )
                .about("Verify a unix virtual disk"),
        )
        .subcommand(
            Command::new("info")
                .about("Get info for a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to get info"),
                ),
        )
        .subcommand(Command::new("create-uvd").about("Build a unix virtual disk from source code"))
        .subcommand(Command::new("publish").about("Publish a unix virtual disk"))
        .subcommand(
            Command::new("remote")
                .about("Manage a unix virtual disk remote url")
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
                .about("Update a unix virtual disk")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to update"),
                ),
        )
        .subcommand(
            Command::new("create-usb")
                .about("Create a unix virtual disk usb drive")
                .arg(
                    Arg::new("uvd")
                        .required(true)
                        .index(1)
                        .help("The unix virtual disk to create usb drive"),
                )
                .arg(
                    Arg::new("usb")
                        .required(true)
                        .index(2)
                        .help("The usb drive to copy to"),
                ),
        )
        .subcommand(Command::new("upgrade").about("Upgrade all unix virtual disk"))
        .subcommand(Command::new("new").about("Start a new unix virtual disk project"))
        .subcommand(Command::new("archive").about("Archive the source code"))
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli();
    if let Some(sub) = matches.subcommand_matches("install") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return install(uvd);
    }
    if let Some(sub) = matches.subcommand_matches("reinstall") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return reinstall(uvd);
    }
    if let Some(sub) = matches.subcommand_matches("uninstall") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return uninstall(uvd);
    }
    if let Some(sub) = matches.subcommand_matches("search") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return search(uvd);
    }
    if let Some(sub) = matches.subcommand_matches("update") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return update(uvd);
    }
    if matches.subcommand_matches("upgrade").is_some() {
        return upgrade();
    }
    if matches.subcommand_matches("list").is_some() {
        return list();
    }
    if let Some(sub) = matches.subcommand_matches("verify") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return verify(uvd);
    }
    if let Some(sub) = matches.subcommand_matches("info") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        return info(uvd);
    }
    if matches.subcommand_matches("publish").is_some() {
        return publish();
    }
    if let Some(sub) = matches.subcommand_matches("create-usb") {
        let uvd = sub.get_one::<String>("uvd").unwrap();
        let usb = sub.get_one::<String>("usb").unwrap();
        return create_usb(uvd, usb);
    }
    if matches.subcommand_matches("login").is_some() {
        return uvd::hub::login().await;
    }
    if let Some(sub) = matches.subcommand_matches("remote") {
        if let Some(sub) = sub.subcommand_matches("add") {
            let name = sub.get_one::<String>("name").unwrap();
            let url = sub.get_one::<String>("url").unwrap();
            return add_remote(name, url);
        }
        if let Some(sub) = sub.subcommand_matches("remove") {
            let name = sub.get_one::<String>("name").unwrap();
            return remove_remote(name);
        }
    }
    if let Some(sub) = matches.subcommand_matches("add") {
        let deps = sub.get_one::<String>("deps").unwrap();
        return adding_dependency(deps);
    }
    if let Some(sub) = matches.subcommand_matches("rm") {
        let deps = sub.get_one::<String>("deps").unwrap();
        return remove_dependency(deps);
    }
    if matches.subcommand_matches("create-uvd").is_some() {
        return create_uvd();
    }
    if matches.subcommand_matches("new").is_some() {
        return new();
    }
    Ok(())
}
