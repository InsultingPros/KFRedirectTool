# KF UZ2 Lib

[<img alt="github" src="https://img.shields.io/badge/github-kfuz2_lib-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/InsultingPros/KFRedirectTool/tree/main/crates/kfuz2_lib 'github') [<img alt="crates.io" src="https://img.shields.io/crates/v/kfuz2_lib?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/kfuz2_lib 'crates.io') [<img alt="docs.rs" src="https://img.shields.io/docsrs/kfuz2_lib?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/kfuz2_lib 'docs.rs') [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/InsultingPros/KFRedirectTool/build.yml?style=for-the-badge" height="20">](https://github.com/InsultingPros/KFRedirectTool/actions/workflows/build.yml 'build status')

A Rust library for creating `uz2` redirect files for Killing Floor 1 / Unreal Engine 2.5 games.

```toml
[dependencies]
kfuz2_lib = "1"
```

Compiler support: requires rustc **1.85+**

## Features

- Contains core functionality for compressing-decompressing unreal files.
- [zlib-rs](https://github.com/memorysafety/zlib-rs) is used to achieve much [faster file processing](../../docs/Benchmark.md) compared to all analoges.
- Cross-platform: supports Windows, Linux, and MacOS.
- Minimal dependency footprint for lightweight integration.
- Optionally check for default Killing Floor 1 files, and omit them from processing.

## Usage

```rust
use kfuz2_lib::helper::try_to_compress;
use kfuz2_lib::types::InputArguments;

let mut input_arguments = InputArguments {
        input_path: path_to_unreal_file,
        output_path: path_to_desired_output_directory,
        log_level: kfuz2_lib::types::LogLevel::Default,
        ignore_kf_files: true,
    };
try_to_compress(&mut input_arguments)?;
```

## Acknowledgments

- UZ2 package documentation - [UZ2 File Format](https://wiki.beyondunreal.com/UZ2_file#File_format) / [UT Package File Format v 1.6](https://archive.org/details/ut-package-file-format).
- Inspirations - [tinyuz2](https://unrealadmin.org/forums/showthread.php?t=10192) ([elmuerte](https://github.com/elmuerte)) and [uzLib](https://unrealadmin.org/forums/showthread.php?p=172927) ([Mc.Gugi](https://unrealadmin.org/forums/member.php?u=17138)). Not used directly, but were very helpful.
