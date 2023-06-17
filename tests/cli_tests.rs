use serial_test::serial;
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    process::Command,
};

mod common;

fn get_file_sha1(input_file: &str) -> Result<String, io::Error> {
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

fn assert_or_panic_hash(input_file: &str, hash_to_compare: &str) {
    if let Ok(file_hash) = get_file_sha1(input_file) {
        assert_eq!(file_hash, hash_to_compare);
    } else {
        panic!("Could not gather {} hash...", input_file);
    }
}

// if this panics - let it happen!
fn execution_result(input_args: Option<&[&str]>) -> i32 {
    match input_args {
        Some(args) => {
            let status: std::process::ExitStatus = Command::new(common::EXE_RELEASE)
                .args(args)
                .status()
                .expect("failed to execute process");

            status.code().unwrap()
        }
        None => kfuz2_lib::constants::exit_codes::ERROR_BAD_ARGUMENTS as i32,
    }
}

#[test]
#[serial]
fn validate_file() {
    assert_or_panic_hash(common::VALIDATION_FILE, common::SHA1_UNCOMPRESSED);
}

#[test]
#[serial]
fn args_help() {
    assert_eq!(
        execution_result(Some(&["-h"])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn args_empty() {
    assert_eq!(
        execution_result(None),
        kfuz2_lib::constants::exit_codes::ERROR_BAD_ARGUMENTS as i32
    );
}

#[test]
#[serial]
fn args_incomplete() {
    assert_eq!(
        execution_result(Some(&["-o"])),
        kfuz2_lib::constants::exit_codes::ARGUMENT_PARSING_ERROR as i32
    );
}

#[test]
#[serial]
fn incorrect_extension() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE_EXT])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn incorrect_file() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn incorrect_file_nocheck() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE, "--nocheck"])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn incorrect_file_uz2() {
    assert_eq!(
        execution_result(Some(&["-d", common::INCORRECT_FILE_UZ2])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn incorrect_file_uz2_nocheck() {
    assert_eq!(
        execution_result(Some(&["-d", common::INCORRECT_FILE_UZ2, "--nocheck"])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn vanilla_file() {
    assert_eq!(
        execution_result(Some(&[common::VANILLA_FILE])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn vanilla_file_nocheck() {
    assert_eq!(
        execution_result(Some(&[common::VANILLA_FILE, "--nocheck"])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn compress_compressed_ext_file() {
    assert_eq!(
        execution_result(Some(&[common::FILE_COMPRESSED])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn decompress_decompressed_ext_file() {
    assert_eq!(
        execution_result(Some(&["-d", common::VALIDATION_FILE])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn compression_correct_output_verbose() {
    assert_eq!(
        execution_result(Some(&[
            "-v",
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED_OUTPUT, common::SHA1_COMPRESSED);
}

#[test]
#[serial]
fn compression_correct_output_verbose_quiet() {
    assert_eq!(
        execution_result(Some(&[
            "-q",
            "-v",
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED_OUTPUT, common::SHA1_COMPRESSED);
}

#[test]
#[serial]
fn compression_correct_output() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED_OUTPUT, common::SHA1_COMPRESSED);
}

#[test]
#[serial]
fn compression_correct() {
    assert_eq!(
        execution_result(Some(&[common::VALIDATION_FILE])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED, common::SHA1_COMPRESSED);
}

#[test]
#[serial]
fn compression_incorrect_input() {
    assert_eq!(
        execution_result(Some(&[common::OUTPUT_DECOMPRESSED])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn decompression_correct_output_verbose() {
    assert_eq!(
        execution_result(Some(&[
            "-v",
            "-o",
            common::OUTPUT_DECOMPRESSED,
            "-d",
            common::FILE_COMPRESSED_OUTPUT,
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );

    assert_or_panic_hash(common::FILE_DECOMPRESSED_OUTPUT, common::SHA1_UNCOMPRESSED);
}

#[test]
#[serial]
fn decompression_correct_output() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::OUTPUT_DECOMPRESSED,
            "-d",
            common::FILE_COMPRESSED_OUTPUT
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_DECOMPRESSED_OUTPUT, common::SHA1_UNCOMPRESSED);
}

#[test]
#[serial]
fn decompression_correct() {
    assert_eq!(
        execution_result(Some(&["-d", common::FILE_COMPRESSED])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::VALIDATION_FILE, common::SHA1_UNCOMPRESSED);
}

#[test]
#[serial]
fn decompression_incorrect_input() {
    assert_eq!(
        execution_result(Some(&[common::OUTPUT_COMPRESSED])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn mixed_args_test() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE,
            "-v"
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_SUCCESS as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED_OUTPUT, common::SHA1_COMPRESSED);
}

#[test]
#[serial]
fn file_output_instead_of_directory() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::VANILLA_FILE,
            common::VALIDATION_FILE,
            "-v"
        ])),
        kfuz2_lib::constants::exit_codes::ERROR_CANNOT_MAKE as i32
    );
    assert_or_panic_hash(common::FILE_COMPRESSED_OUTPUT, common::SHA1_COMPRESSED);
}
