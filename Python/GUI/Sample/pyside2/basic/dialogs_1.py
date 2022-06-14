import sys

from PySide2.QtWidgets import QApplication, QDialog, QMainWindow, QPushButton


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("My App")

        button = QPushButton("Press me for a dialog!")
        button.clicked.connect(self.button_clicked)
        self.setCentralWidget(button)

    # end::__init__[]
    # def __init__ etc.
    # ... not shown for clarity

    def button_clicked(self, s):
        print("click", s)

        dlg = QDialog(self)
        dlg.setWindowTitle("HELLO!")
        dlg.exec_()


# end::MainWindow[]


app = QApplication(sys.argv)

window = MainWindow()
window.show()

app.exec_()