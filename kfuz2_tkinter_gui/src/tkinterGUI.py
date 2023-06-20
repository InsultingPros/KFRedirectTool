# tkinter powered app for kfuz2
# Author        : Shtoyan
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from enum import IntEnum, StrEnum, auto
from multiprocessing import Pool, cpu_count
import os
import tkinter as tk
from tkinter import BooleanVar, StringVar, ttk, filedialog
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


class OPERATION_TYPE(IntEnum):
    Compression = auto()
    Decompression = auto()


class LOG_LEVEL(StrEnum):
    Default = "Log Level - Default"
    Verbose = "Log Level - Verbose"
    Silent = "Log Level - Silent"


class App(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.init_variables()

        self.geometry("750x150")
        self.title("Killing Floor 1 Redirect Tool")
        # Reference: https://coolors.co/palette/264653-2a9d8f-e9c46a-f4a261-e76f51
        self["background"] = "#264653"
        # Grid
        self.columnconfigure(0, weight=0)
        self.columnconfigure(1, weight=0)
        self.columnconfigure(2, weight=0)
        self.columnconfigure(3, weight=0)
        self.columnconfigure(4, weight=0)
        self.columnconfigure(5, weight=1)
        # Style
        # style = ttk.Style()
        # style.theme_use("clam")
        # style.configure("TButton", background="#E76F51")
        # style.configure("TMenubutton", background="#E76F51")
        # Widgets
        self.add_Menus()
        self.create_widgets()

    def init_variables(self) -> None:
        # what platform is this?
        self.current_system: str = uname().system
        if self.current_system == "Windows":
            self.kfuz2 = "kfuz2.exe"
        else:
            self.kfuz2 = "kfuz2"

        path_script: Path = Path(os.path.realpath(__file__))
        self.cli: Path = path_script.parent.joinpath(self.kfuz2)
        if not self.cli.exists():
            print(f"Can not find {self.cli=}")
            exit()
        self.verbose: bool = False
        self.quiet: bool = False
        self.disable_multi_threading: bool = False
        self.File_List: list[str] = []
        self.Input: str = ""
        self.Output: str = ""

    def add_Menus(self) -> None:
        menus = tk.Menu(self)

        menu_file = tk.Menu(menus, tearoff=0)
        menu_input = tk.Menu(menu_file, tearoff=0)
        menu_file.add_cascade(label="Recent Output", menu=menu_input)
        menu_output = tk.Menu(menu_file, tearoff=0)
        menu_file.add_cascade(label="Recent Output", menu=menu_output)
        menu_file.add_separator()
        menu_file.add_command(label="Exit", command=lambda: self.destroy())
        menus.add_cascade(label="File", menu=menu_file)

        menu_help = tk.Menu(menus, tearoff=0)
        menu_help.add_command(
            label="Github",
            command=lambda: open_new("https://github.com/InsultingPros/KFRedirectTool"),
        )
        menus.add_cascade(label="Help", menu=menu_help)

        self.config(menu=menus)

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
            command=lambda: self.process_files(),
        )
        btn_Uncompress = ttk.Button(
            self,
            width=20,
            text="Uncompress",
            state="disabled",
            command=lambda: self.process_files(OPERATION_TYPE.Decompression),
        )

        om_var = StringVar()
        options: list[LOG_LEVEL] = [
            LOG_LEVEL.Default,
            LOG_LEVEL.Verbose,
            LOG_LEVEL.Silent,
        ]
        om_log_level = ttk.OptionMenu(
            self,
            om_var,
            LOG_LEVEL.Default,
            *options,
            command=lambda _: self.set_log_level(om_var),
        )
        om_log_level.config(width=20)

        cb_var = BooleanVar()
        cb_var.set(False)
        cb_multi_thread = ttk.Checkbutton(
            self,
            width=20,
            text="Disable Multithreading",
            variable=cb_var,
            command=lambda: self.set_multi_threading(cb_var),
        )

        # Grid
        lb_input.grid(column=1, row=0, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        btn_select_input.grid(
            column=0, columnspan=1, row=0, sticky=tk.NSEW, padx=5, pady=5
        )

        lb_output.grid(column=1, row=1, columnspan=5, sticky=tk.EW, padx=5, pady=5)
        btn_select_output.grid(
            column=0, columnspan=1, row=1, sticky=tk.NSEW, padx=5, pady=5
        )

        btn_open_output.grid(
            column=0, row=2, columnspan=1, sticky=tk.NSEW, padx=5, pady=5
        )
        om_log_level.grid(column=1, row=2, columnspan=1, sticky=tk.EW, padx=5, pady=5)
        cb_multi_thread.grid(
            column=2, row=2, columnspan=1, sticky=tk.NSEW, padx=5, pady=5
        )

        btn_Compress.grid(column=1, row=5, columnspan=1, sticky=tk.NSEW, padx=5, pady=5)
        btn_Uncompress.grid(
            column=2, row=5, columnspan=1, sticky=tk.NSEW, padx=5, pady=5
        )

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

    def set_log_level(self, level: StringVar) -> None:
        cmp: str = level.get()
        match cmp:
            case LOG_LEVEL.Default:
                self.quiet = False
                self.verbose = False
            case LOG_LEVEL.Verbose:
                self.quiet = False
                self.verbose = True
            case _:
                self.quiet = True
                self.verbose = False

    def set_multi_threading(self, switch: BooleanVar) -> None:
        self.disable_multi_threading = switch.get()

    def process_files(self, type: OPERATION_TYPE = OPERATION_TYPE.Compression) -> None:
        if type == OPERATION_TYPE.Decompression:
            prefix: str = "DE"
        else:
            prefix: str = ""

        print(f"=============== {prefix}COMPRESSION START ===============")
        input_args: list[list[str]] = self.get_args(type)
        start: float = time()

        if self.disable_multi_threading:
            for arg in input_args:
                ext_run(arg)
        else:
            with Pool(processes=cpu_count()) as pool:
                pool.map(ext_run, input_args)
        end: float = time()
        print(f"Exectution time {end - start}")
        print(f"=============== {prefix}COMPRESSION END ===============")

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


# thread pool throws exception on pickle, have to extract this from class
def ext_run(args: list[Any]) -> None:
    run(args)


if __name__ == "__main__":
    tkintergui = App()
    tkintergui.mainloop()
