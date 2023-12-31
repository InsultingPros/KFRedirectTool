// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![allow(clippy::cast_possible_truncation)]
use kfuz2_lib::{
    compressor::compress,
    decompressor::decompress,
    errors::{CompressStreamError, DecompressStreamError},
    helper::{
        additional_processing_information, validate_compressible_path,
        validate_decompressible_path, PathChecks,
    },
    types::{InputArguments, LogLevel},
};
use std::{path::PathBuf, sync::atomic::Ordering, time::Instant};
use walkdir::WalkDir;

/// Start compression
/// # Panics
///
/// Will panic if fail to unwrap output. FIX ME!
pub fn start_compression(gui_vars: &crate::ui::app::Variables) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_vars);
    reset_pbar_atomics(gui_vars, file_list.len() as u16);
    println!("Starting compression!");
    let start: Instant = Instant::now();

    if gui_vars.persisted_vars.disable_multi_threading {
        for file_list_path in &file_list {
            parse_compression_result(file_list_path, gui_vars, start);
        }
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    for chunk_path in chunk {
                        parse_compression_result(chunk_path, gui_vars, start);
                    }
                });
            }
        });
    }

    println!(
        "Compression done in {:?}, successful: {}, failed: {}, ignored: {}, total: {}",
        start.elapsed(),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_success
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_failed
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_ignored
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_total
            .load(Ordering::Relaxed),
    );
}

/// Start decompression
/// # Panics
///
/// Will panic if fail to unwrap output. FIX ME!
pub fn start_decompression(gui_vars: &crate::ui::app::Variables) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_vars);
    reset_pbar_atomics(gui_vars, file_list.len() as u16);
    println!("Starting decompression!");
    let start: Instant = Instant::now();

    if gui_vars.persisted_vars.disable_multi_threading {
        for file_list_path in &file_list {
            parse_decompression_result(file_list_path, gui_vars, start);
        }
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    for chunk_path in chunk {
                        parse_decompression_result(chunk_path, gui_vars, start);
                    }
                });
            }
        });
    }

    println!(
        "Decompression done in {:?}, successful: {}, failed: {}, ignored: {}, total: {}",
        start.elapsed(),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_success
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_failed
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_ignored
            .load(Ordering::Relaxed),
        gui_vars
            .runtime_vars
            .pbar
            .file_num_total
            .load(Ordering::Relaxed),
    );
}

fn parse_decompression_result(
    file_list_path: &PathBuf,
    gui_vars: &crate::ui::app::Variables,
    start: Instant,
) {
    match try_to_decompress(
        &mut InputArguments {
            input_path: file_list_path.into(),
            output_path: gui_vars.persisted_vars.paths.output_dir.clone().unwrap(),
            ignore_kf_files: gui_vars.persisted_vars.ignore_kf_files,
            log_level: gui_vars.persisted_vars.log_level,
        },
        gui_vars
            .runtime_vars
            .cancel_processing
            .load(Ordering::Relaxed),
    ) {
        Ok(()) => {
            gui_vars
                .runtime_vars
                .pbar
                .file_num_success
                .fetch_add(1, Ordering::Relaxed);
        }
        Err(e) => {
            println!("{e}");
            gui_vars
                .runtime_vars
                .pbar
                .file_num_failed
                .fetch_add(1, Ordering::Relaxed);
        }
    };
    set_pbar_timings(
        gui_vars,
        start.elapsed().as_secs(),
        start.elapsed().subsec_millis(),
    );
}

