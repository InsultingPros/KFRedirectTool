# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from enum import Enum
from hashlib import sha1
import sys
from zlib import compress, decompress, error
from pathlib import Path
from os import path
from sys import argv
import argparse
import logging

class STATE(Enum):
    COMPRESSION = 0
    DECOMPRESSION = 1

# reference: https://github.com/Vel-San/gitlab-migrator/blob/main/scripts/migrate.py#L20-L24
__appname__ = path.splitext(path.basename(argv[0]))[0]
LOG = logging.getLogger(__appname__)
BASIC_FORMAT = "[%(levelname)s]:[%(filename)s:%(lineno)s - %(funcName)s()] %(message)s"
logging.basicConfig(format=BASIC_FORMAT)
LOG.setLevel(logging.DEBUG)

KFFILETAG: bytes = b"\xC2" + b"\x83" + b"\x2A" + b"\x9E"
# UNREALFILETAG: bytes = b'\xC1' + b'\x83' + b'\x2A' + b'\x9E'
CHUNKSIZE_COMPRESSED: int = 33096
CHUNKSIZE_UNCOMPRESSED: int = 32768


class UZ2API:
    """Shiny shiny tool!"""

    def __init__(self) -> None:
        self.filesize_original: int
        self.filesize_modified: int
        self.chunks: int

    # reference : https://wiki.beyondunreal.com/UZ2_file#File_format
    def compress(self, inputfile: str, outputDir: str = "") -> bool:
        """Compress passed Unreal Packages"""

        if not Path(inputfile).exists():
            LOG.error(r"Package doesn't exist!")
            return False

        if inputfile.endswith(".uz2"):
            LOG.error("Package is already compressed!")
            return False

        chunk_list: list[bytes] = []
        sha2, sha3 = sha1(), sha1()
        self.chunks = 0

        # get uncompressed chunks
        with open(inputfile, "rb") as f:
            # read first 4 bytes and check the file type
            file_tag = f.read(4)
            if KFFILETAG not in file_tag:
                LOG.error("This is not an Unreal Package!")
                return False

            # get the original file size
            f.seek(0, 2)
            self.filesize_original = f.tell()
            f.seek(0)

            # start to fill the chunk list
            while byte := f.read(CHUNKSIZE_UNCOMPRESSED):
                sha2.update(byte)
                chunk_list.append(byte)
                self.chunks += 1
            LOG.info(f"inputFile hash is: {sha2.hexdigest()}")

        outputfile = self.get_output_dest(inputfile, STATE.COMPRESSION, outputDir)
        with open(outputfile, "wb") as y:
            for z in chunk_list:
                # compress the chunk
                try:
                    res = compress(z, 7)
                except (error):
                    LOG.error("Something went wrong while trying to compress the data!")
                    return False
                # get compressed data length in 4 bytes
                resl = len(res).to_bytes(4, byteorder="little")
                # get chunks length in 4 bytes
                reschunk = len(z).to_bytes(4, byteorder="little")
                # write everything in this exact order
                y.write(resl)
                sha3.update(resl)
                y.write(reschunk)
                sha3.update(reschunk)
                y.write(res)
                sha3.update(res)
            self.filesize_modified = y.tell()
            LOG.info(f"outputFile hash is: {sha3.hexdigest()}")

        return True

    def uncompress(self, inputfile: str, outputDir: str = "") -> bool:

        if not Path(inputfile).exists():
            LOG.error(r"Package doesn't exist!")
            return False

        if not inputfile.endswith(".uz2"):
            LOG.error("This is not a compressed file!")
            return False

        chunk_list: list[bytes] = []
        sha4, sha5 = sha1(), sha1()

        bCorrectFile: bool = False
        with open(inputfile, "rb") as f:
            while cmprChunkSizeB := f.read(4):
                sha4.update(cmprChunkSizeB)
                cmprChunkSize: int = int.from_bytes(cmprChunkSizeB, byteorder="little")

                uncmprChunkSizeB: bytes = f.read(4)
                sha4.update(uncmprChunkSizeB)
                uncmprChunkSize: int = int.from_bytes(
                    uncmprChunkSizeB, byteorder="little"
                )

                data = f.read(cmprChunkSize)
                sha4.update(data)
                try:
                    yyy = decompress(data, bufsize=uncmprChunkSize)
                except (error):
                    LOG.error("Compressed file is invalid.")
                    return False
                if not bCorrectFile:
                    if KFFILETAG in yyy[:4]:
                        bCorrectFile = True
                    else:
                        LOG.error("This is not an Unreal Package!")
                        return False
                chunk_list.append(yyy)
            self.filesize_original = f.tell()
            LOG.info(f"inputFile hash is: {sha4.hexdigest()}")

        outputfile = self.get_output_dest(inputfile, STATE.DECOMPRESSION, outputDir)
        with open(outputfile, "wb") as f:
            for z in chunk_list:
                f.write(z)
                sha5.update(z)
            self.filesize_modified = f.tell()
            LOG.info(f"outputFile hash is: {sha5.hexdigest()}")

        return True

    def get_output_dest(
        self, inptDir: str, state: STATE, otptDir: str = ""
    ) -> str:
        match state:
            # compression
            case STATE.COMPRESSION:
                if not otptDir:
                    x = Path(inptDir).with_suffix(Path(inptDir).suffix + ".uz2")
                    return str(x)

                ext = Path(inptDir).suffix
                name = Path(inptDir).stem
                potptDir = Path(otptDir)

                if not potptDir.exists():
                    potptDir.mkdir()

                y = potptDir / name
                return str(y.with_suffix(ext + ".uz2"))

            # uncompression
            case STATE.DECOMPRESSION:
                if not otptDir:
                    z = str(Path(inptDir)).removesuffix(".uz2")
                    return z

                name = Path(inptDir).stem
                potptDir = Path(otptDir)

                if not potptDir.exists():
                    potptDir.mkdir()

                y = potptDir / name
                return str(y)


def main(input_args: list[str]) -> None:
    parser = argparse.ArgumentParser()

    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--c", "--compress", type=str)
    group.add_argument("--d", "--decompress", type=str)

    parser.add_argument("--o",'--output', type=str, required=False)
    args: argparse.Namespace = parser.parse_args(input_args)

    uz2_api = UZ2API()
    try:
        if args.c:
            uz2_api.compress(args.c, args.o)
        if args.d:
            uz2_api.uncompress(args.d, args.o)

    except Exception as e:
        print("Error while trying to execute: ", str(e))


if __name__ == "__main__":
    main(sys.argv[1:])
