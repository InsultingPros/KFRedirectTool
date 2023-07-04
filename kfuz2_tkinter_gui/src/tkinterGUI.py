# tkinter powered app for kfuz2
# Author        : Shtoyan
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from enum import IntEnum, StrEnum, auto
from multiprocessing import Pool, cpu_count
import os
import tkinter as tk
import pickle
from tkinter import BooleanVar, StringVar, ttk, filedialog
from typing import Any, Final
from webbrowser import open_new
from subprocess import run
from pathlib import Path
from os import walk
from platform import uname
from time import time

PICKLE_NAME: Final[str] = "tkinterGUI"
# This gui is mainly built for KF1, so you might want to manually
# add file extensions of your UE2-based game
DEFAULT_KF_EXTENSIONS: Final[tuple[str, ...]] = (
    ".u",
    ".utx",
    ".usx",
    ".ukx",
    ".uax",
    ".rom",
    ".uz2",
)
DEFAULT_WIN_X: Final[int] = 750
DEFAULT_WIN_Y: Final[int] = 150
# Reference: https://coolors.co/palette/264653-2a9d8f-e9c46a-f4a261-e76f51
DEFAULT_LABEL_COLOR_EMPTY: Final[str] = "lightgrey"
DEFAULT_LABEL_COLOR_SELECTED: Final[str] = "#e9c46a"
DEFAULT_WINDOW_COLOR: Final[str] = "#264653"


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
        self.wm_protocol("WM_DELETE_WINDOW", lambda: self.on_close())
        self.init_variables()

        self.geometry(f"{self.win_x}x{self.win_y}")

        self.title("Killing Floor 1 Redirect Tool")
        self["background"] = DEFAULT_WINDOW_COLOR
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

        self.mainloop()

    def on_close(self) -> None:
        self.save_state()
        self.destroy()

    def save_state(self) -> None:
        my_pickle: Path = Path(os.path.realpath(__file__)).parent.joinpath(PICKLE_NAME)
        try:
            with open(my_pickle, "wb") as f:
                pickle.dump(
                    obj=[
                        self.winfo_width(),
                        self.winfo_height(),
                        self.Input,
                        self.Output,
                        self.disable_multi_threading,
                        self.log_level,
                        self.no_check,
                        self.tkvar_extensions.get(),
                    ],
                    file=f,
                )
        except Exception as e:
            print("Error appeared while trying to pickle stuff: " + str(e))

    def load_state(self) -> bool:
        my_pickle: Path = Path(os.path.realpath(__file__)).parent.joinpath(PICKLE_NAME)
        try:
            with open(my_pickle, "rb") as f:
                (
                    self.win_x,
                    self.win_y,
                    self.Input,
                    self.Output,
                    self.disable_multi_threading,
                    self.log_level,
                    self.no_check,
                    self.extensions,
                ) = pickle.load(f)
            return True
        except Exception as e:
            print("Error appeared while trying to pickle stuff: " + str(e))
            return False

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
            self.on_close()
        self.File_List: list[str] = []

        if not self.load_state():
            self.win_x: int = DEFAULT_WIN_X
            self.win_y: int = DEFAULT_WIN_Y
            self.Input: str = ""
            self.Output: str = ""
            self.disable_multi_threading: bool = False
            self.log_level = LOG_LEVEL.Default
            self.no_check: bool = False
            self.extensions: str = ",".join(DEFAULT_KF_EXTENSIONS)

        self.tkvar_extensions = StringVar(self, value=self.extensions)

    def add_Menus(self) -> None:
        menus = tk.Menu(self)

        menu_file = tk.Menu(menus, tearoff=0)
        menu_file.add_command(label="Exit", command=lambda: self.on_close())
        menus.add_cascade(label="File", menu=menu_file)

        menu_adv = tk.Menu(menus, tearoff=0)
        cbadv_var = BooleanVar(self)
        cbadv_var.set(self.no_check)
        menu_adv.add_checkbutton(
            label="Disable KF Checks",
            variable=cbadv_var,
            command=lambda: self.disable_kf_checks(cbadv_var),
        )
        menu_adv.add_command(
            label="Edit Extension List...", command=lambda: self.edit_extension_list()
        )
        menus.add_cascade(label="Advanced", menu=menu_adv)

        menu_help = tk.Menu(menus, tearoff=0)
        menu_help.add_command(
            label="Github",
            command=lambda: open_new("https://github.com/InsultingPros/KFRedirectTool"),
        )
        menus.add_cascade(label="Help", menu=menu_help)

        self.config(menu=menus)

    def create_widgets(self) -> None:
        lb_input = ttk.Label(
            self,
            text=self.Input if self.Input != "" else "Input: ...",
            width=80,
            background=DEFAULT_LABEL_COLOR_SELECTED
            if self.Input != ""
            else DEFAULT_LABEL_COLOR_EMPTY,
        )

        lb_output = ttk.Label(
            self,
            text=self.Output if self.Output != "" else "Output: ...",
            width=80,
            background=DEFAULT_LABEL_COLOR_SELECTED
            if self.Output != ""
            else DEFAULT_LABEL_COLOR_EMPTY,
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
            state="enabled" if self.Output != "" else "disabled",
            command=self.open_output,
        )
        btn_Compress = ttk.Button(
            self,
            width=20,
            text="Compress",
            state="enabled" if self.Input != "" else "disabled",
            command=lambda: self.process_files(),
        )
        btn_Uncompress = ttk.Button(
            self,
            width=20,
            text="Uncompress",
            state="enabled" if self.Input != "" else "disabled",
            command=lambda: self.process_files(OPERATION_TYPE.Decompression),
        )

        om_var = StringVar(self)
        om_log_level = ttk.OptionMenu(
            self,
            om_var,
            self.log_level,
            *[
                LOG_LEVEL.Default,
                LOG_LEVEL.Verbose,
                LOG_LEVEL.Silent,
            ],
            command=lambda _: self.set_log_level(om_var),
        )
        om_log_level.config(width=20)

        cb_var = BooleanVar(self)
        cb_var.set(self.disable_multi_threading)
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

    def disable_kf_checks(self, switch: BooleanVar) -> None:
        self.no_check = switch.get()

    def edit_extension_list(self) -> None:
        adv_win = tk.Toplevel(self)
        adv_win.geometry("600x40")
        adv_win.columnconfigure(0, weight=1)
        adv_win.columnconfigure(1, weight=0)
        adv_win.columnconfigure(2, weight=0)
        adv_win.columnconfigure(3, weight=0)

        self.temp_var = StringVar(adv_win, value=self.tkvar_extensions.get())
        entry_extensions = ttk.Entry(adv_win, width=120, textvariable=self.temp_var)
        btn_save = ttk.Button(
            adv_win,
            width=15,
            text="Save",
            state="enabled",
            command=lambda: self.save_entry(self.temp_var),
        )
        btn_reset = ttk.Button(
            adv_win,
            width=15,
            text="Reset",
            state="enabled",
            command=lambda: self.reset_extensions(self.temp_var),
        )

        entry_extensions.grid(
            column=0, row=0, columnspan=2, sticky=tk.NSEW, padx=5, pady=5
        )
        btn_save.grid(column=2, row=0, columnspan=1, sticky=tk.NSEW, padx=5, pady=5)
        btn_reset.grid(column=3, row=0, columnspan=1, sticky=tk.NSEW, padx=5, pady=5)

        adv_win.bind("<Escape>", lambda _: adv_win.destroy())
        adv_win.grab_set()

    def save_entry(self, entry_var: StringVar) -> None:
        self.extensions = entry_var.get()
        self.tkvar_extensions.set(self.extensions)

    def reset_extensions(self, entry_var: StringVar) -> None:
        self.extensions: str = ",".join(DEFAULT_KF_EXTENSIONS)
        self.tkvar_extensions.set(self.extensions)
        entry_var.set(self.extensions)

    def select_output(self, label: ttk.Label, button: ttk.Button) -> str:
        self.Output = filedialog.askdirectory(title="Select Output Folder")
        if self.Output != "":
            label.config(text=self.Output)
            label.config(background=DEFAULT_LABEL_COLOR_SELECTED)
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
            lb_input.config(background=DEFAULT_LABEL_COLOR_SELECTED)
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
                run(["xdg-open", path_output])

    def set_log_level(self, level: StringVar) -> None:
        self.log_level: str = level.get()

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
            if self.no_check:
                iter.insert(0, "--nocheck")
            if self.log_level == LOG_LEVEL.Verbose:
                iter.insert(0, "-v")
            elif self.log_level == LOG_LEVEL.Silent:
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
            ext_list: list[str] = self.tkvar_extensions.get().split(",", -1)
            ext_tup = tuple(ext_list)

            for root, _, files in walk(self.Input):
                for file in files:
                    if file.endswith(ext_tup):
                        self.File_List.append(str(Path(root) / file))


# thread pool throws exception on pickle, have to extract this from class
def ext_run(args: list[Any]) -> None:
    run(args)


if __name__ == "__main__":
    try:
        my_app = App()
    except Exception as e:
        print("Error appeared: " + str(e))
