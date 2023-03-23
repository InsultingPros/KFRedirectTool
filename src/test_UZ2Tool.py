import unittest

from UZ2Tool import UZ2API
from pathlib import Path
from os.path import realpath
from hashlib import sha1

def create_dir(input_path: Path) -> None:
    if not input_path.exists():
        input_path.mkdir()

def delete_file(input_file: Path) -> None:
    try:
        if input_file.exists():
            input_file.unlink()
    except Exception as e:
            exit("Failed to delete the file: " + str(e))

# reference: https://stackoverflow.com/a/17782753
def sha1_for_file(path: Path, block_size: int=256*128) -> str:
    sha = sha1()
    with open(path,'rb') as f:
        for chunk in iter(lambda: f.read(block_size), b''):
            sha.update(chunk)

    return sha.hexdigest()


class TestStringMethods(unittest.TestCase):
    def setUp(self) -> None:
        test_file_name: str = "BitCore.u"
        path_my_dir: Path = Path(realpath(__file__)).parent
        self.path_test_file: Path = path_my_dir.joinpath("test_file", test_file_name)

        self.path_compressed_dir: Path = path_my_dir.joinpath("test_file", "Compressed")
        create_dir(self.path_compressed_dir)
        self.path_compressed_file: Path = (
            self.path_compressed_dir.joinpath(test_file_name).with_suffix(".u.uz2")
            )

        self.path_decompressed_dir: Path = path_my_dir.joinpath("test_file", "Decompressed")
        create_dir(self.path_decompressed_dir)
        self.path_decompressed_file: Path = (
            self.path_decompressed_dir.joinpath(test_file_name).with_suffix(".u")
            )

    def test_compression_decompression(self) -> None:
        uz2_api = UZ2API()

        # check compression result
        self.assertEqual(
            uz2_api.compress(str(self.path_test_file), str(self.path_compressed_dir)),
            True
            )

        # check decompression result
        self.assertEqual(
            uz2_api.uncompress(
            str(self.path_compressed_file), str(self.path_decompressed_dir)
            ),
            True
            )

        # compare hashes
        self.assertEqual(
            sha1_for_file(self.path_test_file),
            sha1_for_file(self.path_decompressed_file)
            )

        # cleanup
        delete_file(self.path_decompressed_file)
        delete_file(self.path_compressed_file)


if __name__ == '__main__':
    unittest.main()
