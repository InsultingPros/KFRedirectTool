# tkinter powered app for kfuz2
# Author        : Shtoyan
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from concurrent.futures import ProcessPoolExecutor
from enum import IntEnum, StrEnum, auto
from functools import partial
from multiprocessing import Manager, cpu_count
from multiprocessing.managers import SyncManager
from pathlib import Path
from pickle import dump, load
from platform import uname
from subprocess import run
from threading import Event, Thread
from time import time
from tkinter import (
    CENTER,
    DISABLED,
    HORIZONTAL,
    NORMAL,
    NSEW,
    BooleanVar,
    Menu,
    StringVar,
    Tk,
    Toplevel,
    filedialog,
    messagebox,
)
from tkinter.ttk import Button, Checkbutton, Entry, Label, OptionMenu, Progressbar
from typing import Any, Final
from webbrowser import open_new

PICKLE_NAME: Final[str] = "tkinterGUI"
"""Settings file name."""
DEFAULT_EXTENSIONS: Final[tuple[str, ...]] = (
    ".u",
    ".utx",
    ".usx",
    ".ukx",
    ".uax",
    ".rom",
    ".uz2",
)
"""Default file extension list.

This gui is mainly built for KF1, so you might want to manually add file extensions of your UE2-based game."""
DEFAULT_WIN_X: Final[int] = 750
DEFAULT_WIN_Y: Final[int] = 150
# Reference: https://coolors.co/palette/264653-2a9d8f-e9c46a-f4a261-e76f51
DEFAULT_LABEL_COLOR_EMPTY: Final[str] = "lightgrey"
DEFAULT_LABEL_COLOR_SELECTED: Final[str] = "#e9c46a"
DEFAULT_WINDOW_COLOR: Final[str] = "#264653"


class OperationType(IntEnum):
    Compression = auto()
    Decompression = auto()


class Log(StrEnum):
    Default = "Log Level - Default"
    Verbose = "Log Level - Verbose"
    Silent = "Log Level - Silent"


