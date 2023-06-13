# tkinter powered app for kfuz2
# Author        : Shtoyan
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

import os
import tkinter as tk
from tkinter import IntVar, ttk
from tkinter import filedialog
from typing import Any
from webbrowser import open_new
from subprocess import run
from pathlib import Path
from os import walk
from platform import uname

unrealExtensions: tuple[str, ...] = (".u", ".utx", ".usx", ".ukx", ".uax", ".rom")


class App(tk.Tk):
    def __init__(self):
        super().__init__()
        # what platform is this?
        self.current_system: str = uname().system
        self.kfuz2: str = self.executable_name()

        path_script: Path = Path(os.path.realpath(__file__))
        self.cli: Path = path_script.parent.joinpath(self.kfuz2)
        if not self.cli.exists():
            print(f"Can not find {self.cli=}")
            exit()
        self.Disable_Checks: bool = True
        self.File_List: list[str] = []
        self.Input: str = ""
        self.Output: str = ""

        self.geometry("700x250")
        self.title("Killing Floor 1 Redirect Tool")
        self.add_Menus()

        self.columnconfigure(0, weight=0)
        self.columnconfigure(1, weight=0)
        self.columnconfigure(2, weight=0)
        self.columnconfigure(3, weight=0)
        self.columnconfigure(4, weight=1)
        self.columnconfigure(5, weight=1)

        self.create_widgets()
        self.mainloop()

    def executable_name(self) -> str:
        match self.current_system:
            case "Windows":
                return "kfuz2.exe"
            case _:
                return "kfuz2"

    def run_cli(self, args: list[Any]) -> None:
        if self.Disable_Checks:
            args.insert(0, "--nocheck")

        if self.Output != "":
            args.insert(0, self.Output)
            args.insert(0, "-o")

        args.insert(0, self.cli)

        try:
            run(args, check=True)
        except Exception as e:
            print(str(e))

    def create_widgets(self) -> None:
        lb_input = ttk.Label(self, text="Input: ...", width=80, background="lightgrey")
        lb_output = ttk.Label(
            self, text="Output: ...", width=80, background="lightgrey"
        )

        btn_select_input = ttk.Button(
            self,
            width=15,
            text="Select Input",
            command=lambda: self.select_input(
                lb_input, btn_Compress, btn_Uncompress, cb_Disable_Checks
            ),
        )
        btn_select_output = ttk.Button(
            self,
            width=15,
            text="Select Output",
            command=lambda: self.select_output(lb_output, btn_open_output),
        )
        btn_open_output = ttk.Button(
            self,
            width=15,
            text="Open Output",
            state="disabled",
            command=self.open_output,
        )
        btn_Compress = ttk.Button(
            self,
            width=30,
            text="Compress",
            state="disabled",
            command=self.start_Compression,
        )
        btn_Uncompress = ttk.Button(
            self,
            width=30,
            text="Uncompress",
            state="disabled",
            command=self.start_Uncompression,
        )
        # a little hack to uncheck this checkbox by default
        cb_var = IntVar()
        cb_var.set(1)
        cb_Disable_Checks = ttk.Checkbutton(
            self,
            text="Disable KF specific checks.",
            variable=cb_var,
            onvalue=1,
            offvalue=0,
            state="disabled",
            command=self.set_check_status,
        )

        # Grid
        # Labels
        lb_input.grid(column=1, row=0, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        lb_output.grid(column=1, row=1, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        # Buttons
        btn_select_input.grid(column=0, row=0, padx=5, pady=5)
        btn_select_output.grid(column=0, row=1, sticky=tk.S, padx=5, pady=5)
        btn_open_output.grid(column=0, row=3, sticky=tk.S, padx=5, pady=5)
        btn_Compress.grid(column=2, row=5, sticky=tk.S, padx=5, pady=5)
        btn_Uncompress.grid(column=3, row=5, sticky=tk.S, padx=5, pady=5)
        # Check box
        cb_Disable_Checks.grid(
            column=0, row=5, columnspan=2, sticky=tk.NS, padx=5, pady=5
        )

    def set_check_status(self) -> None:
        self.Disable_Checks = not self.Disable_Checks

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

        for file in self.File_List:
            self.run_cli([file])
        print("=============== DONE ===============")

    def start_Uncompression(self) -> None:
        self.refresh_file_list()
        for file in self.File_List:
            self.run_cli(["-d", file])
        print("=============== DONE ===============")

    def refresh_file_list(self) -> None:
        self.File_List.clear()
        if not Path(self.Input).exists:
            print("This is not a valid path!")
            pass
        else:
            for root, _, files in walk(self.Input):
                for file in files:
                    if not self.Disable_Checks:
                        if file.endswith(unrealExtensions):
                            self.File_List.append(str(Path(root) / file))
                    else:
                        self.File_List.append(str(Path(root) / file))

    def select_output(self, label: ttk.Label, button: ttk.Button) -> str:
        self.Output = filedialog.askdirectory(title="Select Output Folder")
        if self.Output != "":
            label.config(text=self.Output)
            label.config(background="lightgreen")
            button.config(state="enabled")
        return self.Output

    def select_input(
        self,
        label: ttk.Label,
        btn_compress: ttk.Button,
        btn_uncompress: ttk.Button,
        cb_nochecks: ttk.Checkbutton,
    ) -> str:
        self.Input = filedialog.askdirectory(title="Select Input Folder")
        if self.Input != "":
            label.config(text=self.Input)
            label.config(background="lightgreen")
            btn_compress.config(state="enabled")
            btn_uncompress.config(state="enabled")
            cb_nochecks.config(state="enabled")
            cb_nochecks.invoke()
        return self.Input

    def open_output(self) -> None:
        path_output = Path(self.Output)
        if not path_output.exists():
            print(f"Can not find {path_output=}!")

        match self.current_system:
            case "Darwin":
                run(["open", "--", path_output])
            case "Windows":
                run(["explorer", path_output])
            case _:
                run(["xdg-open", "--", path_output])

    def open_Repo(self) -> None:
        open_new("https://github.com/InsultingPros/KFRedirectTool")

    def exitProgram(self) -> None:
        exit()


def main() -> None:
    App()


if __name__ == "__main__":
    main()
