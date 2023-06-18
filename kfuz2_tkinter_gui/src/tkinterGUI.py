# tkinter powered app for kfuz2
# Author        : Shtoyan
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from enum import Enum
from multiprocessing import Pool, cpu_count
import os
import tkinter as tk
from tkinter import IntVar, ttk, filedialog
from typing import Any
from webbrowser import open_new
from subprocess import run
from pathlib import Path
from os import walk
from platform import uname
from time import time

# This gui is mainly built for KF1, so you might want to manually
# add file extensions of your UE2-based game
KF_EXTENSIONS: tuple[str, ...] = (".u", ".utx", ".usx", ".ukx", ".uax", ".rom", ".uz2")


class OPERATION_TYPE(Enum):
    Compression = 0
    Decompression = 1


class App(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        # what platform is this?
        self.current_system: str = uname().system
        self.kfuz2: str = self.executable_name()

        path_script: Path = Path(os.path.realpath(__file__))
        self.cli: Path = path_script.parent.joinpath(self.kfuz2)
        if not self.cli.exists():
            print(f"Can not find {self.cli=}")
            exit()
        self.verbose: bool = True
        self.quiet: bool = True
        self.File_List: list[str] = []
        self.Input: str = ""
        self.Output: str = ""

        self.geometry("750x190")
        self.title("Killing Floor 1 Redirect Tool")
        # Reference: https://coolors.co/palette/264653-2a9d8f-e9c46a-f4a261-e76f51
        self["background"] = "#264653"
        self.add_Menus()

        self.columnconfigure(0, weight=0)
        self.columnconfigure(1, weight=0)
        self.columnconfigure(2, weight=0)
        self.columnconfigure(3, weight=0)
        self.columnconfigure(4, weight=0)
        self.columnconfigure(5, weight=1)

        self.create_widgets()
        self.mainloop()

    def executable_name(self) -> str:
        match self.current_system:
            case "Windows":
                return "kfuz2.exe"
            case _:
                return "kfuz2"

    def get_args(
        self, type: OPERATION_TYPE = OPERATION_TYPE.Compression
    ) -> list[list[str]]:
        result: list[list[str]] = []
        self.refresh_file_list()

        for file in self.File_List:
            iter: list[Any] = []

            iter.insert(0, file)
            if type == OPERATION_TYPE.Decompression:
                iter.insert(0, "-d")
            # tmp.insert(0, "--nocheck")
            if self.verbose:
                iter.insert(0, "-v")
            if self.quiet:
                iter.insert(0, "-q")
            if self.Output != "":
                iter.insert(0, self.Output)
                iter.insert(0, "-o")

            iter.insert(0, self.cli)
            result.insert(0, iter)
        return result

    def create_widgets(self) -> None:
        lb_input = ttk.Label(self, text="Input: ...", width=80, background="lightgrey")
        lb_output = ttk.Label(
            self, text="Output: ...", width=80, background="lightgrey"
        )

        btn_select_input = ttk.Button(
            self,
            width=15,
            text="Select Input",
            command=lambda: self.select_input(lb_input, btn_Compress, btn_Uncompress),
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
            width=20,
            text="Compress",
            state="disabled",
            command=self.start_Compression,
        )
        btn_Uncompress = ttk.Button(
            self,
            width=20,
            text="Uncompress",
            state="disabled",
            command=self.start_Uncompression,
        )

        var = IntVar()
        var.set(0)
        rb_1 = ttk.Radiobutton(
            self,
            width=20,
            text="Default Log",
            variable=var,
            value=0,
            command=lambda: self.set_log_level(var),
        )
        rb_2 = ttk.Radiobutton(
            self,
            width=20,
            text="Verbose Log",
            variable=var,
            value=1,
            command=lambda: self.set_log_level(var),
        )
        rb_3 = ttk.Radiobutton(
            self,
            width=20,
            text="Disable Log",
            variable=var,
            value=2,
            command=lambda: self.set_log_level(var),
        )

        # Grid
        lb_input.grid(column=1, row=0, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        btn_select_input.grid(column=0, row=0, padx=5, pady=5)

        lb_output.grid(column=1, row=1, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        btn_select_output.grid(column=0, row=1, sticky=tk.S, padx=5, pady=5)

        btn_open_output.grid(column=0, row=2, sticky=tk.S, padx=5, pady=5)
        rb_1.grid(column=1, row=2, columnspan=1, sticky=tk.NW, padx=5, pady=5)
        rb_2.grid(column=2, row=2, columnspan=1, sticky=tk.NW, padx=5, pady=5)
        rb_3.grid(column=3, row=2, columnspan=1, sticky=tk.NW, padx=5, pady=5)

        btn_Compress.grid(column=1, row=5, columnspan=1, sticky=tk.NSEW, padx=5, pady=5)
        btn_Uncompress.grid(
            column=2, row=5, columnspan=1, sticky=tk.NSEW, padx=5, pady=5
        )

    def set_log_level(self, level: IntVar) -> None:
        num: int = level.get()
        match num:
            case 0:
                self.quiet = False
                self.verbose = False
            case 1:
                self.quiet = False
                self.verbose = True
            case _:
                self.quiet = True
                self.verbose = False

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
        print("=============== COMPRESSION START ===============")
        input_args: list[list[str]] = self.get_args()
        start: float = time()
        # print(f"!!! Cpu count is {cpu_count()}")
        with Pool(processes=cpu_count()) as pool:
            pool.map(ext_run, input_args)
        end: float = time()
        print(f"Exectution time {end - start}")
        print("=============== COMPRESSION END ===============")

    def start_Uncompression(self) -> None:
        print("=============== DECOMPRESSION START ===============")
        input_args: list[list[str]] = self.get_args(type=OPERATION_TYPE.Decompression)
        start: float = time()
        # print(f"!!! Cpu count is {cpu_count()}")
        with Pool(processes=cpu_count()) as pool:
            pool.map(ext_run, input_args)
        end: float = time()
        print(f"Exectution time {end - start}")
        print("=============== DECOMPRESSION END ===============")

    def refresh_file_list(self) -> None:
        self.File_List.clear()
        if not Path(self.Input).exists():
            print("This is not a valid path!")
            pass
        else:
            for root, _, files in walk(self.Input):
                for file in files:
                    if file.endswith(KF_EXTENSIONS):
                        self.File_List.append(str(Path(root) / file))

    def select_output(self, label: ttk.Label, button: ttk.Button) -> str:
        self.Output = filedialog.askdirectory(title="Select Output Folder")
        if self.Output != "":
            label.config(text=self.Output)
            label.config(background="#e9c46a")
            button.config(state="enabled")
        return self.Output

    def select_input(
        self,
        lb_input: ttk.Label,
        btn_compress: ttk.Button,
        btn_uncompress: ttk.Button,
    ) -> str:
        self.Input = filedialog.askdirectory(title="Select Input Folder")
        if self.Input != "":
            lb_input.config(text=self.Input)
            lb_input.config(background="#e9c46a")
            btn_compress.config(state="enabled")
            btn_uncompress.config(state="enabled")
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


def ext_run(args: list[Any]) -> None:
    run(args)


def main() -> None:
    App()


if __name__ == "__main__":
    main()
