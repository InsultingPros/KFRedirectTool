use crate::common::{execute_with_arguments, get_temp_dir};
use kfuz2_cli::exit_codes;

mod common;

// file test
#[test]
fn compress_incorrect_extension_exe() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_UCC_EXE
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}

#[test]
fn compress_incorrect_file_from_exe() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_UCC_U
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compress_nocheck_incorrect_file() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_UCC_U,
            "--nocheck"
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compress_vanilla_file() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_KFMUTATORS_U
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}

#[test]
fn compress_nocheck_vanilla_file() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_KFMUTATORS_U,
            "--nocheck"
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compress_compressed_ext_file() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_UZ2
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
    tmp_dir.close().unwrap();
}

#[test]
fn compression_correct_output_verbose() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-v",
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_U
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compression_correct_output_verbose_quiet() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-q",
            "-v",
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_U
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compression_correct_output() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_U
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compression_correct() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_U
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn compression_incorrect_input() {
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

#[test]
fn mixed_args_test() {
    let tmp_dir = get_temp_dir();
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            tmp_dir.path().to_str().unwrap(),
            common::REF_BITCORE_U,
            "-v"
        ])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn file_output_instead_of_directory() {
    assert_eq!(
        execute_with_arguments(Some(&[
            "-o",
            common::REF_KFMUTATORS_U,
            common::REF_BITCORE_U,
            "-v"
        ])),
        i32::from(exit_codes::ERROR_CANNOT_MAKE)
    );
}
