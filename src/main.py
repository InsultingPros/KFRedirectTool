from PyQt5 import QtCore, QtGui, QtWidgets
from VanillaPackages import vanillaSystem
from importlib.resources import files, as_file
from uifile import Ui_MainWindow

source = files('resources').joinpath('KFTinyUZ2.exe')
with as_file(source) as eml:
    print('done!')

print(vanillaSystem)

class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super(MainWindow, self).__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)

        # output field
        self.ui.pushButton_5.clicked.connect(lambda: self.select_Output())
        self.ui.toolButton_3.clicked.connect(lambda: self.toolbtn_Output())

        # tabs
        self.ui.tabWidget.tabBarClicked.connect(lambda: self.clckTab())

        # tab #1
        self.ui.pushButton.clicked.connect(lambda: self.compress())
        self.ui.pushButton_2.clicked.connect(lambda: self.deCompress())
        self.ui.toolButton.clicked.connect(lambda: self.toolbtn_Output())

        # tab #2
        self.ui.toolButton_5.clicked.connect(lambda: self.toolbtn_Output())
        self.ui.pushButton_9.clicked.connect(lambda: self.select_GameFolder())
        self.ui.pushButton_3.clicked.connect(lambda: self.open_Output())
        self.ui.pushButton_4.clicked.connect(lambda: self.compressAll())

        # menu bar
        self.ui.menuHelp.triggered.connect(lambda: self.clckHelp())
        self.ui.actionQuit.triggered.connect(self.close)

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
    import sys
    app = QtWidgets.QApplication(sys.argv)

    window = MainWindow()
    window.show()

    sys.exit(app.exec())