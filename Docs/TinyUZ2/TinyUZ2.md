# TinyUZ2 Info

> [Go back to README](../README.md)

> Yeah we are using our python based script instead of this app, but this was used originally and all the info can be useful for someone.

To use this for KF 1, we need to patche [`TinyUZ2.exe`](https://forums.epicgames.com/unreal-tournament-2003-2004/user-maps-mods/full-releases/93640-tinyuz2)'s unreal package [header](https://wiki.beyondunreal.com/Unreal_package#Package_header) `9E2A83C1` (`2653586369`) to KF1 compatible `9E2A83C2` (`2653586370`).

- Install [Python 3.10.x](https://www.python.org/).
- Put [script](TinyUZ2Patcher.py) in the same directory with `TinyUZ2.exe`.
- Run and follow the instructions.
  - You can apply the patch / revert the patch / exit the script.
- If you plan to share it - do not forget to refer the [license](LICENSE-TinyUZ2).
- Enjoy.

## TinyUZ2 Commands

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

You gotta use it with command prompt. Example:

```text
CD C:\<PATH TO FILE>\tinyuz2
tinyuz2 -C FILEPATH
tinyuz2 -D FILEPATH
```
