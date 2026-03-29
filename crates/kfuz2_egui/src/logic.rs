// Author       : Shtoyan
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![allow(clippy::cast_possible_truncation)]
use crate::ui;
use kfuz2_lib::{
    errors::UZ2LibErrors,
    helper::{try_to_compress, try_to_decompress},
    types::InputArguments,
};
use rayon::prelude::*;
use std::{path::PathBuf, sync::atomic::Ordering, time::Instant};
use walkdir::WalkDir;

/// Get file list from input directory.
fn collect_input_files(gui_app: &ui::app::Kfuz2Egui) -> Vec<PathBuf> {
    let mut result = vec![];
    if let Some(x) = &gui_app.input_dir {
        for entry in WalkDir::new(x)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|ext| gui_app.extension_list.contains(ext))
            })
        {
            result.push(entry.into_path());
        }
    }
    result
}

fn set_pbar_file_length(gui_app: &ui::app::Kfuz2Egui, file_length: u16) {
    gui_app
        .pbar
        .file_num_total
        .swap(file_length, Ordering::Release);
}

/// Start compression
/// # Panics
///
/// Will panic if fail to unwrap output. FIX ME!
pub fn start_compression(gui_app: &mut ui::app::Kfuz2Egui) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_app);
    set_pbar_file_length(gui_app, file_list.len() as u16);
    println!("Starting compression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        for file_list_path in &file_list {
            parse_compression_result(file_list_path, gui_app, start);
        }
    } else {
        file_list.par_iter().for_each(|chunk_path| {
            parse_compression_result(chunk_path, gui_app, start);
        });
    }

    println!(
        "Compression done in {:?}, successful: {:?}, failed: {:?}, ignored: {:?}, canceled: {:?}, total: {:?}",
        start.elapsed(),
        gui_app.pbar.file_num_success,
        gui_app.pbar.file_num_failed,
        gui_app.pbar.file_num_ignored,
        gui_app.pbar.file_num_canceled,
        gui_app.pbar.file_num_total,
    );
    gui_app.pbar.animate = false;
}

/// Start decompression
/// # Panics
///
/// Will panic if fail to unwrap output. FIX ME!
pub fn start_decompression(gui_app: &mut ui::app::Kfuz2Egui) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_app);
    set_pbar_file_length(gui_app, file_list.len() as u16);
    println!("Starting decompression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        for file_list_path in &file_list {
            parse_decompression_result(file_list_path, gui_app, start);
        }
    } else {
        file_list.par_iter().for_each(|chunk_path| {
            parse_decompression_result(chunk_path, gui_app, start);
        });
    }

    println!(
        "Decompression done in {:?}, successful: {:?}, failed: {:?}, ignored: {:?}, canceled: {:?}, total: {:?}",
        start.elapsed(),
        gui_app.pbar.file_num_success,
        gui_app.pbar.file_num_failed,
        gui_app.pbar.file_num_ignored,
        gui_app.pbar.file_num_canceled,
        gui_app.pbar.file_num_total,
    );
    gui_app.pbar.animate = false;
}

fn parse_decompression_result(
    file_list_path: &PathBuf,
    gui_app: &ui::app::Kfuz2Egui,
    time: Instant,
) {
    let result = try_to_decompress_c(
        &mut InputArguments {
            input_path: file_list_path.into(),
            output_path: gui_app.output_dir.clone().unwrap(),
            ignore_kf_files: gui_app.ignore_kf_files,
            log_level: gui_app.log_level,
        },
        gui_app.cancel_processing.load(Ordering::Acquire),
    );
    update_pbar_file_statuses(gui_app, &result);
    update_elapsed_time(gui_app, time);
}

fn parse_compression_result(file_list_path: &PathBuf, gui_app: &ui::app::Kfuz2Egui, time: Instant) {
    let result = try_to_compress_c(
        &mut InputArguments {
            input_path: file_list_path.into(),
            output_path: gui_app.output_dir.clone().unwrap(),
            ignore_kf_files: gui_app.ignore_kf_files,
            log_level: gui_app.log_level,
        },
        gui_app.cancel_processing.load(Ordering::Acquire),
    );
    update_pbar_file_statuses(gui_app, &result);
    update_elapsed_time(gui_app, time);
}

fn update_pbar_file_statuses(gui_app: &ui::app::Kfuz2Egui, result: &Result<(), UZ2LibErrors>) {
    match result {
        Ok(()) => {
            gui_app
                .pbar
                .file_num_success
                .fetch_add(1, Ordering::Release);
        }
        Err(e) => {
            println!("{e}");

            match e {
                UZ2LibErrors::IsKFPackage(_) | UZ2LibErrors::FileAlreadyCompressed(_) => {
                    gui_app
                        .pbar
                        .file_num_ignored
                        .fetch_add(1, Ordering::Release);
                }
                UZ2LibErrors::Canceled => {
                    gui_app
                        .pbar
                        .file_num_canceled
                        .fetch_add(1, Ordering::Release);
                }
                _ => {
                    gui_app.pbar.file_num_failed.fetch_add(1, Ordering::Release);
                }
            }
        }
    }
}

fn update_elapsed_time(gui_app: &ui::app::Kfuz2Egui, time: Instant) {
    gui_app
        .pbar
        .time_elapsed
        .0
        .swap(time.elapsed().as_secs(), Ordering::Release);
    gui_app
        .pbar
        .time_elapsed
        .1
        .swap(time.elapsed().subsec_millis(), Ordering::Release);
}

/// Try to compress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly compress the data or remove file on failure.
pub fn try_to_compress_c(
    input_arguments: &mut InputArguments,
    cancel: bool,
) -> Result<(), UZ2LibErrors> {
    if cancel {
        return Err(UZ2LibErrors::Canceled);
    }
    try_to_compress(input_arguments)
}

/// Try to decompress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly decompress the data or remove file on failure.
pub fn try_to_decompress_c(
    input_arguments: &mut InputArguments,
    cancel: bool,
) -> Result<(), UZ2LibErrors> {
    if cancel {
        return Err(UZ2LibErrors::Canceled);
    }

    try_to_decompress(input_arguments)
}
