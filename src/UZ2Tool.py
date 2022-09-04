# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool
# License       : https://www.gnu.org/licenses/gpl-3.0.en.html

from hashlib import sha1
from zlib import compress, decompress, error
from pathlib import Path
from os import path
from sys import argv
import logging

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
    def compress(self, inputfile: str, outputDir: str = None) -> bool:
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
            if not KFFILETAG in file_tag:
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

        outputfile = self.get_output_dest(inputfile, outputDir, idx=1)
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

    def uncompress(self, inputfile: str, outputDir: str = None) -> bool:

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

        outputfile = self.get_output_dest(inputfile, outputDir, idx=10)
        with open(outputfile, "wb") as f:
            for z in chunk_list:
                f.write(z)
                sha5.update(z)
            self.filesize_modified = f.tell()
            LOG.info(f"outputFile hash is: {sha5.hexdigest()}")

        return True

    def get_output_dest(
        self, inptDir: str, otptDir: str = None, idx: int = None
    ) -> str:
        match idx:
            # compression
            case 1:
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
            case _:
                if not otptDir:
                    z = str(Path(inptDir)).removesuffix(".uz2")
                    return z

                name = Path(inptDir).stem
                potptDir = Path(otptDir)

                if not potptDir.exists():
                    potptDir.mkdir()

                y = potptDir / name
                return str(y)


def main():
    # testing area, we don't need this in final version
    # SHA1 = 3baedd1ff630afc35dc3bf3a97e5aa0a0c3bcc6f
    u_File = r"D:\Games\KF Dedicated Server\System\KF2HUD.u"
    # SHA1 = e08a34fa758336a8350371ff0641d683feea7393
    uz2_File = r"D:\Games\KF Dedicated Server\Redirect\KF2HUD.u.uz2"
    # test file for testing decompression
    u_Test_File = r"D:\Games\KF Dedicated Server\System\KF2HUDTEST.u"
    # test output
    OUTPUT = r"D:\Games\KF Dedicated Server\TESTTTTT"

    r = UZ2API()
    LOG.info(f"Compressing: {u_File}, output: {OUTPUT}!")
    if r.compress(u_File, OUTPUT):
        LOG.info(f"{r.chunks=}")
        LOG.info(f"inputFile size is: {r.filesize_original} bytes")
        LOG.info(f"outputFile size is: {r.filesize_modified} bytes")
        LOG.info(
            "Compression ratio: "
            + "{:.1%}".format(r.filesize_modified / r.filesize_original)
        )

    print("\n")
    LOG.info(f"DeCompressing: {uz2_File}, output: {OUTPUT}!")
    if r.uncompress(uz2_File, OUTPUT):
        LOG.info(f"inputFile size is: {r.filesize_original} bytes")
        LOG.info(f"outputFile size is: {r.filesize_modified} bytes")
        LOG.info(
            "Compression ratio: "
            + "{:.1%}".format(r.filesize_original / r.filesize_modified)
        )


if __name__ == "__main__":
    main()