class App(Tk):
    """`tkinterGUI`'s Main window."""

    # We can easily skip this optimization, but why not?
    __slots__: tuple[str, ...] = (
        "manager",
        "stop_event",
        "win_x",
        "win_y",
        "kfuz2",
        "cli",
        "input",
        "output",
        "disable_multi_threading",
        "log_level",
        "no_check",
        "extensions",
        "file_list",
        "tkvar_extensions",
    )

    def __init__(self) -> None:
        super().__init__()
        self.manager: SyncManager = Manager()
        """Main app's `SyncManager`."""
        self.stop_event: Event = self.manager.Event()
        """SyncManager `Event` to stop file processing at any given moment."""
        self.win_x: int = DEFAULT_WIN_X
        """`X` dimension of main window."""
        self.win_y: int = DEFAULT_WIN_Y
        """`Y` dimension of main window."""
        self.kfuz2: str = ""
        """UZ2 compressor's executable name."""
        if uname().system == "Windows":
            self.kfuz2 = "kfuz2.exe"
        else:
            self.kfuz2 = "kfuz2"
        self.cli: Path = Path(__file__).parent.joinpath(self.kfuz2)
        """Path to UZ2 compressor."""
        if not self.cli.exists():
            print(f"Can not find {self.cli=}")
            self.on_close()
        self.input: str = ""
        """`Input` directory for processed files."""
        self.output: str = ""
        """`Output` directory for processed files."""
        self.disable_multi_threading: bool = False
        """Multi-threading switch, useful for hdd users."""
        self.log_level = Log.Default
        """UZ2 compressor's log levels."""
        self.no_check: bool = False
        """UZ2 compressor's `nocheck` argument. Switch for KF1's default file checks."""
        self.extensions: str = ",".join(DEFAULT_EXTENSIONS)
        """Extension list to filter input files."""
        self.file_list: list[list[Any]] = []
        """Input files with all UZ2 executable arguments."""

        # init everything
        self.load_state()
        self.tkvar_extensions = StringVar(self, value=self.extensions)
        """Tkinter variable to control extension list."""
        self.wm_protocol("WM_DELETE_WINDOW", self.on_close)

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
        self.add_menus()
        self.create_widgets()

        self.mainloop()

    def on_close(self) -> None:
        """On app close save variables to settings file."""
        self.save_state()
        self.destroy()

    def save_state(self) -> None:
        """Save app variables to settings file."""
        my_pickle: Path = Path(__file__).parent.joinpath(PICKLE_NAME)
        try:
            with open(my_pickle, "wb") as f:
                dump(
                    obj=[
                        self.winfo_width(),
                        self.winfo_height(),
                        self.input,
                        self.output,
                        self.disable_multi_threading,
                        self.log_level,
                        self.no_check,
                        self.tkvar_extensions.get(),
                    ],
                    file=f,
                )
        except Exception as err:
            print("Error appeared while trying to pickle stuff: " + str(err))

    def load_state(self) -> bool:
        """Load app variables from settings file."""
        my_pickle: Path = Path(__file__).parent.joinpath(PICKLE_NAME)
        if not my_pickle.exists():
            print(
                f"Config '{PICKLE_NAME}' was not found! Restart the app to generate it."
            )
            return False
        try:
            with open(my_pickle, "rb") as f:
                (
                    self.win_x,
                    self.win_y,
                    self.input,
                    self.output,
                    self.disable_multi_threading,
                    self.log_level,
                    self.no_check,
                    self.extensions,
                ) = load(f)
            return True
        except Exception as err:
            print("Error appeared while trying to pickle stuff: " + str(err))
            return False

    def add_menus(self) -> None:
        """Add Menu Bar items."""
        menus = Menu(self)

        menu_file = Menu(menus, tearoff=0)
        menu_file.add_command(label="Exit", command=self.on_close)
        menus.add_cascade(label="File", menu=menu_file)

        menu_adv = Menu(menus, tearoff=0)
        cbadv_var = BooleanVar(self)
        cbadv_var.set(self.no_check)
        menu_adv.add_checkbutton(
            label="Disable KF Checks",
            variable=cbadv_var,
            command=partial(self.disable_kf_checks, cbadv_var),
        )
        menu_adv.add_command(
            label="Edit Extension List...", command=partial(EditExtensionsTL, self)
        )
        menus.add_cascade(label="Advanced", menu=menu_adv)

        menu_help = Menu(menus, tearoff=0)
        menu_help.add_command(
            label="Github",
            command=partial(
                open_new, "https://github.com/InsultingPros/KFRedirectTool"
            ),
        )
        menus.add_cascade(label="Help", menu=menu_help)

        self.config(menu=menus)

    def create_widgets(self) -> None:
        """Create all other tkinter widgets."""
        lb_input = Label(
            self,
            text=self.input if self.input != "" else "Input: ...",
            width=80,
            background=DEFAULT_LABEL_COLOR_SELECTED
            if self.input != ""
            else DEFAULT_LABEL_COLOR_EMPTY,
        )
        lb_output = Label(
            self,
            text=self.output if self.output != "" else "Output: ...",
            width=80,
            background=DEFAULT_LABEL_COLOR_SELECTED
            if self.output != ""
            else DEFAULT_LABEL_COLOR_EMPTY,
        )
        btn_open_output = Button(
            self,
            width=15,
            text="Open Output",
            state=NORMAL if self.output != "" else DISABLED,
            command=self.open_output,
        )
        btn_select_output = Button(
            self,
            width=15,
            text="Select Output",
            command=partial(self.select_output, lb_output, btn_open_output),
        )
        btn_compress = Button(
            self,
            width=20,
            text="Compress",
            state=NORMAL if self.input != "" else DISABLED,
            command=self.start_processing_thread,
        )
        btn_uncompress = Button(
            self,
            width=20,
            text="Uncompress",
            state=NORMAL if self.input != "" else DISABLED,
            command=partial(self.start_processing_thread, OperationType.Decompression),
        )
        btn_select_input = Button(
            self,
            width=15,
            text="Select Input",
            command=partial(self.select_input, lb_input, btn_compress, btn_uncompress),
        )

        om_var = StringVar(self)
        om_log_level = OptionMenu(
            self,
            om_var,
            self.log_level,
            *[
                Log.Default,
                Log.Verbose,
                Log.Silent,
            ],
            command=lambda _: self.set_log_level(om_var),
        )
        om_log_level.config(width=20)

        cb_var = BooleanVar(self)
        cb_var.set(self.disable_multi_threading)
        cb_multi_thread = Checkbutton(
            self,
            width=20,
            text="Disable Multithreading",
            variable=cb_var,
            command=partial(self.set_multi_threading, cb_var),
        )

        # Grid
        lb_input.grid(column=1, row=0, columnspan=5, sticky=NSEW, padx=5, pady=5)
        btn_select_input.grid(
            column=0, columnspan=1, row=0, sticky=NSEW, padx=5, pady=5
        )
        lb_output.grid(column=1, row=1, columnspan=5, sticky=NSEW, padx=5, pady=5)
        btn_select_output.grid(
            column=0, columnspan=1, row=1, sticky=NSEW, padx=5, pady=5
        )

        btn_open_output.grid(column=0, row=2, columnspan=1, sticky=NSEW, padx=5, pady=5)
        om_log_level.grid(column=1, row=2, columnspan=1, sticky=NSEW, padx=5, pady=5)
        cb_multi_thread.grid(column=2, row=2, columnspan=1, sticky=NSEW, padx=5, pady=5)

        btn_compress.grid(column=1, row=5, columnspan=1, sticky=NSEW, padx=5, pady=5)
        btn_uncompress.grid(column=2, row=5, columnspan=1, sticky=NSEW, padx=5, pady=5)

    def disable_kf_checks(self, switch: BooleanVar) -> None:
        """Switch KF file check variable."""
        self.no_check = switch.get()

    def select_output(self, label: Label, button: Button) -> str:
        """Select `Output` directory from file dialog."""
        self.output = filedialog.askdirectory(title="Select Output Folder")
        if self.output != "":
            label.config(text=self.output)
            label.config(background=DEFAULT_LABEL_COLOR_SELECTED)
            button.config(state=NORMAL)
        return self.output

    def select_input(
        self,
        lb_input: Label,
        btn_compress: Button,
        btn_uncompress: Button,
    ) -> str:
        """Select `Input` directory from file dialog."""
        self.input = filedialog.askdirectory(title="Select Input Folder")
        if self.input != "":
            lb_input.config(text=self.input)
            lb_input.config(background=DEFAULT_LABEL_COLOR_SELECTED)
            btn_compress.config(state=NORMAL)
            btn_uncompress.config(state=NORMAL)
        return self.input

    def open_output(self) -> None:
        """Open `Output` directory in file explorer."""
        path_output = Path(self.output)
        if not path_output.exists():
            print(f"Can not find {path_output=}!")

        match uname().system:
            case "Darwin":
                run(["open", "--", path_output])
            case "Windows":
                run(["explorer", path_output])
            case _:
                run(["xdg-open", path_output])

    def set_log_level(self, level: StringVar) -> None:
        """Set UZ2 compressor's log level."""
        self.log_level = Log(level.get())

    def set_multi_threading(self, switch: BooleanVar) -> None:
        """Switch multi-threading variable."""
        self.disable_multi_threading = switch.get()

    def start_processing_thread(
        self, op_type: OperationType = OperationType.Compression
    ) -> None:
        """Run file processing in a separate thread and draw a basic progress bar."""
        self.get_file_list(op_type)
        ProgressBarTL(self)
        Thread(target=self.process_files, args=[op_type], daemon=True).start()

    def process_files(self, op_type: OperationType = OperationType.Compression) -> None:
        """(De) compressing of input files."""
        prefix: str = ""
        if op_type == OperationType.Decompression:
            prefix = "DE"
        if self.file_list:
            print(f"=============== {prefix}COMPRESSION START ===============")
            # reset event
            self.stop_event.clear()
            partial_run = partial(ext_run, event=self.stop_event)
            # now open the progress bar
            start: float = time()

            if self.disable_multi_threading:
                for arg in self.file_list:
                    partial_run(arg)
            else:
                with ProcessPoolExecutor(max_workers=cpu_count()) as executor:
                    executor.map(partial_run, self.file_list)
                # with Pool(processes=cpu_count()) as pool:
                #     pool.map(partial(ext_run, event=self.stop_event), input_args)
            end: float = time()
            print(f"Execution time {end - start}")
            print(f"=============== {prefix}COMPRESSION END ===============")
            self.file_list.clear()
        else:
            print("No files to process!")

    def get_file_list(self, op_type: OperationType = OperationType.Compression) -> None:
        """Add UZ2 arguments to input files and generate the final list."""
        raw_files: list[str] = self.get_raw_files()

        for file in raw_files:
            entry: list[Any] = []

            entry.insert(0, file)
            if op_type == OperationType.Decompression:
                entry.insert(0, "-d")
            if self.no_check:
                entry.insert(0, "--nocheck")
            if self.log_level == Log.Verbose:
                entry.insert(0, "-v")
            elif self.log_level == Log.Silent:
                entry.insert(0, "-q")

            if self.output != "":
                entry.insert(0, self.output)
                entry.insert(0, "-o")

            entry.insert(0, self.cli)
            self.file_list.insert(0, entry)

    def get_raw_files(self) -> list[str]:
        """Collect file list from `Input` directory."""
        result: list[str] = []

        if not Path(self.input).exists():
            print("This is not a valid path!")
        else:
            ext_list: list[str] = self.tkvar_extensions.get().split(",", -1)
            path_input: Path = Path(self.input)

            for content in path_input.rglob("*"):
                if not content.is_file():
                    continue
                if content.suffix in ext_list:
                    result.append(str(content))

        return result


