# KF1 Redirect Tool

[**TinyUZ2**](https://forums.epicgames.com/unreal-tournament-2003-2004/user-maps-mods/full-releases/93640-tinyuz2) detects unreal packages by starting bits `9E2A83C1`. But Killing Floor 1 changed that to `9E2A83C2`, so we patched the executable to use KF's value instead. All credits for `KFTinyUZ2.exe` goes to [Michiel Hendriks](https://github.com/elmuerte), my only contribution is that slight modification.

## Patching TinyUZ2

Check [PATCHER.md](Docs/PATCHER.md)

## Building

Check [BUILD.md](Docs/BUILD.md).

<https://doc.qt.io/qtforpython/deployment-nuitka.html>
<https://doc.qt.io/qtforpython/deployment-fbs.html>

## KFTinyUZ2 Commands

```cpp
%s [-c|-d|-s|-h] [-v|-q] [-t] [-o <path>] file
```

- `-c` compress file (default).
- `-d` decompress file.
- `-h` helper message.
- `-o <path>` set output path.
- `-s` show file information (GUID, GEN, ...).
- `-t` test mode, don't create output files.
- `-q` quiet.
- `-v` increase verbosity.

## Example

You gotta use it with command prompt.

```text
Example: (CD C:\Unzipped\tinyuz2_1.2.1-win32\tinyuz2)
Then, you do this.
tinyuz2 -C FILEPATH - Compress
tinyuz2 -D FILEPATH - Decompress.
```

## Useful links

- [UnrealDeps](https://unrealadmin.org/forums/showthread.php?t=30406)
- [UZ2 File Info](https://wiki.beyondunreal.com/UZ2_file)

## Credits

- [Michiel Hendriks](https://github.com/elmuerte) for TinyUZ2.
- [dkanus](https://github.com/dkanus) for helping me with stupid questions.
