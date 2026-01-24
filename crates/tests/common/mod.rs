use assert_cmd::cargo::CommandCargoExt as _;
use kfuz2_cli::exit_codes;
use sha1_smol::Sha1;
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
    path::Path,
    process::Command,
};

/// Directory for `compress` output.
pub const OUTPUT_COMPRESSED_DIR: &str = "test_files//output_compressed";
/// Processed file after `compression`
pub const OUTPUT_COMPRESSED_FILE: &str = "test_files//output_compressed//BitCore.u.uz2";

/// Directory for `decompress` output.
pub const OUTPUT_DECOMPRESSED_DIR: &str = "test_files//output_decompressed";
/// Processed file after `decompression`
pub const OUTPUT_DECOMPRESSED_FILE: &str = "test_files//output_decompressed//BitCore.u";

/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const REFERENCE_FILE_U: &str = "test_files//reference_correct_files//BitCore.u";
/// `SHA1` hash for validation file
pub const REFERENCE_FILE_U_HASH: &str = "e81de24a8d78e58c104dfffbd201da416e49218a";
/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const REFERENCE_FILE_UZ2: &str = "test_files//reference_correct_files//BitCore.uz2";

/// `UCC.exe`, not a valid file to process.
pub const INCORRECT_FILE_EXT: &str = "test_files//reference_incorrect_files//UCC.exe";
/// `UCC.exe` with extension changed to `u`.
pub const INCORRECT_FILE: &str = "test_files//reference_incorrect_files//UCC.u";
/// `UCC.exe` compressed and with only `uz2` extension. Should not process.
pub const INCORRECT_FILE_UZ2: &str = "test_files//reference_incorrect_files//UCC.uz2";
/// Vanilla `KFMutators.u` package, to validate the `--nocheck` key.
pub const INCORRECT_FILE_VANILLA: &str = "test_files//reference_incorrect_files//KFMutators.u";

pub fn get_file_sha1(input_file: &str) -> Result<String, io::Error> {
    let mut hasher: Sha1 = Sha1::new();
    let mut buffer: Vec<u8> = vec![0u8; 1024];
    let mut reader: BufReader<File> = BufReader::new(File::open(Path::new(input_file))?);

    loop {
        let count: usize = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.digest().to_string())
}

pub fn check_if_hash_eq(input_file: &str, hash_to_compare: &str) {
    if let Ok(file_hash) = get_file_sha1(input_file) {
        assert_eq!(file_hash, hash_to_compare);
    } else {
        panic!("Could not gather {input_file}'s SHA1...");
    }
}

// https://users.rust-lang.org/t/cargo-bin-deprecated/136080/7
#[allow(deprecated)]
// if this panics - let it happen!
pub fn execute_with_arguments(input_args: Option<&[&str]>) -> i32 {
    input_args.map_or_else(
        || i32::from(exit_codes::ERROR_BAD_ARGUMENTS),
        |args| {
            let status = Command::cargo_bin("kfuz2_cli")
                .expect("no bin found!")
                .args(args)
                .status()
                .expect("failed to execute process");

            status.code().expect("Status code was none!")
        },
    )
}

pub fn cleanup_leftover_files() {
    fs::remove_dir_all(OUTPUT_COMPRESSED_DIR)
        .and_then(|()| fs::create_dir(OUTPUT_COMPRESSED_DIR))
        .expect("should not fail!");

    fs::remove_dir_all(OUTPUT_DECOMPRESSED_DIR)
        .and_then(|()| fs::create_dir(OUTPUT_DECOMPRESSED_DIR))
        .expect("should not fail!");
}
