# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool

# various
from sys import argv
from os import chdir
from subprocess import run
# QT
from PyQt5.QtWidgets import QMainWindow, QApplication, QFileDialog, QLabel
from PyQt5.QtCore import QSettings
from PyQt5 import uic
# packages
# from VanillaPackages import disallowedPackages
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

        # set working directory
        chdir(self.myDir)

        # enable drag-n-drop support
        self.setAcceptDrops(True)
        # output field
        # at first check the last used output folder
        self.settings = QSettings('KFRedirectTool', 'RedirectTool')
        if self.settings.contains('OutputDirectory'):
            self.output = self.settings.value('OutputDirectory')
            print(self.settings.value('OutputDirectory'))
        else:
            print('Output folder not found it config file')

        self.updateLabels(self.label_3, self.output)
        self.pushButton_5.clicked.connect(lambda: self.select_Output())
        self.toolButton_3.clicked.connect(lambda: self.toolbtn_Output())

        # tabs
        self.tabWidget.tabBarClicked.connect(lambda: self.clckTab())

        # tab #1
        # self.listWidget.setDragDropMode()
        self.pushButton.clicked.connect(lambda: self.compress())
        self.pushButton_2.clicked.connect(lambda: self.deCompress())
        self.toolButton.clicked.connect(lambda: self.toolbtn_Output())

        # tab #2
        self.toolButton_5.clicked.connect(lambda: self.toolbtn_Output())
        self.pushButton_9.clicked.connect(lambda: self.select_GameFolder())
        self.pushButton_3.clicked.connect(lambda: self.open_Output())
        self.pushButton_4.clicked.connect(lambda: self.compressAll())

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
        match mode:
            case 'compress':
                run(('KFTinyUZ2.exe', '-c', '-o', self.output, pathToFile))
            case 'decompress':
                run(('KFTinyUZ2.exe', '-d', '-o', self.output, pathToFile))
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

    def compressAll(self) -> None:
        """comment"""
        print('compress all clicked!')

    def select_Output(self) -> None:
        """comment"""
        print('output clicked!')
        self.output = QFileDialog.getExistingDirectory(self, "Select Directory")
        self.settings.setValue('OutputDirectory', self.output)
        self.updateLabels(self.label_3, self.output)
        print(f'settings file saved with {self.output}')

    def select_GameFolder(self):
        """comment"""
        print('game folder clicked!')

    def toolbtn_Output(self):
        """comment"""
        print('toolbtn clicked!')

    def compress(self):
        """comment"""
        print('compress clicked!')

    def deCompress(self):
        """comment"""
        print('decompress clicked!')

    def clckTab(self):
        """comment"""
        print('tabs clicked!')


if __name__ == "__main__":
    app = QApplication(argv)

    window = MainWindow()
    window.show()

    app.exec_()