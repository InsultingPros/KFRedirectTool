[UZ2]: https://wiki.beyondunreal.com/UZ2_file
[TinyUZ2]: https://forums.epicgames.com/unreal-tournament-2003-2004/user-maps-mods/full-releases/93640-tinyuz2
[UnrealDeps]: https://unrealadmin.org/forums/showthread.php?t=30406
[KFTools UZ2]: https://www.moddb.com/games/killing-floor/downloads/kftools-uz2
[release]: https://github.com/InsultingPros/KFRedirectTool/releases

# KF1 Redirect Tool

[![GitHub all releases](https://img.shields.io/github/downloads/InsultingPros/KFRedirectTool/total)](https://github.com/InsultingPros/KFRedirectTool/releases)

Here we go, yet another [UZ2] compressor tool for Unreal. But why?

- Unreal 2-2.5 compressors ([TinyUZ2], [UnrealDeps] and others) do not support KF 1 packages.
- [KFTools UZ2] is buggy and require some specific `.Net` version. Aaaand somehow it can break with Windows updates.
- `bash / cmd / ps` scripts require a lot of manual work on writing-testing-editing-calling-etc.
  - They all require you to have the `UCC.exe` with all the proper dll files.

On the other hand KF1 Redirect Tool:

- Does not rely on KF1 binaries. So it is fully portable and can run everywhere.
- Works faster than UCC's `compress` / `decompress` commandlets. Or you can tweak the `zlib`'s `DEFLATE` level and make it compress more.
- Provides more information in console.
- Has a minimalistic and handy GUI.

![IMG](Docs/media/tkinter.PNG)

## Usage

First step: install [Python >3.10.x](https://www.python.org/) and download the latest [release].

- Run the **tkinterUZ2.py**.
- Choose `Input` folder, which contains KF1 packages.
  - You can compress the whole server at once, btw, it will omit vanilla packages.
- Choose `Output` folder if you want everything to be collected in one place. Else, new files will be created near originals.
- Press `compress` / `decompress` buttons, depending on what you want.
- Check the progress in the console.
- Done!

## Credits

- [dkanus](https://github.com/dkanus) - for helping me with dumb questions.
- [Vel](https://github.com/Vel-San) - for helping with python formatting, code enhancements.
- [elmuerte](https://github.com/elmuerte) - his [TinyUZ2] served me well with [KF1 patch](Docs/TinyUZ2/TinyUZ2.md).
