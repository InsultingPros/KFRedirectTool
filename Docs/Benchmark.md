[tinyuz2]: https://unrealadmin.org/forums/showthread.php?t=10192
[uzLib]: https://unrealadmin.org/forums/showthread.php?p=172927
[zlib-ng]: https://github.com/zlib-ng/zlib-ng

# Speed Comparison

> [Go back to README](../kfuz2/README.md)

> **Note** most of difference comes from [zlib-ng] usage. Underlying algorithms, error handling are almost identical.

Test Setup:

- Hardware - Core i5 4570 / 860 evo ssd.
- Software - Win 10 latest, 0 active background processes.
- Test directory - whole [UT2K4 Steam client](https://store.steampowered.com/app/13230/Unreal_Tournament_2004_Editors_Choice_Edition/).

Compression, time checked with python script.

| kfuz2 | [uzLib] | [tinyuz2] |
|---|---|---|
| 136s | 224.4s | 222.8s |
