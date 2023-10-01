use crate::ui;
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

/// Get file list from input directory.
fn collect_input_files(gui_app: &ui::app::MyApp) -> Vec<PathBuf> {
    let mut result = vec![];
    if let Some(x) = &gui_app.input_dir {
        for entry in WalkDir::new(x)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                return match e.path().extension().and_then(std::ffi::OsStr::to_str) {
                    Some(ext) => gui_app.extension_list.contains(ext),
                    None => false,
                };
            })
        {
            result.push(entry.into_path());
        }
    }
    result
}

pub fn start_compression(gui_app: &ui::app::MyApp) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_app);

    // reset file counts
    gui_app.pbar.reset();
    gui_app
        .pbar
        .file_num_total
        .swap(file_list.len() as u16, Ordering::Relaxed);

    println!("Starting compression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        file_list.iter().for_each(|file_list_path| {
            match try_to_compress(
                &mut InputArguments {
                    input_path: file_list_path.into(),
                    output_path: gui_app.output_dir.clone().unwrap(),
                    ignore_kf_files: gui_app.ignore_kf_files,
                    log_level: gui_app.log_level,
                },
                gui_app.cancel_processing.load(Ordering::Relaxed),
            ) {
                Ok(_) => {
                    gui_app
                        .pbar
                        .file_num_success
                        .fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    println!("{}", e);
                    match e {
                        CompressStreamError::IsKFPackage(_) => {
                            gui_app
                                .pbar
                                .file_num_ignored
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        _ => {
                            gui_app.pbar.file_num_failed.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            };
            gui_app
                .pbar
                .time_elapsed
                .swap(start.elapsed().as_secs(), Ordering::Relaxed);
        });
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    chunk.iter().for_each(|chunk_path| {
                        match try_to_compress(
                            &mut InputArguments {
                                input_path: chunk_path.into(),
                                output_path: gui_app.output_dir.clone().unwrap(),
                                ignore_kf_files: gui_app.ignore_kf_files,
                                log_level: gui_app.log_level,
                            },
                            gui_app.cancel_processing.load(Ordering::Relaxed),
                        ) {
                            Ok(_) => {
                                gui_app
                                    .pbar
                                    .file_num_success
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Err(e) => {
                                println!("{}", e);
                                match e {
                                    CompressStreamError::IsKFPackage(_) => {
                                        gui_app
                                            .pbar
                                            .file_num_ignored
                                            .fetch_add(1, Ordering::Relaxed);
                                    }
                                    _ => {
                                        gui_app
                                            .pbar
                                            .file_num_failed
                                            .fetch_add(1, Ordering::Relaxed);
                                    }
                                }
                            }
                        };
                        gui_app
                            .pbar
                            .time_elapsed
                            .swap(start.elapsed().as_secs(), Ordering::Relaxed);
                    });
                });
            }
        });
    }

    println!(
        "Compression done in {:?}, successful: {}, failed: {}, ignored: {}, total: {}",
        start.elapsed(),
        gui_app.pbar.file_num_success.load(Ordering::Relaxed),
        gui_app.pbar.file_num_failed.load(Ordering::Relaxed),
        gui_app.pbar.file_num_ignored.load(Ordering::Relaxed),
        gui_app.pbar.file_num_total.load(Ordering::Relaxed),
    );
}

pub fn start_decompression(gui_app: &ui::app::MyApp) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_app);

    // reset file counts
    gui_app.pbar.reset();
    gui_app
        .pbar
        .file_num_total
        .swap(file_list.len() as u16, Ordering::AcqRel);

    println!("Starting decompression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        file_list.iter().for_each(|file_list_path| {
            match try_to_decompress(
                &mut InputArguments {
                    input_path: file_list_path.into(),
                    output_path: gui_app.output_dir.clone().unwrap(),
                    ignore_kf_files: gui_app.ignore_kf_files,
                    log_level: gui_app.log_level,
                },
                gui_app.cancel_processing.load(Ordering::Relaxed),
            ) {
                Ok(_) => {
                    gui_app
                        .pbar
                        .file_num_success
                        .fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    println!("{}", e);
                    gui_app.pbar.file_num_failed.fetch_add(1, Ordering::Relaxed);
                }
            };
            gui_app
                .pbar
                .time_elapsed
                .swap(start.elapsed().as_secs(), Ordering::Relaxed);
        });
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    chunk.iter().for_each(|chunk_path| {
                        match try_to_decompress(
                            &mut InputArguments {
                                input_path: chunk_path.into(),
                                output_path: gui_app.output_dir.clone().unwrap(),
                                ignore_kf_files: gui_app.ignore_kf_files,
                                log_level: gui_app.log_level,
                            },
                            gui_app.cancel_processing.load(Ordering::Relaxed),
                        ) {
                            Ok(_) => {
                                gui_app
                                    .pbar
                                    .file_num_success
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Err(e) => {
                                println!("{}", e);
                                gui_app.pbar.file_num_failed.fetch_add(1, Ordering::Relaxed);
                            }
                        };
                        gui_app
                            .pbar
                            .time_elapsed
                            .swap(start.elapsed().as_secs(), Ordering::Relaxed);
                    });
                });
            }
        });
    }

    println!(
        "Decompression done in {:?}, successful: {}, failed: {}, ignored: {}, total: {}",
        start.elapsed(),
        gui_app.pbar.file_num_success.load(Ordering::Relaxed),
        gui_app.pbar.file_num_failed.load(Ordering::Relaxed),
        gui_app.pbar.file_num_ignored.load(Ordering::Relaxed),
        gui_app.pbar.file_num_total.load(Ordering::Relaxed),
    );
}

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
                    additional_processing_information(&result)?;
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

/// try to decompress given file
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
                    additional_processing_information(&result)?;
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
