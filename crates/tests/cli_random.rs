use crate::common::execute_with_arguments;
use kfuz2_cli::exit_codes;

mod common;
// todo: add mixed arguments test

#[test]
fn empty_arguments() {
    assert_eq!(
        execute_with_arguments(None),
        i32::from(exit_codes::ERROR_BAD_ARGUMENTS)
    );
}

#[test]
fn show_help_text() {
    assert_eq!(
        execute_with_arguments(Some(&["-h"])),
        i32::from(exit_codes::ERROR_SUCCESS)
    );
}

#[test]
fn incomplete_argument_o() {
    assert_eq!(
        execute_with_arguments(Some(&["-o"])),
        i32::from(exit_codes::ARGUMENT_PARSING_ERROR)
    );
}

#[test]
fn incomplete_argument_d() {
    assert_eq!(
        execute_with_arguments(Some(&["-d"])),
        i32::from(exit_codes::ARGUMENT_PARSING_ERROR)
    );
}
