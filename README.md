# unreal_asset

[![Build status](https://github.com/AstroTechies/unreal_asset/workflows/CI/badge.svg)](https://github.com/AstroTechies/unrealmodding/actions?query=workflow%3ACI)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Unreal Engine 4/5 asset file parsing library. Initially developed for Astroneer.

## Crates

This repo contrains multiple user facing crates for working with Unreal Engine file formats and creating Mods.

### [unreal_asset](./unreal_asset/)

[![Documentation](https://docs.rs/unreal_asset/badge.svg)](https://docs.rs/unreal_asset/)
[![Crates.io](https://img.shields.io/crates/v/unreal_asset.svg)](https://crates.io/crates/unreal_asset)

This core crate allows for parsing of Unreal asset binary files. It is internally split into multiple sub-crates to
improve compile times.

### [unreal_helpers](./unreal_helpers/)

[![Documentation](https://docs.rs/unreal_helpers/badge.svg)](https://docs.rs/unreal_helpers/)
[![Crates.io](https://img.shields.io/crates/v/unreal_helpers.svg)](https://crates.io/crates/unreal_helpers)

Core crate that provides utilities for wotking with Unreal Engine binary files. It is relied on by all the other binary
parsing crates in this repo.

## License

Licensed under [MIT license](./LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, shall be licensed
as above, without any additional terms or conditions.
