# Tkinter GUI for `KFUZ2`

[![GitHub all releases](https://img.shields.io/github/downloads/InsultingPros/KFRedirectTool/total)](https://github.com/InsultingPros/KFRedirectTool/releases)

This GUI app is meant to be used with Killing Floor 1 and supports multithreading (leading to x2-3 speed up compared to one by one file processing).

*N.B.* If you want to use it with any other UE2-based games, you need to do few small modifications - disable KF1 specific checks and edit default extension list.

![IMG](../Docs/media/tkinter.PNG)

## Usage

First step: install [Python >3.10.x](https://www.python.org/).

- Download **tkinterGUI.py**, `kfuz2` executable for your OS from release section, and put them in the same directory.
- Choose `Input` folder, which contains game packages.
  - You can compress the whole server at once, btw, it will omit vanilla packages.
- Choose `Output` folder if you want everything to be collected in one place. Else, new files will be created near originals.
- Press `compress` / `decompress` buttons, depending on what you want.
- Check the progress in the console.
- Done!

Supported operating systems: Windows, Linux, MacOS.

## Credits

- [dkanus](https://github.com/dkanus) - for helping me with dumb questions.
- [Vel](https://github.com/Vel-San) - for helping with python formatting, code enhancements.
