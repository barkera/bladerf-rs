libbladerf bindings for Rust
============================
`libbladerf-rs` provides bindings and useful constructs for interacting with
Nuand's [libbladerf](https://github.com/Nuand/bladeRF) library. This crate is
currently targeting `v2.x.x` in order to support both the BladeRF 1 and
BladeRF 2/micro platforms.

This crate is very much a WIP and currently does not have the majority of the
functions wrapped for the library. Development is currently targeted towards
RX functionality and then will be moved on to TX before other functionality is
wrapped.

## Usage
Currently this crate is not published to [crates.io](https://crates.io) and will
not be published until the majority of `libbladerf` has been wrapped. For now,
if you really want to use this crate, add the following to your `Cargo.toml`:
```toml
[dependencies.libbladerf]
git = "git://github.com/barkera/bladerf-rs.git"
```

For now, there is another crate, [bladerf](https://github.com/ryankurte/rust-bladerf)
which will support older versions of `libbladerf` and the BladeRF 1, but will
not work with the BladeRF 2/micro.
