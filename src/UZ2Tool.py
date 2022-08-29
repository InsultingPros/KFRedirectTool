# The only KF 1 Redirect tool you want to use
# Author        : NikC-
# Home Repo     : https://github.com/InsultingPros/KFRedirectTool

from hashlib import sha1
from zlib import compress, decompress
from pathlib import Path

KFFILETAG: bytes = b'\xC2' + b'\x83' + b'\x2A' + b'\x9E'
# UNREALFILETAG: bytes = b'\xC1' + b'\x83' + b'\x2A' + b'\x9E'

CHUNKSIZE_COMPRESSED: int   = 33096
CHUNKSIZE_UNCOMPRESSED: int = 32768

class renameMeLater():
    """Shiny shiny tool!"""
    def __init__(self) -> None:
        self.filesize_original: int
        self.filesize_modified: int
        self.chunks: int

    # reference : https://wiki.beyondunreal.com/UZ2_file#File_format
    def compress(self, inputfile, outputfile) -> bool:
        """Compress passed Unreal Packages"""

        if not Path(inputfile).exists():
            print(r"Package doesn't exist!")
            return False

        if inputfile.endswith('.uz2'):
            print('Package is already compressed!')
            return False

        chunk_list: list[bytes] = []
        sha2, sha3 = sha1(), sha1()
        self.chunks = 0

        # get uncompressed chunks
        with open(inputfile, 'rb') as f:
            # read first 4 bytes and check the file type
            file_tag = f.read(4)
            if not KFFILETAG in file_tag:
                print('This is not an Unreal Package!')
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
            print(f'inputFile hash is: {sha2.hexdigest()}')

        # start to write the output
        with open(outputfile, 'wb') as y:
            for z in chunk_list:
                # compress the chunk
                res = compress(z, 7)
                # get compressed data length in 4 bytes
                resl = len(res).to_bytes(4, byteorder = 'little')
                # get chunks length in 4 bytes
                reschunk = len(z).to_bytes(4, byteorder = 'little')
                # write everything in this exact order
                y.write(resl)
                sha3.update(resl)
                y.write(reschunk)
                sha3.update(reschunk)
                y.write(res)
                sha3.update(res)
            self.filesize_modified = y.tell()
            print(f'outputFile hash is: {sha3.hexdigest()}')

        return True

    def uncompress(self, inputfile, outputfile) -> bool:

        if not Path(inputfile).exists():
            print(r"Package doesn't exist!")
            return False

        if not inputfile.endswith('.uz2'):
            print('This is not a compressed file!')
            return False

        chunk_list: list[bytes] = []
        sha4, sha5 = sha1(), sha1()

        with open(inputfile, 'rb') as f:
            while cmprChunkSizeB := f.read(4):
                sha4.update(cmprChunkSizeB)
                cmprChunkSize: int = int.from_bytes(cmprChunkSizeB, byteorder = 'little')

                uncmprChunkSizeB: bytes = f.read(4)
                sha4.update(uncmprChunkSizeB)
                uncmprChunkSize: int = int.from_bytes(uncmprChunkSizeB, byteorder = 'little')

                data = f.read(cmprChunkSize)
                sha4.update(data)
                yyy = decompress(data, bufsize=uncmprChunkSize)
                # TODO check first 4 bytes from first chunk
                chunk_list.append(yyy)
            self.filesize_original = f.tell()
            print(f'inputFile hash is: {sha4.hexdigest()}')

        with open(outputfile, 'wb') as f:
            for z in chunk_list:
                f.write(z)
                sha5.update(z)
            self.filesize_modified = f.tell()
            print(f'outputFile hash is: {sha5.hexdigest()}')

            # read first 4 bytes and check the file typ
            # filetag = f.read(4)
            # if not KFFILETAG in filetag:
            #     print('This is not an Unreal Package!')
            #     return False

        return True


def main():
    # testing area, we don't need this in final version
    # SHA1 = 3baedd1ff630afc35dc3bf3a97e5aa0a0c3bcc6f
    u_File = r'D:\Games\KF Dedicated Server\System\KF2HUD.u'
    # SHA1 = e08a34fa758336a8350371ff0641d683feea7393
    uz2_File = r'D:\Games\KF Dedicated Server\Redirect\KF2HUD.u.uz2'
    # test file for testing decompression
    u_Test_File = r'D:\Games\KF Dedicated Server\System\KF2HUDTEST.u'


    r = renameMeLater()
    print(f'Compressing {u_File} to {uz2_File}!')
    if r.compress(u_File, uz2_File):
        print(f'{r.chunks=}')
        print(f'inputFile size is: {r.filesize_original} bytes')
        print(f'outputFile size is: {r.filesize_modified} bytes')
        print('Compression ratio: ' + '{:.1%}'.format(r.filesize_modified/r.filesize_original))

    print('\n')
    print(f'DeCompressing {uz2_File} to {u_Test_File}!')
    if r.uncompress(uz2_File, u_Test_File):
        print(f'inputFile size is: {r.filesize_original} bytes')
        print(f'outputFile size is: {r.filesize_modified} bytes')
        print('Compression ratio: ' + '{:.1%}'.format(r.filesize_original/r.filesize_modified))

if __name__ == "__main__":
    main()