class EditExtensionsTL(Toplevel):
    """Toplevel window for file extension editing."""

    __slots__: tuple[str, ...] = ("parent", "temp_var")

    def __init__(self, parent: App) -> None:
        super().__init__(parent)
        self.geometry("750x40")
        self.columnconfigure(0, weight=1)
        self.columnconfigure(1, weight=0)
        self.columnconfigure(2, weight=0)
        self.columnconfigure(3, weight=0)
        # variables
        self.parent: App = parent
        """Main app's ref."""
        self.temp_var = StringVar(self, value=self.parent.tkvar_extensions.get())
        """Temporary variable to set main app's extension list."""
        self.create_widgets()

        self.bind("<Escape>", lambda _: self.destroy())
        self.grab_set()
        self.focus_set()

    def create_widgets(self) -> None:
        entry_extensions = Entry(self, width=120, textvariable=self.temp_var)
        btn_save = Button(
            self,
            width=15,
            text="Save",
            state=NORMAL,
            command=partial(self.save_entry, self.temp_var),
        )
        btn_reset = Button(
            self,
            width=15,
            text="Reset",
            state=NORMAL,
            command=partial(self.reset_extensions, self.temp_var),
        )

        entry_extensions.grid(
            column=0, row=0, columnspan=2, sticky=NSEW, padx=5, pady=5
        )
        btn_save.grid(column=2, row=0, columnspan=1, sticky=NSEW, padx=5, pady=5)
        btn_reset.grid(column=3, row=0, columnspan=1, sticky=NSEW, padx=5, pady=5)

    def save_entry(self, entry_var: StringVar) -> None:
        """Save and apply file extension list."""
        self.parent.extensions = entry_var.get()
        self.parent.tkvar_extensions.set(self.parent.extensions)

    def reset_extensions(self, entry_var: StringVar) -> None:
        """Reset file extension list."""
        self.parent.extensions = ",".join(DEFAULT_EXTENSIONS)
        self.parent.tkvar_extensions.set(self.parent.extensions)
        entry_var.set(self.parent.extensions)


