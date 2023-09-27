[build_badge]: https://img.shields.io/github/actions/workflow/status/InsultingPros/KFRedirectTool/build.yml?style=for-the-badge
[release_badge]: https://img.shields.io/github/downloads/InsultingPros/KFRedirectTool/total?style=for-the-badge

# KF UZ2

[![build_badge]](https://github.com/InsultingPros/KFRedirectTool/actions/workflows/build.yml) [![release_badge]](https://github.com/InsultingPros/KFRedirectTool/releases)

> [Go back to README](../../README.md)

KF UZ2 is yet another compressor-decompressor designed for UE2-based games, offering few notable features:

- Supports all UE2 games but provides additional checks for KF1 files.
- Works on 64-bit Linux, MacOS, Windows.
- Does not require game DLLs or `UCC.exe`.
- Boasts the [fastest](../../docs/Benchmark.md) file processing speed compared to other available compressors, thanks to the use of [zlib-ng](https://github.com/zlib-ng/zlib-ng).

## Supported CLI arguments

The following optional arguments can be used:

- `-h` / `--help`: Prints the help message.
- `-q` / `--quiet`: This option ensures silent operation, providing no feedback or information during file processing. Activating this mode will override `-v` option.
- `-v` / `--verbose`: This option enables detailed operation, displaying extensive information during file processing. If both this and the `-q` option are active, the quiet mode will take precedence, suppressing the verbose output.
- `-o` / `--output <directory>`: Specifies the target directory. If not provided, processed files will be saved in the same directory as the input file.
- `-d` / `--decompress <file>`: Decompresses the input file. If not used, the input file will be compressed.
- `--nocheck`: Disables the additional check for verifying if the input file matches KF1's format or belongs to one of its built-in packages.

## Usage Examples

Compress a file in the same directory:

```bash
.\kfuz2_cli.exe "D:\Documents\BitCore.u"
```

Compress a file in a different directory:

```bash
.\kfuz2_cli.exe "D:\Documents\BitCore.u" -o "D:\Documents\Redirect"
```

For decompression use `-d` argument.
