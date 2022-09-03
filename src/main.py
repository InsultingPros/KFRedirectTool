# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

# various
from sys import argv
from os import chdir, walk
from subprocess import run
from pathlib import Path
# QT
from PyQt5.QtWidgets import QMainWindow, QApplication, QFileDialog, QLabel
from PyQt5.QtCore import QSettings
from PyQt5 import uic
# packages
from VanillaPackages import unrealExtensions, disallowedPackages
import importlib.resources # import files, as_file


class MainWindow(QMainWindow):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        layout = importlib.resources.path('resources', 'layout.ui') # files('resources').joinpath('layout.ui')
        uic.loadUi(layout, self)

        # some global variable that we will use later
        # KFTinyUZ2 = files('resources').joinpath('KFTinyUZ2.exe')
        self.KFTinyUZ2 = importlib.resources.path('resources', 'KFTinyUZ2.exe')
        self.myDir = self.KFTinyUZ2.parent
        self.filesToCompress: list[str] = []

        self.outputFolder = ''
        self.settings = QSettings('KFRedirectTool', 'RedirectTool')
        self.readSettings()

        self.refreshFilesToCompress()
        print(self.filesToCompress)
        # set working directory
        chdir(self.myDir)

        # output field
        self.updateLabels(self.label_3, self.outputFolder)
        self.pushButton_5.clicked.connect(lambda: self.selectFolders())

        # tab #1
        # self.listWidget.setDragDropMode()
        self.pushButton.clicked.connect(lambda: self.compress())
        self.pushButton_2.clicked.connect(lambda: self.deCompress())
        self.pushButton_3.clicked.connect(lambda: self.open_Output())
        self.pushButton_4.clicked.connect(lambda: self.clearSelection())

        # menu bar
        self.menuHelp.triggered.connect(lambda: self.clckHelp())
        self.actionQuit.triggered.connect(self.close)

    # def dragEnterEvent(self, event):
    #     if event.mimeData().hasUrls():
    #         event.accept()
    #     else:
    #         event.ignore()

    # def dropEvent(self, event):
    #     files = [u.toLocalFile() for u in event.mimeData().urls()]
    #     for f in files:
    #         print(f)

    def execKFTinyUZ2(self, mode: int, pathToFile) -> None:
        """start to compress / decompress files with KFTinyUZ2"""
        print('execKFTinyUZ2 called!')
        match mode:
            case 'compress':
                run(('KFTinyUZ2.exe', '-c', '-o', self.outputFolder, pathToFile))
            case 'decompress':
                run(('KFTinyUZ2.exe', '-d', '-o', self.outputFolder, pathToFile))
            case 'info':
                run(('KFTinyUZ2.exe', '-s', pathToFile))
            case 'test':
                run(('KFTinyUZ2.exe', '-t', pathToFile))
            case _:
                print('execKFTinyUZ2: Illegal mode selected!')
                pass

        # tagetFile = str(tinyuz2.parent.parent.parent / 'test' / 'KF-Suburbia.rom')
        # # tagetFile = repr(tagetFile)
        # print(tagetFile)
        # print('KFTinyUZ2.exe -c ' + '"' + tagetFile + '"')

        # with as_file(source) as eml:
        #     print('done!')

        # print(disallowedPackages)

    def readSettings(self) -> None:
        """read and reuse last used output and game directories"""
        if self.settings.contains('OutputDirectory'):
            self.outputFolder = self.settings.value('OutputDirectory')
            print(self.settings.value('OutputDirectory'))

    def refreshFilesToCompress(self):
        """Is this a real game folder with UE packages?"""
        self.filesToCompress.clear()
        if not Path(self.outputFolder).exists:
            print('This is not a valid path!')
            pass
        else:
            for root, dirs, files in walk(self.outputFolder):
                for file in files:
                    if file.endswith(unrealExtensions) and file.lower() not in disallowedPackages:
                        self.filesToCompress.append(Path(root) / file)

    def updateLabels(self, QLabel: QLabel, content: str) -> None:
        """comment"""
        print('labels updated')
        QLabel.setText(content)

    def clckHelp(self) -> None:
        """comment"""
        print('help clicked!')

    def open_Output(self) -> None:
        """comment"""
        print('open output clicked!')

    def selectFolders(self) -> None:
        """Select Output and Game folders."""
        print('output clicked!')
        self.outputFolder = QFileDialog.getExistingDirectory(self, "Select Directory")
        self.settings.setValue('OutputDirectory', self.outputFolder)
        self.updateLabels(self.label_3, self.outputFolder)
        print(f'settings file saved with {self.outputFolder}')

    def compress(self):
        """comment"""
        print('compress clicked!')
        for x in self.filesToCompress:
            self.execKFTinyUZ2('compress', x)

    def deCompress(self):
        """comment"""
        print('decompress clicked!')

    def clearSelection(self):
        """comment"""
        print('clearSelection clicked!')


def main() -> None:
    app = QApplication(argv)

    window = MainWindow()
    window.show()

    app.exec_()


if __name__ == "__main__":
    main()