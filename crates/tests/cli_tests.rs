use common::{check_if_hash_eq, execution_result};
use kfuz2_cli::types;
use serial_test::serial;

mod common;

// N.B. check these 2 again at the end!
#[test]
#[serial]
fn initial_check_validation_file_u() {
    check_if_hash_eq(common::VALIDATION_FILE_U, common::SHA1_VALIDATION_FILE_U);
}

#[test]
#[serial]
fn initial_check_validation_file_uz2() {
    check_if_hash_eq(
        common::VALIDATION_FILE_UZ2,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

// arguments
#[test]
#[serial]
fn show_help_arguments() {
    assert_eq!(
        execution_result(Some(&["-h"])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn empty_arguments() {
    assert_eq!(
        execution_result(None),
        types::exit_codes::ERROR_BAD_ARGUMENTS as i32
    );
}

#[test]
#[serial]
fn non_complete_argument_o() {
    assert_eq!(
        execution_result(Some(&["-o"])),
        types::exit_codes::ARGUMENT_PARSING_ERROR as i32
    );
}

#[test]
#[serial]
fn non_complete_argument_d() {
    assert_eq!(
        execution_result(Some(&["-d"])),
        types::exit_codes::ARGUMENT_PARSING_ERROR as i32
    );
}

// file test
#[test]
#[serial]
fn compress_incorrect_extension_exe() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE_EXT])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn compress_incorrect_file_from_exe() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn compress_nocheck_incorrect_file() {
    assert_eq!(
        execution_result(Some(&[common::INCORRECT_FILE, "--nocheck"])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn decompress_incorrect_file_uz2() {
    assert_eq!(
        execution_result(Some(&["-d", common::INCORRECT_FILE_UZ2])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn decompress_nocheck_incorrect_file_uz2() {
    assert_eq!(
        execution_result(Some(&["-d", common::INCORRECT_FILE_UZ2, "--nocheck"])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn compress_vanilla_file() {
    assert_eq!(
        execution_result(Some(&[common::VANILLA_FILE])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn compress_nocheck_vanilla_file() {
    assert_eq!(
        execution_result(Some(&[common::VANILLA_FILE, "--nocheck"])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
}

#[test]
#[serial]
fn compress_compressed_ext_file() {
    assert_eq!(
        execution_result(Some(&[common::VALIDATION_FILE_UZ2])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn decompress_decompressed_ext_file() {
    assert_eq!(
        execution_result(Some(&["-d", common::VALIDATION_FILE_U])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
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
            common::VALIDATION_FILE_U
        ])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::FILE_COMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
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
            common::VALIDATION_FILE_U
        ])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::FILE_COMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

#[test]
#[serial]
fn compression_correct_output() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE_U
        ])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::FILE_COMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

#[test]
#[serial]
fn compression_correct() {
    assert_eq!(
        execution_result(Some(&[common::VALIDATION_FILE_U])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::VALIDATION_FILE_UZ2,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

#[test]
#[serial]
fn compression_incorrect_input() {
    assert_eq!(
        execution_result(Some(&[common::OUTPUT_DECOMPRESSED])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
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
        types::exit_codes::ERROR_SUCCESS as i32
    );

    check_if_hash_eq(
        common::FILE_DECOMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_U,
    );
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
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::FILE_DECOMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_U,
    );
}

#[test]
#[serial]
fn decompression_correct() {
    assert_eq!(
        execution_result(Some(&["-d", common::VALIDATION_FILE_UZ2])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(common::VALIDATION_FILE_U, common::SHA1_VALIDATION_FILE_U);
}

#[test]
#[serial]
fn decompression_incorrect_input() {
    assert_eq!(
        execution_result(Some(&[common::OUTPUT_COMPRESSED])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
}

#[test]
#[serial]
fn mixed_args_test() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::OUTPUT_COMPRESSED,
            common::VALIDATION_FILE_U,
            "-v"
        ])),
        types::exit_codes::ERROR_SUCCESS as i32
    );
    check_if_hash_eq(
        common::FILE_COMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

#[test]
#[serial]
fn file_output_instead_of_directory() {
    assert_eq!(
        execution_result(Some(&[
            "-o",
            common::VANILLA_FILE,
            common::VALIDATION_FILE_U,
            "-v"
        ])),
        types::exit_codes::ERROR_CANNOT_MAKE as i32
    );
    check_if_hash_eq(
        common::FILE_COMPRESSED_OUTPUT,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}

#[test]
#[serial]
fn final_check_validation_file_u() {
    check_if_hash_eq(common::VALIDATION_FILE_U, common::SHA1_VALIDATION_FILE_U);
}

#[test]
#[serial]
fn final_check_validation_file_uz2() {
    check_if_hash_eq(
        common::VALIDATION_FILE_UZ2,
        common::SHA1_VALIDATION_FILE_UZ2,
    );
}
