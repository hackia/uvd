# Universal verified disc

The universal verified disc, it's a project to replace DVD by a new technology.

DVD is a widely used optical disc format for storing digital data, including movies, music, and software.

However, DVD technology has limitations in terms of storage capacity and data security.

The universal verified disc aims to address these limitations by providing a more secure and reliable way to store and
verify data.

## Features

`uvd` pack software to run on `linux`, `windows`, `bsd` and `macos`, with the same commands.

If the software it's a free software, `uvd` the source code in `uvd` format.

## Install

```bash
cargo install uvd
```

## Usage

Three servers are provided for `uvd`:

- [hub.uvd.io](https://hub.uvd.io)
- [uvd.dev](https://uvd.dev)
- [api.uvd.io](https://api.uvd.io)

The server `hub.uvd.io` it's the uvd hub, it's the main server for `uvd`.

The server `uvd.dev` it's the uvd dev server, it's the server for build all `uvd` from source code.

The server `api.uvd.io` it's the uvd api server, it's the api server for `uvd` dependency manager and build server.

### For developers

Then you must use `uvd init` to initialize the project in your source code root directory.

Then you must use `uvd archive` to archive the project.

Then you must use `uvd submit` to upload the archive to the build server.

Then you must use `uvd run` to run the project.

### Basic usage

Then you can use `uvd login` to connect your packages to the uvd hub.

Then you can use `uvd install` to install packages from the uvd hub.

Then you can use `uvd install <uvd_name>` to install packages from the uvd hub.

Then you can use `uvd reinstall <uvd_name>` to reinstall packages from the uvd hub.

Then you can use `uvd uninstall <uvd_name>` to uninstall packages from the uvd hub.

Then you can use `uvd list` to list all packages installed from the uvd hub.

Then you can use `uvd logout` to disconnect from the uvd hub.

Then you can use `uvd list` to list all packages installed from the uvd hub.

Then you can use `uvd logout` to disconnect from the uvd hub.
