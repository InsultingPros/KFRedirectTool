# Patcher for TinyUZ2.exe
# Allows to use it with KF1 packages
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from os import path
from pathlib import Path

# need to swap 'C1' to 'C2'
PATCHES = (
    # (offset, original, patched)
    (0x000010E2, b"\xC1", b"\xC2"),
    (0x000014BF, b"\xC1", b"\xC2"),
    (0x000016F5, b"\xC1", b"\xC2"),
    (0x00001749, b"\xC1", b"\xC2"),
)


def patchRevert():
    """Revert the patch."""
    with open(tinyuz2exe, "r+b") as file:
        for patch_tuple in PATCHES:
            offset, original, patched = patch_tuple
            file.seek(offset)
            patched_data = file.read(len(patched))
            if patched_data == patched:
                file.seek(offset)
                file.write(original)
                print("Patched: 0x{:x}: {} -> {}".format(offset, patched, original))


def patchApply():
    """Apply the patch."""
    with open(tinyuz2exe, "r+b") as file:
        for patch_tuple in PATCHES:
            offset, original, patched = patch_tuple
            file.seek(offset)
            patched_data = file.read(len(original))
            if patched_data == original:
                file.seek(offset)
                file.write(patched)
                print("Patched: 0x{:x}: {} -> {}".format(offset, original, patched))


def int_input(text):
    """Makes sure that that user input is an int."""
    while True:
        try:
            num = int(input(text))
        except ValueError:
            print("You must enter an integer.")
        else:
            return num


##########################################################################

print("\nWelcome to TinyUZ2.exe Patcher!\n")

tinyuz2exe = Path(path.dirname(path.realpath(__file__)) + "/tinyuz2.exe")
print(tinyuz2exe)
if not Path(tinyuz2exe).exists():
    print(
        "tinyuz2.exe not found in directory! Make sure you run this script in KF1 directory!\n"
    )
    input("Press any key to continue.")

print(" - Type 1 to apply the patch.")
print(" - Type 2 to revert the patch.")
print(" - Type 3 to exit.\n")

while True:
    match int_input("Enter your value: "):
        case 1:
            patchApply()
        case 2:
            patchRevert()
        case 3:
            print("Exiting...")
            exit()
        case _:
            print("Wrong number! Try again.")
