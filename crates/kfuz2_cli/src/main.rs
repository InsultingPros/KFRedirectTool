// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

// Make rules more strict
#![warn(clippy::all, clippy::pedantic)]
// I will enable this lint only after `rust-analyzer` matures and supports variable renaming inside curly brackets
#![allow(clippy::uninlined_format_args)]

use kfuz2_cli::{
    cli,
    helper::{compose_input_arguments, try_to_compress, try_to_decompress},
    types::exit_codes,
};
use kfuz2_lib::types::{InputArguments, LogLevel::Silent};
use std::process::ExitCode;

fn main() -> ExitCode {
    // get enviroment arguments
    let env_arguments: cli::Options = gumdrop::Options::parse_args_default_or_exit();

    // compose arguments for internal use
    let mut input_arguments: InputArguments = match compose_input_arguments(&env_arguments) {
        Ok(result) => result,
        Err(exit_code) => return exit_code,
    };

    // process file
    if env_arguments.decompress.is_some() {
        match try_to_decompress(&mut input_arguments) {
            Ok(_) => ExitCode::from(exit_codes::ERROR_SUCCESS),
            Err(e) => {
                if input_arguments.log_level != Silent {
                    eprintln!("Terminated with error: {}", e);
                }
                std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE))
            }
        }
    } else {
        match try_to_compress(&mut input_arguments) {
            Ok(_) => ExitCode::from(exit_codes::ERROR_SUCCESS),
            Err(e) => {
                if input_arguments.log_level != Silent {
                    eprintln!("Terminated with error: {}", e);
                }
                std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE))
            }
        }
    };

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}