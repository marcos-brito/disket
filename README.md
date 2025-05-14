# disket

[![crates.io](https://img.shields.io/crates/v/disket.svg)](https://crates.io/crates/disket)
[![docs.rs](https://img.shields.io/badge/docs.rs-disket-blue)](https://docs.rs/disket)
[![build](https://img.shields.io/github/actions/workflow/status/marcos-brito/disket/check.yaml?branch=main)](https://github.com/marcos-brito/disket/actions?query=branch:main)
[![license](https://img.shields.io/badge/license-MIT-blue)](https://github.com/marcos-brito/disket/blob/main/LICENSE)

`disket` is a Rust cross-platform library for managing disks and volumes. It provides
high-level APIs for mount/unmounting, fetching information and watching arrival/removal of devices.

# Usage

To start using `disket`, add this to your `Cargo.toml`:

```toml
[dependencies]
disket = "1"
```

This will add the set of `default` features. Check the [documentation](https://docs.rs/disket) for other
features.

The following block shows some basic functionality. It retrieves disks, mounts every partition and does
the same whenever a new device arrives.

```rust
fn main() -> Result<(), Box<dyn Error>> {
    todo!()
}
```

# Supported platforms

Check the [support section](https://docs.rs/disket/index.html#support) of the documentation.

# Contributing

Contributions are very much welcome. You can just make a PR, and we'll discuss right there if
needed. If it is something you are not so sure about or anything that changes the public API, open an issue. 

New platforms don't need to be available for every module, but they should be fully compatible with the 
abstraction layer of the module they are in. A new platform for the `mount` module should be compatible
with its [public API](https://github.com/marcos-brito/disket/blob/main/src/mount/mod.rs).
