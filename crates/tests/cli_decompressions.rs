use crate::common::{check_if_hash_eq, execute_with_arguments, get_temp_dir};
use kfuz2_cli::exit_codes;

mod common;

#[test]
fn decompress_incorrect_file_uz2() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_UCC_UZ2
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}

#[test]
fn decompress_nocheck_incorrect_file_uz2() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_UCC_UZ2,
            "--nocheck"
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}

#[test]
fn decompress_decompressed_ext_file() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_BITCORE_U
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}

// FIXME!
#[test]
fn decompression_correct_output_verbose() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-v",
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_BITCORE_UZ2,
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
    let result_file = tmp_dir.path().join("BitCore.u");

    check_if_hash_eq(result_file.to_str().unwrap(), common::REF_BITCORE_U_HASH);
}

#[test]
fn decompression_correct_output() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_BITCORE_UZ2,
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
    let result_file = tmp_dir.path().join("BitCore.u");
    check_if_hash_eq(result_file.to_str().unwrap(), common::REF_BITCORE_U_HASH);
}

#[test]
fn decompression_correct() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            "-d",
            common::REF_BITCORE_UZ2
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
    let result_file = tmp_dir.path().join("BitCore.u");
    check_if_hash_eq(result_file.to_str().unwrap(), common::REF_BITCORE_U_HASH);
}

#[test]
fn decompression_incorrect_input() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            tmp_dir.path().to_str().unwrap(),
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}