fn parse_compression_result(
    file_list_path: &PathBuf,
    gui_vars: &crate::ui::app::Variables,
    start: Instant,
) {
    match try_to_compress(
        &mut InputArguments {
            input_path: file_list_path.into(),
            output_path: gui_vars.persisted_vars.paths.output_dir.clone().unwrap(),
            ignore_kf_files: gui_vars.persisted_vars.ignore_kf_files,
            log_level: gui_vars.persisted_vars.log_level,
        },
        gui_vars
            .runtime_vars
            .cancel_processing
            .load(Ordering::Relaxed),
    ) {
        Ok(()) => {
            gui_vars
                .runtime_vars
                .pbar
                .file_num_success
                .fetch_add(1, Ordering::Relaxed);
        }
        Err(e) => {
            println!("{e}");
            match e {
                CompressStreamError::IsKFPackage(_) => {
                    gui_vars
                        .runtime_vars
                        .pbar
                        .file_num_ignored
                        .fetch_add(1, Ordering::Relaxed);
                }
                _ => {
                    gui_vars
                        .runtime_vars
                        .pbar
                        .file_num_failed
                        .fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    };
    set_pbar_timings(
        gui_vars,
        start.elapsed().as_secs(),
        start.elapsed().subsec_millis(),
    );
}

/// Try to compress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly compress the data or remove file on failure.
pub fn try_to_compress(
    input_arguments: &mut InputArguments,
    cancel: bool,
) -> Result<(), CompressStreamError> {
    if cancel {
        return Err(CompressStreamError::Canceled);
    }

    validate_compressible_path(input_arguments)?;

    // create streams
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;
    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;

    match compress(&mut input_stream, &mut output_stream, input_arguments) {
        Ok(result) => {
            if input_arguments.log_level != LogLevel::Minimal {
                println!(
                    "{} compressed in {:?}",
                    input_arguments.input_path.get_file_name().unwrap_or("404"),
                    result.time
                );
                if input_arguments.log_level == LogLevel::Verbose {
                    additional_processing_information(&result);
                }
            }
            Ok(())
        }
        Err(e) => {
            // print!("{}", e);
            std::fs::remove_file(&input_arguments.output_path)?;
            Err(e)
            // Err(CompressStreamError::FailedToCompress(
            //     input_arguments.input_path.to_owned(),
            // ))
        }
    }
}

/// Try to decompress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly decompress the data or remove file on failure.
pub fn try_to_decompress(
    input_arguments: &mut InputArguments,
    cancel: bool,
) -> Result<(), DecompressStreamError> {
    if cancel {
        return Err(DecompressStreamError::Canceled);
    }

    validate_decompressible_path(input_arguments)?;

    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;

    match decompress(&mut input_stream, &mut output_stream, input_arguments) {
        Ok(result) => {
            if input_arguments.log_level != LogLevel::Minimal {
                println!(
                    "{} decompressed in {:?}",
                    input_arguments.input_path.get_file_name().unwrap_or("404"),
                    result.time
                );
                if input_arguments.log_level == LogLevel::Verbose {
                    additional_processing_information(&result);
                }
            }
            Ok(())
        }
        Err(e) => {
            // print!("{}", e);
            std::fs::remove_file(&input_arguments.output_path)?;
            Err(e)
            // Err(DecompressStreamError::FailedToCompress(
            //     input_arguments.input_path.to_owned(),
            // ))
        }
    }
}

/// Get file list from input directory.
#[inline]
fn collect_input_files(gui_vars: &crate::ui::app::Variables) -> Vec<PathBuf> {
    let mut result = vec![];
    if let Some(x) = &gui_vars.persisted_vars.paths.input_dir {
        for entry in WalkDir::new(x)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| {
                return match e.path().extension().and_then(std::ffi::OsStr::to_str) {
                    Some(ext) => gui_vars
                        .persisted_vars
                        .extensions
                        .extension_list
                        .contains(ext),
                    None => false,
                };
            })
        {
            result.push(entry.into_path());
        }
    }
    result
}

/// Reset progress bar atomics
#[inline]
fn reset_pbar_atomics(gui_vars: &crate::ui::app::Variables, file_length: u16) {
    gui_vars.runtime_vars.pbar.reset();
    gui_vars
        .runtime_vars
        .pbar
        .file_num_total
        .swap(file_length, Ordering::Relaxed);
}

#[inline]
fn set_pbar_timings(gui_vars: &crate::ui::app::Variables, secs: u64, m_secs: u32) {
    gui_vars
        .runtime_vars
        .pbar
        .time_elapsed
        .0
        .swap(secs, Ordering::Relaxed);
    gui_vars
        .runtime_vars
        .pbar
        .time_elapsed
        .1
        .swap(m_secs, Ordering::Relaxed);
}
