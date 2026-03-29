#![allow(dead_code)]
use kfuz2_cli::exit_codes;
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};
use tempfile::Builder;

/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const REF_BITCORE_U: &str = "reference_files//BitCore.u";
/// `SHA1` hash for validation file
pub const REF_BITCORE_U_HASH: &str = "e81de24a8d78e58c104dfffbd201da416e49218a";
/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const REF_BITCORE_UZ2: &str = "reference_files//BitCore.u.uz2";

/// `UCC.exe`, not a valid file to process.
pub const REF_UCC_EXE: &str = "reference_files//UCC.exe";
/// `UCC.exe` with extension changed to `u`.
pub const REF_UCC_U: &str = "reference_files//UCC.u";
/// `UCC.exe` compressed and with only `uz2` extension. Should not process.
pub const REF_UCC_UZ2: &str = "reference_files//UCC.uz2";
/// Vanilla `KFMutators.u` package, to validate the `--nocheck` key.
pub const REF_KFMUTATORS_U: &str = "reference_files//KFMutators.u";

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

pub fn get_temp_dir() -> tempfile::TempDir {
    Builder::new().tempdir().unwrap()
}

// if this panics - let it happen!
pub fn execute_with_arguments(input_args: Option<&[&str]>) -> i32 {
    input_args.map_or_else(
        || i32::from(exit_codes::ERROR_BAD_ARGUMENTS),
        |args| {
            let status = escargot::CargoBuild::new()
                .package("kfuz2_cli")
                .bin("kfuz2_cli")
                .current_release()
                .run()
                .expect("failed to build cli!")
                .command()
                .args(args)
                .status()
                .expect("failed to get cli run status!");

            status.code().expect("Status code was none!")
        },
    )
}
