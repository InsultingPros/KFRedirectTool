// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{constants, InputArguments, State};
use anyhow::{bail, Context, Result};
use sha1_smol::Sha1;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path;

/// Spawn a `BufWriter` for input.
pub fn open_output_ue_stream(input_arguments: &InputArguments) -> Result<BufWriter<File>> {
    Ok(BufWriter::new(File::create(&input_arguments.output_path)?))
}

/// Validate and spawn a `BufReader` for input.
pub fn open_input_ue_stream(input_arguments: &InputArguments) -> Result<BufReader<File>> {
    let mut reader: BufReader<File> = BufReader::new(File::open(&input_arguments.input_path)?);

    // skip any signature checks
    if input_arguments.nocheck {
        Ok(reader)
    } else {
        match input_arguments.app_state {
            State::Decompression => Ok(reader),
            State::Compression => match file_header_is_correct(&mut reader) {
                Ok(_) => Ok(reader),
                Err(_) => bail!(format!(
                    "{:#?}: file IS NOT a kf package!",
                    input_arguments.input_path
                ),),
            },
        }
    }
}

/// Validate input-output files and return `PathBuf` array
pub fn validate_input_output_paths(input_arguments: &mut InputArguments) -> Result<()> {
    // check if input is a file
    let input_file_path: &Path = Path::new(&input_arguments.input_file_str);
    if !input_file_path.is_file() {
        bail!(format!(
            "Input file `{}` doesn't exist!",
            &input_arguments.input_file_str
        ))
    }

    // validate file extension
    validate_input_file_extension(input_file_path, input_arguments.app_state)?;

    // get the file name for further use
    let file_name: &str = input_file_path
        .file_name()
        .and_then(OsStr::to_str)
        .expect("Could not extract string from OsStr!");

    if !input_arguments.nocheck {
        // omit vanilla files
        if constants::KF_DEFAULT_PACKAGES.contains(&file_name.to_lowercase().as_str()) {
            bail!(format!(
                "{} is a default game package. Skipping it!",
                input_arguments.input_file_str
            ))
        }
    }

    // .with_context(|| format!("Failed to read instrs from {}", path.display()))?;

    // 1. try to extract output path
    let output_file_path: &Path = match &input_arguments.output_file_str {
        Some(result) => {
            let dir_path: &Path = Path::new(result);
            if !dir_path.exists() {
                fs::create_dir(dir_path)
                    .context(format!("Can not create output directory `{}`!", result))?;
            }
            dir_path
        }
        None => input_file_path.parent().unwrap(),
    };

    // get a proper name for output file
    let output_file_name: String = match input_arguments.app_state {
        State::Decompression => file_name.replace(".uz2", ""),
        State::Compression => format!("{}.uz2", file_name),
    };

    input_arguments.input_path = input_file_path.to_path_buf();
    input_arguments.output_path = output_file_path.join(output_file_name);

    Ok(())
}

pub fn validate_input_file_extension(input_file_path: &Path, state: &State) -> Result<()> {
    let input_is_uz2: bool = file_has_compressed_extension(input_file_path);
    let input_is_kf_package: bool = file_has_kf_extension(input_file_path);

    match state {
        State::Compression => {
            // can't compress compressed files
            if input_is_uz2 {
                bail!(format!(
                    "Input file `{:?}` has 'uz2' extension. Can not compress it!",
                    input_file_path
                ),)
            }

            // can't compress files with invalid extensions
            if !input_is_kf_package {
                bail!(format!(
                    "Input file `{:?}` doesn't have valid extension. Can not compress it!",
                    input_file_path
                ))
            } else {
                Ok(())
            }
        }

        State::Decompression => {
            // can't decompress not compressed files
            if !input_is_uz2 {
                bail!(format!(
                    "Input file `{:?}` doesn't have 'uz2' extension. Can not decompress it!",
                    input_file_path
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
fn file_header_is_correct(reader: &mut BufReader<File>) -> Result<()> {
    let mut buf_file_header: Vec<u8> = vec![0u8; 4];
    reader.read_exact(&mut buf_file_header)?;
    reader.rewind()?;

    if buf_file_header == constants::KF_SIGNATURE {
        Ok(())
    } else {
        bail!("Incorrect file header!")
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
) -> Result<()> {
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
