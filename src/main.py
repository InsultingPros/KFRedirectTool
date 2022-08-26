# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool

# various
import sys
# QT
from PyQt5.QtWidgets import QMainWindow, QApplication, QPushButton, QToolButton, QTableWidget, QMenu
from PyQt5 import uic
# packages
from VanillaPackages import disallowedPackages
from importlib.resources import files, as_file

source = files('resources').joinpath('KFTinyUZ2.exe')
with as_file(source) as eml:
    print('done!')

# print(disallowedPackages)

class MainWindow(QMainWindow):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        layout = files('resources').joinpath('layout.ui')
        uic.loadUi(layout, self)

        # output field
        # self.pushButton_5 = self.findChild(QPushButton, 'pushButton_5')
        self.pushButton_5.clicked.connect(lambda: self.select_Output())
        self.toolButton_3.clicked.connect(lambda: self.toolbtn_Output())

        # tabs
        self.tabWidget.tabBarClicked.connect(lambda: self.clckTab())

        # tab #1
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
    app = QApplication(sys.argv)

    window = MainWindow()
    window.show()

    app.exec_()