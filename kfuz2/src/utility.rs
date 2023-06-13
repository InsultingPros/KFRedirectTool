// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{constants, InputArguments, State};
use sha1_smol::Sha1;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Spawn a `BufWriter` for input.
pub fn open_output_ue_stream(input_arguments: &InputArguments) -> io::Result<BufWriter<File>> {
    Ok(BufWriter::new(File::create(input_arguments.output_path)?))
}

/// Validate and spawn a `BufReader` for input.
pub fn open_input_ue_stream(input_arguments: &InputArguments) -> io::Result<BufReader<File>> {
    let mut reader: BufReader<File> = BufReader::new(File::open(input_arguments.input_path)?);

    // skip any signature checks
    if input_arguments.nocheck {
        Ok(reader)
    } else {
        match input_arguments.app_state {
            State::Decompression => Ok(reader),
            State::Compression => match file_header_is_correct(&mut reader) {
                Ok(_) => Ok(reader),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "{:#?}: file IS NOT a kf package!",
                        input_arguments.input_path
                    ),
                )),
            },
        }
    }
}

/// Validate input-output files and return `PathBuf` array
pub fn get_input_output_paths(
    input_file: &String,
    output_file: &Option<String>,
    state: &State,
    disable_kf_checks: bool,
) -> Result<[PathBuf; 2], std::io::Error> {
    // check if input is a file
    let input_file_path: &Path = Path::new(input_file);
    if !input_file_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Input file `{}` doesn't exist!", input_file),
        ));
    }

    // validate file extension
    validate_input_file_extension(input_file_path, state)?;

    // get the file name for further use
    let file_name: &str = input_file_path
        .file_name()
        .and_then(OsStr::to_str)
        .expect("Could not extract string from OsStr!");

    if !disable_kf_checks {
        // omit vanilla files
        if constants::KF_DEFAULT_PACKAGES.contains(&file_name.to_lowercase().as_str()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{} is a default game package. Skipping it!", input_file),
            ));
        }
    }

    // 1. try to extract output path
    let output_file_path: &Path = match output_file {
        Some(result) => {
            let dir_path = Path::new(result);
            if !dir_path.exists() {
                match fs::create_dir(dir_path) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Input file `{}` doesn't exist!", input_file),
                        ));
                    }
                }
            }
            dir_path
        }
        None => input_file_path.parent().unwrap(),
    };

    // get a proper name for output file
    let output_file_name: String = match state {
        State::Decompression => file_name.replace(".uz2", ""),
        State::Compression => format!("{}.uz2", file_name),
    };

    Ok([
        input_file_path.to_path_buf(),
        output_file_path.join(output_file_name),
    ])
}

pub fn validate_input_file_extension(
    input_file_path: &Path,
    state: &State,
) -> Result<(), std::io::Error> {
    let input_is_uz2: bool = file_has_compressed_extension(input_file_path);
    let input_is_kf_package: bool = file_has_kf_extension(input_file_path);

    match state {
        State::Compression => {
            // can't compress compressed files
            if input_is_uz2 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Input file `{:?}` has 'uz2' extension. Can not compress it!",
                        input_file_path
                    ),
                ));
            }

            // can't compress files with invalid extensions
            if !input_is_kf_package {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Input file `{:?}` doesn't have valid extension. Can not compress it!",
                        input_file_path
                    ),
                ))
            } else {
                Ok(())
            }
        }

        State::Decompression => {
            // can't decompress not compressed files
            if !input_is_uz2 {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Input file `{:?}` doesn't have 'uz2' extension. Can not decompress it!",
                        input_file_path
                    ),
                ))
            } else {
                Ok(())
            }
        }
    }
}

/// Check if file extension matches with default list: u, utx, usx, ukx, uax, rom
fn file_has_kf_extension(input_path: &Path) -> bool {
    match input_path.extension().and_then(OsStr::to_str) {
        Some(extension) => constants::DEFAULT_EXTENSIONS.contains(&extension),
        _ => false,
    }
}

/// Check if file extension is `uz2`.
fn file_has_compressed_extension(input_path: &Path) -> bool {
    match input_path.extension().and_then(OsStr::to_str) {
        Some(extension) => extension.to_lowercase() == constants::COMPRESSED_EXTENSION,
        _ => false,
    }
}

/// Check if this file is a valid UE package.
fn file_header_is_correct(reader: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut buf_file_header: Vec<u8> = vec![0u8; 4];
    reader.read_exact(&mut buf_file_header)?;
    reader.rewind()?;

    if buf_file_header == constants::KF_SIGNATURE {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Incorrect file header!",
        ))
    }
}

/// Spawn and return`sha1` hasher.
pub fn get_sha1_hasher(verbose_mode: bool) -> Option<Sha1> {
    if verbose_mode {
        Some(Sha1::new())
    } else {
        None
    }
}

// we don't care about precision loss here
#[allow(clippy::cast_precision_loss)]
pub fn print_verbose_information(
    input_stream: &BufReader<File>,
    output_stream: &BufWriter<File>,
    hasher: &Option<Sha1>,
    chunk_count: u32,
) -> Result<(), std::io::Error> {
    if let Some(sha1) = hasher {
        println!("> SHA1: {:?}", sha1.digest());
    }

    let size_info: String = format!(
        "Size {:.5}kb -> {:.5}kb (ratio {:.2})",
        input_stream.get_ref().metadata()?.len() / 1024,
        output_stream.get_ref().metadata()?.len() / 1024,
        output_stream.get_ref().metadata()?.len() as f64
            / input_stream.get_ref().metadata()?.len() as f64
    );

    println!("> {}, chunk count: {}", &size_info, chunk_count);

    Ok(())
}
