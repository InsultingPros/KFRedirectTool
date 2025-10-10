// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use kfuz2_cli::{Options, compose_input_arguments, exit_codes};
use kfuz2_lib::{
    helper::{try_to_compress, try_to_decompress},
    types::{InputArguments, LogLevel::Minimal},
};
use std::process::ExitCode;

fn main() -> ExitCode {
    // get enviroment arguments
    let env_arguments: Options = gumdrop::Options::parse_args_default_or_exit();

    // compose arguments for internal use
    let mut input_arguments: InputArguments = match compose_input_arguments(&env_arguments) {
        Ok(result) => result,
        Err(exit_code) => {
            eprintln!(
                "Terminated with error: Missing required arguments. Try 'kfuz2_cli --help' for more information."
            );
            return exit_code;
        }
    };

    // process file
    let operation = if env_arguments.decompress.is_some() {
        try_to_decompress
    } else {
        try_to_compress
    };

    match operation(&mut input_arguments) {
        Ok(()) => ExitCode::from(exit_codes::ERROR_SUCCESS),
        Err(e) => {
            if input_arguments.log_level != Minimal {
                eprintln!("Terminated with error: {e}");
            }
            std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE))
        }
    };

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}
