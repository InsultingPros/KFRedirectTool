# Build Information

> [Go back to README](../README.md)

## Requirements

- C/C++ compiler like Visual Studio or GCC.
- [Python 3.10.6](https://www.python.org/downloads/release/python-3106/).
- [requirements.txt](../requirements/base.txt 'check the dependencies').

## Compilation

```cmd
python -m nuitka --onefile --windows-disable-console --enable-plugin=pyqt5 uifile.py
```
