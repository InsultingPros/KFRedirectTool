# KF Redirect Tool

[build_badge]: https://img.shields.io/github/actions/workflow/status/InsultingPros/KFRedirectTool/build.yml?style=for-the-badge
[release_badge]: https://img.shields.io/github/downloads/InsultingPros/KFRedirectTool/total?style=for-the-badge
[egui]: https://www.egui.rs

[![build_badge]](https://github.com/InsultingPros/KFRedirectTool/actions/workflows/build.yml) [![release_badge]](https://github.com/InsultingPros/KFRedirectTool/releases)

KF UZ2 is yet another compressor-decompressor designed for UE2-based games, offering a few notable features:

- Supports all Unreal Engine 2 based games, but provides additional checks for KF1 files.
- Cross-platform - works on 64-bit Linux, MacOS, Windows.
- Does not require game DLLs or `UCC.exe`.
- Boasts the [fastest](docs/Benchmark.md) file processing speed compared to other available compressors, thanks to the use of [zlib-rs](https://github.com/trifectatechfoundation/zlib-rs).

## Crates

This project consists of:

- [kfuz2_lib](crates/kfuz2_lib/README.md) - core library for all other crates.
- [kfuz2_cli](crates/kfuz2_cli/README.md) - CLI application for compressing-decompressing UE2 based game files.
- [kfuz2_egui](crates/kfuz2_egui/README.md) - gui application based on [egui].

## Building

Just a simple `cargo build --release`, but for your linux distribution you may install `libgtk-3-dev`, (to compile part of [egui]).

## Testing

We are testing the CLI application, so it is required to limit the execution to 1 thread.

```bash
cargo test --locked -- --test-threads 1
```
