// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{exit_codes, Options};
use kfuz2_lib::compressor::compress;
use kfuz2_lib::decompressor::decompress;
use kfuz2_lib::errors::{CompressStreamError, DecompressStreamError};
use kfuz2_lib::helper::{
    additional_processing_information, validate_compressible_path, validate_decompressible_path,
    PathChecks,
};
use kfuz2_lib::types::{InputArguments, LogLevel};
use std::{path::PathBuf, process::ExitCode};

/// Compose arguments for internal use
/// # Errors
///
/// Will return `Err` if input is none.
pub fn compose_input_arguments(env_arguments: &Options) -> Result<InputArguments, ExitCode> {
    // 1. vanilla file check
    let mut result: InputArguments = InputArguments {
        ignore_kf_files: !env_arguments.nocheck,
        ..Default::default()
    };
    // 2. input path
    // decompression
    if let Some(decompress_argument) = &env_arguments.decompress {
        result.input_path = PathBuf::from(decompress_argument);
    }
    // compression
    else {
        if env_arguments.input_file.is_empty() {
            return Err(ExitCode::from(exit_codes::ERROR_BAD_ARGUMENTS));
        }
        result.input_path = PathBuf::from(&env_arguments.input_file[0]);
    }
    // 3. output path
    if let Some(extracted_output) = &env_arguments.output {
        result.output_path = PathBuf::from(extracted_output);
    } else {
        // if none, assign same path as input. Will use this in further checks
        result.output_path.clone_from(&result.input_path);
    }

    // silent has higher priority
    if env_arguments.quiet {
        result.log_level = LogLevel::Minimal;
        return Ok(result);
    }

    if env_arguments.verbose {
        result.log_level = LogLevel::Verbose;
        return Ok(result);
    }

    Ok(result)
}

/// Try to compress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly compress the data or remove file on failure.
pub fn try_to_compress(input_arguments: &mut InputArguments) -> Result<(), CompressStreamError> {
    validate_compressible_path(input_arguments)?;

    // create streams
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;
    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;

    match compress(&mut input_stream, &mut output_stream, input_arguments) {
        Ok(result) => {
            if input_arguments.log_level != LogLevel::Minimal {
                println!(
                    "{} compressed in {:?}",
                    input_arguments
                        .input_path
                        .get_file_name()
                        .unwrap_or("Should not fail!"),
                    result.time
                );
                if input_arguments.log_level == LogLevel::Verbose {
                    additional_processing_information(&result);
                }
            }
        }
        Err(e) => {
            std::fs::remove_file(&input_arguments.output_path)?;
            // eprintln!("Terminating: {e}");
            return Err(e);
        }
    };

    Ok(())
}

/// Try to decompress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly decompress the data or remove file on failure.
pub fn try_to_decompress(
    input_arguments: &mut InputArguments,
) -> Result<(), DecompressStreamError> {
    validate_decompressible_path(input_arguments)?;

    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;

    match decompress(&mut input_stream, &mut output_stream, input_arguments) {
        Ok(result) => {
            if input_arguments.log_level != LogLevel::Minimal {
                println!(
                    "{} decompressed in {:?}",
                    input_arguments
                        .input_path
                        .get_file_name()
                        .unwrap_or("Should not fail!"),
                    result.time
                );
                if input_arguments.log_level == LogLevel::Verbose {
                    additional_processing_information(&result);
                }
            }
        }
        Err(e) => {
            std::fs::remove_file(&input_arguments.output_path)?;
            // eprintln!("Terminating: {e}");
            return Err(e);
        }
    };

    Ok(())
}
