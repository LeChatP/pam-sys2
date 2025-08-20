# pam-sys2 - Rust FFI bindings to the Linux and Open Pluggable Authentication Modules (PAM)

[![Crates.io](https://img.shields.io/crates/v/pam-sys.svg)](https://crates.io/crates/pam-sys)
[![Documentation](https://docs.rs/pam-sys/badge.svg)](https://docs.rs/pam-sys/)
[![License](https://img.shields.io/crates/l/pam-sys.svg?branch=master)](https://travis-ci.org/1wilkens/pam-sys)

This crate uses [`bindgen`](https://github.com/rust-lang/rust-bindgen) to generate the raw FFI
definitions for PAM. For a rustified API consider using [`pam`](https://github.com/1wilkens/pam).

## Fork maintenance

Hello! I forked this crate to provide maintenance and updates, since the original author has not been active for several years. I will make sure the crate remains functional and up to date with the latest Rust, Linux-PAM and OpenPAM developments.

If you’d like to join the discussion, please open an issue.

## Changes

This crate is a fork of the original `pam-sys` crate, with the following changes:
- Updated to use the latest version of `bindgen` for better compatibility and features.
- update Rust edition to 2021
- Added support for both Linux-PAM and OpenPAM (Merging 1wilkens/pam-sys/pulls/28 by @coastalwhite with some edits).
- Use pre-generated bindings for Linux-PAM by default (by @coastalwhite).
- Fixing zigbuild issue (1wilkens/pam-sys/issues/32)
- Added `generate-bindings` feature to control when bindings are generated.

## Supported Rust versions (MSRV)
The library is only continuously built against Rust stable, beta and nightly but as it does not use
a lot of new language features it should probably compile on older versions as well. The MSRV is
mostly determined by the version of `bindgen` used. If you encounter problems building on older
versions and a small fix can be applied to make the build succeed, consider opening a pull request.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
