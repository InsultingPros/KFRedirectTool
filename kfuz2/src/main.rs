// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
// who cares additional cargo metadata?
#![allow(clippy::uninlined_format_args, clippy::cargo_common_metadata)]
#![crate_name = "kfuz2"]
use kfuz2_lib::constants::exit_codes;
use kfuz2_lib::utility::{get_input_output_paths, open_input_ue_stream, open_output_ue_stream};
use kfuz2_lib::{compressor, decompressor, State};

use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    path::PathBuf,
    process::ExitCode,
};

fn main() -> ExitCode {
    // on failure returns 2
    let arguments: kfuz2_lib::cli::Options = gumdrop::Options::parse_args_default_or_exit();

    let (input_file_str, app_state) = if let Some(decompress_argument) = arguments.decompress {
        (decompress_argument, State::Decompression)
    } else {
        if arguments.free.is_empty() {
            return ExitCode::from(exit_codes::ERROR_BAD_ARGUMENTS);
        }
        (arguments.free[0].to_string(), State::Compression)
    };

    let [input_path, output_path] = get_input_output_paths(
        &input_file_str,
        &arguments.output,
        &app_state,
        arguments.nocheck,
    )
    .unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });

    process_file(
        &input_path,
        &output_path,
        &app_state,
        arguments.verbose,
        arguments.nocheck,
    )
    .unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}

/// Do stuff with files depending on application states
fn process_file(
    input_file_path: &PathBuf,
    output_file_path: &PathBuf,
    state: &State,
    verbose_mode: bool,
    disable_kf_checks: bool,
) -> io::Result<()> {
    let input_stream: BufReader<File> =
        open_input_ue_stream(input_file_path, state, disable_kf_checks)?;
    let output_stream: BufWriter<File> = open_output_ue_stream(output_file_path)?;

    let processing_function: fn(BufReader<File>, BufWriter<File>, bool) -> Result<(), io::Error> =
        match state {
            State::Decompression => decompressor::decompress,
            State::Compression => compressor::compress,
        };

    let result: Result<(), io::Error> =
        processing_function(input_stream, output_stream, verbose_mode);
    if result.is_err() {
        std::fs::remove_file(output_file_path)?;
    }
    result
}