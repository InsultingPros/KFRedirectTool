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

    gui_app
        .file_total_num
        .swap(file_list.len() as u16, Ordering::AcqRel);
    gui_app.file_current_num.swap(0u16, Ordering::AcqRel);

    println!("Starting compression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        file_list.iter().for_each(|file_list_path| {
            try_to_compress(&mut InputArguments {
                input_path: file_list_path.into(),
                output_path: gui_app.output_dir.clone().unwrap(),
                ignore_kf_files: gui_app.ignore_kf_files,
                log_level: gui_app.log_level,
            })
            .unwrap_or_else(|e| println!("{}", e));
        });
        gui_app.file_current_num.fetch_add(1, Ordering::AcqRel);
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    chunk.iter().for_each(|chunk_path| {
                        try_to_compress(&mut InputArguments {
                            input_path: chunk_path.into(),
                            output_path: gui_app.output_dir.clone().unwrap(),
                            ignore_kf_files: gui_app.ignore_kf_files,
                            log_level: gui_app.log_level,
                        })
                        .unwrap_or_else(|e| println!("{}", e));
                        gui_app.file_current_num.fetch_add(1, Ordering::AcqRel);
                    });
                });
            }
        });
    }

    println!(
        "Compression done in {:?}, files: {}",
        start.elapsed(),
        file_list.len()
    );
}

pub fn start_decompression(gui_app: &ui::app::MyApp) {
    let file_list: Vec<PathBuf> = collect_input_files(gui_app);

    gui_app
        .file_total_num
        .swap(file_list.len() as u16, Ordering::AcqRel);
    gui_app.file_current_num.swap(0u16, Ordering::AcqRel);

    println!("Starting decompression!");
    let start: Instant = Instant::now();

    if gui_app.disable_multi_threading {
        file_list.iter().for_each(|file_list_path| {
            try_to_decompress(&mut InputArguments {
                input_path: file_list_path.into(),
                output_path: gui_app.output_dir.clone().unwrap(),
                ignore_kf_files: gui_app.ignore_kf_files,
                log_level: gui_app.log_level,
            })
            .unwrap_or_else(|e| println!("{}", e));
        });
        gui_app.file_current_num.fetch_add(1, Ordering::AcqRel);
    } else {
        let num_cpu: usize = num_cpus::get();

        rayon::scope(|s| {
            for chunk in file_list.chunks(num_cpu) {
                s.spawn(move |_| {
                    chunk.iter().for_each(|chunk_path| {
                        try_to_decompress(&mut InputArguments {
                            input_path: chunk_path.into(),
                            output_path: gui_app.output_dir.clone().unwrap(),
                            ignore_kf_files: gui_app.ignore_kf_files,
                            log_level: gui_app.log_level,
                        })
                        .unwrap_or_else(|e| println!("{}", e));
                        gui_app.file_current_num.fetch_add(1, Ordering::AcqRel);
                    });
                });
            }
        });
    }

    println!(
        "Decompression done in {:?}, files: {}",
        start.elapsed(),
        file_list.len()
    );
}

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
) -> Result<(), DecompressStreamError> {
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
