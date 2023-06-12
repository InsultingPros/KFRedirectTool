// extern crate kfuz2;
extern crate kfuz2_lib;
use kfuz2_lib::utility::open_input_ue_stream;
use serial_test::serial;
use std::path::PathBuf;
mod common;
const NOCHECK_WARNING: &str = "This test should FAIL, but we disable kf checks!";

#[test]
#[serial]
fn compress_correct_file() {
    match open_input_ue_stream(
        &PathBuf::from(common::VALIDATION_FILE),
        &kfuz2_lib::State::Compression,
        false,
    ) {
        Ok(_) => println!("test_compression() finished without errors!"),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn compress_correct_file_nocheck() {
    match open_input_ue_stream(
        &PathBuf::from(common::VALIDATION_FILE),
        &kfuz2_lib::State::Compression,
        true,
    ) {
        Ok(_) => println!("test_compression() finished without errors!"),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn decompress_correct_file() {
    match open_input_ue_stream(
        &PathBuf::from(common::VALIDATION_FILE),
        &kfuz2_lib::State::Decompression,
        false,
    ) {
        Ok(_) => println!("test_compression() finished without errors!"),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn decompress_correct_file_nocheck() {
    match open_input_ue_stream(
        &PathBuf::from(common::VALIDATION_FILE),
        &kfuz2_lib::State::Decompression,
        true,
    ) {
        Ok(_) => println!("{}", NOCHECK_WARNING),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn compress_incorrect_file() {
    match open_input_ue_stream(
        &PathBuf::from(common::INCORRECT_FILE),
        &kfuz2_lib::State::Compression,
        false,
    ) {
        Ok(_) => panic!("This test should FAIL"),
        Err(error) => println!("{:?}", error),
    };
}

#[test]
#[serial]
fn compress_incorrect_file_nocheck() {
    match open_input_ue_stream(
        &PathBuf::from(common::INCORRECT_FILE),
        &kfuz2_lib::State::Compression,
        true,
    ) {
        Ok(_) => println!("{}", NOCHECK_WARNING),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn test_other_files() {
    match open_input_ue_stream(
        &PathBuf::from(common::INCORRECT_FILE_EXT),
        &kfuz2_lib::State::Compression,
        false,
    ) {
        Ok(_) => panic!("This test should FAIL!"),
        Err(error) => println!("{:?}", error),
    };
}

#[test]
#[serial]
fn test_other_files_nocheck() {
    match open_input_ue_stream(
        &PathBuf::from(common::INCORRECT_FILE_EXT),
        &kfuz2_lib::State::Compression,
        true,
    ) {
        Ok(_) => println!("{}", NOCHECK_WARNING),
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
#[serial]
fn test_404_file() {
    match open_input_ue_stream(
        &PathBuf::from(common::EMPTY_FILE),
        &kfuz2_lib::State::Compression,
        false,
    ) {
        Ok(_) => panic!("This test should FAIL!"),
        Err(error) => println!("{:?}", error),
    };
}

#[test]
#[serial]
fn test_404_file_nocheck() {
    match open_input_ue_stream(
        &PathBuf::from(common::EMPTY_FILE),
        &kfuz2_lib::State::Compression,
        true,
    ) {
        Ok(_) => panic!("{}", NOCHECK_WARNING),
        Err(error) => println!("{:?}", error),
    };
}
