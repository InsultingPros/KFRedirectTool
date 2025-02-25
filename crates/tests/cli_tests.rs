use common::{check_if_hash_eq, cleanup_leftover_files};
use serial_test::serial;

mod common;

// N.B. check these 2 again at the end!
#[test]
#[serial]
fn initial_check_validation_file_u() {
    check_if_hash_eq(common::REFERENCE_FILE_U, common::REFERENCE_FILE_U_HASH);
}

#[cfg(test)]
#[serial]
mod random_argument_tests {
    use crate::common::execute_with_arguments;
    use kfuz2_cli::exit_codes;

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

    // todo: add mixed arguments test
}

#[cfg(test)]
#[serial]
mod compression_tests {
    use crate::common::{self, execute_with_arguments};
    use kfuz2_cli::exit_codes;

    // file test
    #[test]
    fn compress_incorrect_extension_exe() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::INCORRECT_FILE_EXT
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn compress_incorrect_file_from_exe() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::INCORRECT_FILE
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compress_nocheck_incorrect_file() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::INCORRECT_FILE,
                "--nocheck"
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compress_vanilla_file() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::INCORRECT_FILE_VANILLA
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn compress_nocheck_vanilla_file() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::INCORRECT_FILE_VANILLA,
                "--nocheck"
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compress_compressed_ext_file() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_UZ2
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn compression_correct_output_verbose() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-v",
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_U
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compression_correct_output_verbose_quiet() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-q",
                "-v",
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_U
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compression_correct_output() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_U
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compression_correct() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_U
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
    }

    #[test]
    fn compression_incorrect_input() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::OUTPUT_DECOMPRESSED_DIR
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn mixed_args_test() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_COMPRESSED_DIR,
                common::REFERENCE_FILE_U,
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
                common::INCORRECT_FILE_VANILLA,
                common::REFERENCE_FILE_U,
                "-v"
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }
}

#[cfg(test)]
#[serial]
mod decompression_tests {
    use crate::common::{self, check_if_hash_eq, execute_with_arguments};
    use kfuz2_cli::exit_codes;

    #[test]
    fn decompress_incorrect_file_uz2() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::INCORRECT_FILE_UZ2
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn decompress_nocheck_incorrect_file_uz2() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::INCORRECT_FILE_UZ2,
                "--nocheck"
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn decompress_decompressed_ext_file() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::REFERENCE_FILE_U
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }

    #[test]
    fn decompression_correct_output_verbose() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-v",
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::OUTPUT_COMPRESSED_FILE,
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );

        check_if_hash_eq(
            common::OUTPUT_DECOMPRESSED_FILE,
            common::REFERENCE_FILE_U_HASH,
        );
    }

    #[test]
    fn decompression_correct_output() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::OUTPUT_COMPRESSED_FILE
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
        check_if_hash_eq(
            common::OUTPUT_DECOMPRESSED_FILE,
            common::REFERENCE_FILE_U_HASH,
        );
    }

    #[test]
    fn decompression_correct() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                "-d",
                common::OUTPUT_COMPRESSED_FILE
            ])),
            i32::from(exit_codes::ERROR_SUCCESS)
        );
        check_if_hash_eq(common::REFERENCE_FILE_U, common::REFERENCE_FILE_U_HASH);
    }

    #[test]
    fn decompression_incorrect_input() {
        assert_eq!(
            execute_with_arguments(Some(&[
                "-o",
                common::OUTPUT_DECOMPRESSED_DIR,
                common::OUTPUT_COMPRESSED_DIR
            ])),
            i32::from(exit_codes::ERROR_CANNOT_MAKE)
        );
    }
}

#[test]
#[serial]
fn final_check_validation_file_u() {
    check_if_hash_eq(common::REFERENCE_FILE_U, common::REFERENCE_FILE_U_HASH);
}

#[test]
#[serial]
fn file_cleanup() {
    cleanup_leftover_files();
}
