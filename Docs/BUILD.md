# GUI Applications Build Information

> [Go back to README](../README.md)

Contains all required details for building process.

## QT App

### Requirements

- C/C++ compiler like Visual Studio or GCC.
- [Python 3.10.6](https://www.python.org/downloads/release/python-3106/).
- [requirements.txt](../requirements/qt.txt 'check the dependencies').

### Compilation

```cmd
python -m nuitka --onefile --windows-disable-console --enable-plugin=pyqt5 uifile.py
```

Or make a python script:

```python
from subprocess import run
from os import chdir

myDir = r'D:\Documents\Killing Floor Archive\Projects\KFRedirectTool\src\build'
cmd: tuple = ('python',
                '-m',
                'nuitka',
                '--mingw64',
                '--onefile',
                '--disable-console',
                r'--include-data-dir=D:\Documents\Killing Floor Archive\Projects\KFRedirectTool\src\resources=resources',
                '--enable-plugin=pyqt5',
                r'D:\Documents\Killing Floor Archive\Projects\KFRedirectTool\src\main.py')

chdir(myDir)
run(cmd, shell=True)

input('Press any key to continue...')
```

### Tkinter App Build Information

TODO
