# KF UZ2

[![Build and Test](https://github.com/InsultingPros/KFRedirectTool/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/InsultingPros/KFRedirectTool/actions/workflows/build.yml) [![GitHub all releases](https://img.shields.io/github/downloads/InsultingPros/KFRedirectTool/total)](https://github.com/InsultingPros/KFRedirectTool/releases)

KF UZ2 is yet another compressor-decompressor designed for UE2-based games, offering few notable features:

- Supports all UE2 games but provides additional checks for KF1 files.
- Works on 64-bit Linux, MacOS, Windows.
- Does not require game DLLs or `UCC.exe`.
- Boasts the [fastest](../../Docs/Benchmark.md) file processing speed compared to other available compressors, thanks to the use of [zlib-ng](https://github.com/zlib-ng/zlib-ng).

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

```cmd
.\kfuz2.exe "D:\Documents\BitCore.u"
```

Compress a file in a different directory:

```cmd
.\kfuz2.exe "D:\Documents\BitCore.u" -o "D:\Documents\Redirect"
```

For decompression use `-d` argument.

## Credits

- UZ2 package documentation - [UZ2 File Format](https://wiki.beyondunreal.com/UZ2_file#File_format) / [UT Package File Format v 1.6](https://archive.org/details/ut-package-file-format).
- Inspirations - [elmuerte](https://github.com/elmuerte)'s [tinyuz2](https://unrealadmin.org/forums/showthread.php?t=10192) and [Mc.Gugi](https://unrealadmin.org/forums/member.php?u=17138)'s [uzLib](https://unrealadmin.org/forums/showthread.php?p=172927). Not used directly, but were very helpful.
