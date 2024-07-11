#![allow(dead_code)]
use kfuz2_cli::types;
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    process::Command,
};

/// debug executable
pub const EXE_DEBUG: &str = "..//..//target//debug//kfuz2_cli";
/// release executable
pub const EXE_RELEASE: &str = "..//..//target//release//kfuz2_cli";

/// Directory for `compress` output
pub const OUTPUT_COMPRESSED: &str = "test_files//compressed";
/// Directory for `decompress` output
pub const OUTPUT_DECOMPRESSED: &str = "test_files//decompressed";

/// `SHA1` hash for validation file
pub const SHA1_VALIDATION_FILE_U: &str = "e81de24a8d78e58c104dfffbd201da416e49218a";
/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const VALIDATION_FILE_U: &str = "test_files//validation//BitCore.u";
/// `SHA1` hash for compressed validation file
pub const SHA1_VALIDATION_FILE_UZ2: &str = "ee5015514aa3f641017606521cce4a2994fbf065";
/// Processed file after `compression`, with `output` omitted
pub const VALIDATION_FILE_UZ2: &str = "test_files//validation//BitCore.u.uz2";

/// Processed file after `compression`
pub const FILE_COMPRESSED_OUTPUT: &str = "test_files//compressed//BitCore.u.uz2";
/// Processed file after `decompression`
pub const FILE_DECOMPRESSED_OUTPUT: &str = "test_files//decompressed//BitCore.u";

/// Incorrect file extension
pub const INCORRECT_FILE_EXT: &str = "test_files//incorrect//UCC.exe";
/// Correct extension, but incorrect package
pub const INCORRECT_FILE: &str = "test_files//incorrect//UCC.u";
/// Correct 'uz2' extension, but incorrect file
pub const INCORRECT_FILE_UZ2: &str = "test_files//incorrect//UCC.uz2";
/// Vanilla game packages must be omitted
pub const VANILLA_FILE: &str = "test_files//incorrect//KFMutators.u";
/// Empty file
pub const EMPTY_FILE: &str = "";

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
        panic!("Could not gather {}'s SHA1...", input_file);
    }
}

// if this panics - let it happen!
pub fn execution_result(input_args: Option<&[&str]>) -> i32 {
    match input_args {
        Some(args) => {
            let status: std::process::ExitStatus = Command::new(EXE_RELEASE)
                .args(args)
                .status()
                .expect("failed to execute process");

            status.code().expect("Status code was none!")
        }
        None => types::exit_codes::ERROR_BAD_ARGUMENTS as i32,
    }
}
