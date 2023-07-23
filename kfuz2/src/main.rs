// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
// who cares additional cargo metadata?
#![allow(clippy::uninlined_format_args, clippy::cargo_common_metadata)]
#![crate_name = "kfuz2"]
use kfuz2_lib::constants::exit_codes;
use kfuz2_lib::utility::{
    open_input_ue_stream, open_output_ue_stream, validate_input_output_paths,
};
use kfuz2_lib::{compressor, decompressor, State};

use anyhow::Result;
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    process::ExitCode,
};

fn main() -> ExitCode {
    // on failure returns 2
    let arguments: kfuz2_lib::cli::Options = gumdrop::Options::parse_args_default_or_exit();

    let input_arguments: &mut kfuz2_lib::InputArguments<'_> = &mut kfuz2_lib::InputArguments {
        input_path: PathBuf::new(),
        output_path: PathBuf::new(),
        operation_type: &State::Compression,
        verbose: arguments.verbose,
        nocheck: arguments.nocheck,
        input_file_str: String::new(),
        output_file_str: Some(String::new()),
        quiet: arguments.quiet,
    };

    if let Some(decompress_argument) = arguments.decompress {
        input_arguments.input_file_str = decompress_argument;
        input_arguments.operation_type = &State::Decompression;
    } else {
        if arguments.input_file.is_empty() {
            return ExitCode::from(exit_codes::ERROR_BAD_ARGUMENTS);
        }
        input_arguments.input_file_str = arguments.input_file[0].to_string();
    };

    input_arguments.output_file_str = arguments.output;
    validate_input_output_paths(input_arguments).unwrap_or_else(|e| {
        if !input_arguments.quiet {
            eprintln!("Terminated with error: {}", e);
        }
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });

    process_file(input_arguments).unwrap_or_else(|e| {
        if !input_arguments.quiet {
            eprintln!("Terminated with error: {}", e);
        }
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}

/// Do stuff with files depending on application states
fn process_file(input_arguments: &kfuz2_lib::InputArguments) -> Result<()> {
    let input_stream: BufReader<File> = open_input_ue_stream(input_arguments)?;
    let output_stream: BufWriter<File> = open_output_ue_stream(input_arguments)?;

    let processing_function = match input_arguments.operation_type {
        State::Decompression => decompressor::decompress,
        State::Compression => compressor::compress,
    };

    let result: Result<()> = processing_function(input_stream, output_stream, input_arguments);
    if result.is_err() {
        std::fs::remove_file(&input_arguments.output_path)?;
    }
    result
}