class ProgressBarTL(Toplevel):
    """Toplevel window for file processing progress bar."""

    __slots__: tuple[str, ...] = ("parent",)

    def __init__(self, parent: App) -> None:
        super().__init__(parent)
        self.geometry("400x100")
        self.columnconfigure(0, weight=0)
        self.columnconfigure(1, weight=0)
        self.columnconfigure(2, weight=1)

        self.parent: App = parent
        """Main app's ref."""
        self.create_widgets()
        self.grab_set()
        self.focus_set()
        self.wm_protocol("WM_DELETE_WINDOW", self.on_close)
        self.display_pbar()

    def on_close(self) -> None:
        """On Toplevel close stop file processing."""
        self.parent.stop_event.set()
        self.destroy()

    def monitor(self, to_check: Thread) -> None:
        """Monitor file processing and close self at the end."""
        if self.parent.file_list:
            self.after(100, partial(self.monitor, to_check))
        else:
            to_check.join()
            self.on_close()

    def create_widgets(self) -> None:
        pb = Progressbar(
            self,
            orient=HORIZONTAL,
            mode="indeterminate",
            length=15,
            value=0,
        )
        pb.start()
        lb_status = Label(
            self,
            anchor=CENTER,
            text=f"{len(self.parent.file_list)} files to process.",
            width=15,
            background=DEFAULT_LABEL_COLOR_SELECTED,
        )
        btn_cancel = Button(
            self,
            width=15,
            text="Cancel",
            state=NORMAL,
            command=self.on_close,
        )

        pb.grid(column=2, row=0, columnspan=3, sticky=NSEW, padx=5, pady=5)
        lb_status.grid(column=2, row=1, columnspan=3, sticky=NSEW, padx=5, pady=5)
        btn_cancel.grid(column=2, row=2, padx=5, pady=5)

    def display_pbar(self) -> None:
        if self.parent.file_list:
            t = Thread(daemon=True)
            t.start()
            self.after(50, partial(self.monitor, t))
        else:
            self.on_close()
            messagebox.showwarning(
                title="No files to process!",
                message=f'There are no matching files in "{self.parent.input}". Check your extension list / select proper directory.',
            )


def ext_run(args: list[Any], event: Event) -> None:
    """Execute UZ2 with passed arguments."""
    if event.is_set():
        return
    run(args)


if __name__ == "__main__":
    try:
        my_app = App()
    except KeyboardInterrupt:
        print("Terminated by Ctrl - C")
    except Exception as e:
        print("Error appeared: " + str(e))
