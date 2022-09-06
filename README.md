# KF1 Redirect Tool

[TinyUZ2]: https://forums.epicgames.com/unreal-tournament-2003-2004/user-maps-mods/full-releases/93640-tinyuz2
[UZ2Tool.py]: src/UZ2Tool.py
[tkinter app]: src/tkinterUZ2.py

Here we go, yet another UZ2 compressor tool for Unreal. But why?

- Unreal 2-2.5 compressors ([TinyUZ2], [UnrealDeps](https://unrealadmin.org/forums/showthread.php?t=30406) and others) do not support KF 1 packages.
- [KFTools UZ2](https://www.moddb.com/games/killing-floor/downloads/kftools-uz2) is buggy and require some specific `.Net` version. Aaaand somehow it can break with Windows updates.
- `bash / cmd / ps` scripts require a of lot of manual work on writting-testing-editing-calling-etc.
  - They all require you to have the `UCC.exe` with all the proper dll's.
- Most mod makers are lazy / ignorant or people that do not realize that redirect files are crucial for server hosters. So their main user sources will suffer and will be forced to compress files on their own. If they even decide to use that half done mod ofc.

On the other hand [UZ2Tool.py]:

- Does not rely on KF1 binaries. So it is fully portable and can run everywhere.
- Works faster than UCC's `compress / decompress` commandlets. Or you can tweak the `zlib`'s `DEFLATE` level and make it compress more.
- Provides more information in console.

And on top of that, we provide you ~~two apps~~ [tkinter app]. QT app and standalone executables are in progress!

## Usage

First step: install [Python 3.10.x](https://www.python.org/). Older versions are not supported.

[UZ2Tool.py]:

- CLI support inc.

[tkinter app]:

- Run the script.
- Choose `Input` folder, which contains KF1 packages.
  - You can compress the whole server at once, btw, it will omit vanilla packages.
- Choose `Output` folder if you want everything to be collected in one place. Else, new files will be created near originals.
- Press `compress` / `decompress` buttons, depending on what you want.
- Check the progress in the console.
- Done!

## Documentation

- [UZ2 File Info](https://wiki.beyondunreal.com/UZ2_file): `UZ2` file internals and how-to's.
- [Vanilla Packages](Docs/VanillaPackages.md): a handy list.
- [Archived] [TinyUZ2 Info](Docs/TinyUZ2/TinyUZ2.md): how to patch it for KF1.

## Building

Check [BUILD.md](Docs/BUILD.md).

## Useful links

wipe me later!

- `pyside6-designer.exe`
- <https://doc.qt.io/qtforpython/deployment-nuitka.html>
- <https://doc.qt.io/qtforpython/deployment-fbs.html>
- <https://nuitka.net/doc/user-manual.html#onefile-finding-files>
- <https://doc.qt.io/qt-6/designer-using-a-ui-file.html>

## Credits

- [dkanus](https://github.com/dkanus) for helping with dumb questions.
- [Vel](https://github.com/Vel-San) for helping with python formatting, code enhancements.
- [elmuerte](https://github.com/elmuerte) for [TinyUZ2]. It served me well.
