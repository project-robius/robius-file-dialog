[![version](https://img.shields.io/crates/v/rfd.svg)](https://crates.io/crates/rfd)
[![Documentation](https://docs.rs/rfd/badge.svg)](https://docs.rs/rfd)
[![dependency status](https://deps.rs/crate/rfd/0.15.1/status.svg)](https://deps.rs/crate/rfd/0.15.1)

# `robius-file-dialogs`

`robius-file-dialogs` is a fork of the Rusty File Dialogs ([`rfd`](https://crates.io/crates/rfd)) crate, with minimal compatibility changes for use in the Robius project and other cross-platform applications.
In general, it is kept up to date with [upstream](https://github.com/PolyMeilex/rfd), where changes beneficial to the broader community are also shared.

## Description

Rusty File Dialogs is a cross platform Rust library for using native file open/save dialogs.
It provides both asynchronous and synchronous APIs. Supported platforms:

  * Windows
  * macOS
  * Linux & BSDs (GTK3 or XDG Desktop Portal)
  * WASM32 (async only)

Refer to the [documentation](https://docs.rs/rfd) for more details.

## Platform-specific notes

### Linux

Please refer to [Linux & BSD backends](https://docs.rs/rfd/latest/rfd/#linux--bsd-backends) for information about the needed dependencies to be able to compile on Linux.
