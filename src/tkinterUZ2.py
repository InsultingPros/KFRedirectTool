# # tkinter powered app for UZ2Tool
# # Author        : NikC-
# # Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# # License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from UZ2Tool import UZ2API
from VanillaPackages import unrealExtensions, disallowedPackages
import tkinter as tk
from tkinter import ttk, filedialog
from webbrowser import open_new
from subprocess import Popen
from pathlib import Path
from os import walk, path


class App(tk.Tk):
    def __init__(self):
        super().__init__()

        self.UZ2API = UZ2API()
        self.File_List: list[str] = []
        self.Input: str = ""
        self.Output: str = ""

        self.geometry("500x300")
        self.title("Killing Floor 1 Redirect Tool")
        self.add_Menus()

        self.columnconfigure(0, weight=1)
        self.columnconfigure(1, weight=3)

        self.create_widgets()
        self.mainloop()

    def create_widgets(self) -> None:
        lb_output = ttk.Label(self, text="Output:")
        lb_input = ttk.Label(self, text="Input:")

        btn_select_input = ttk.Button(
            self, text="Select Input", command=lambda: self.select_input(lb_input)
        )
        btn_select_output = ttk.Button(
            self, text="Select Output", command=lambda: self.select_output(lb_output)
        )
        btn_open_output = ttk.Button(self, text="Open Output", command=self.open_output)
        btn_Compress = ttk.Button(self, text="Compress", command=self.start_Compression)
        btn_Uncompress = ttk.Button(
            self, text="Uncompress", command=self.start_Uncompression
        )

        lb_output.grid(column=0, row=0, columnspan=4, sticky=tk.S, padx=5, pady=5)
        lb_input.grid(column=0, row=1, columnspan=4, sticky=tk.S, padx=5, pady=5)

        btn_select_input.grid(column=0, row=2, sticky=tk.S, padx=5, pady=5)
        btn_select_output.grid(column=1, row=2, sticky=tk.S, padx=5, pady=5)
        btn_open_output.grid(column=2, row=2, sticky=tk.S, padx=5, pady=5)
        btn_Compress.grid(column=3, row=2, sticky=tk.S, padx=5, pady=5)
        btn_Uncompress.grid(column=4, row=2, sticky=tk.S, padx=5, pady=5)

    def add_Menus(self) -> None:
        menu = tk.Menu(self)
        self.config(menu=menu)

        fileMenu = tk.Menu(menu, tearoff=0)
        fileMenu2 = tk.Menu(fileMenu, tearoff=0)
        fileMenu.add_cascade(label="Recent Output", menu=fileMenu2)
        fileMenu.add_command(label="Exit", command=self.exitProgram)
        menu.add_cascade(label="File", menu=fileMenu)

        menu_About = tk.Menu(menu, tearoff=0)
        menu_About.add_command(label="Github", command=self.open_Repo)
        menu.add_cascade(label="Info", menu=menu_About)

    def start_Compression(self) -> None:
        self.refresh_file_list()
        for x in self.File_List:
            self.UZ2API.compress(x, self.Output)
        print("=============== DONE ===============")

    def start_Uncompression(self) -> None:
        self.refresh_file_list()
        for x in self.File_List:
            self.UZ2API.uncompress(x, self.Output)
        print("=============== DONE ===============")

    def refresh_file_list(self) -> None:
        self.File_List.clear()
        if not Path(self.Input).exists:
            print("This is not a valid path!")
            pass
        else:
            for root, dirs, files in walk(self.Input):
                for file in files:
                    if (
                        file.endswith(unrealExtensions)
                        and file.lower() not in disallowedPackages
                    ):
                        self.File_List.append(str(Path(root) / file))

    def select_output(self, label: ttk.Label) -> str:
        self.Output = filedialog.askdirectory(title="Select Output Folder")
        label.config(text=self.Output)
        return self.Output

    def select_input(self, label: ttk.Label) -> str:
        self.Input = filedialog.askdirectory(title="Select Input Folder")
        label.config(text=self.Input)
        return self.Input

    def open_output(self) -> None:
        x = Path(self.Output)
        if x.exists():
            Popen(f"""explorer "{x}" """)

    def open_Repo(self) -> None:
        open_new("https://github.com/InsultingPros/KFRedirectTool")

    def exitProgram(self) -> None:
        exit()


if __name__ == "__main__":
    app = App()
