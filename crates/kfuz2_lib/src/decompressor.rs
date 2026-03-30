// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    InputArguments, LogLevel, ProcessingResult, constants,
    errors::UZ2LibErrors,
    print_additional_information,
    validator::{PathChecks as _, validate_decompressible_paths},
};
use sha1_smol::Sha1;
use std::{
    io::{Error, ErrorKind, Read, Write},
    time::Instant,
};
use zlib_rs::{InflateConfig, ReturnCode, compress_bound, decompress_slice};

/// Try to decompress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly decompress the data or remove file on failure.
pub fn run_decompression(input_arguments: &mut InputArguments) -> Result<(), UZ2LibErrors> {
    validate_decompressible_paths(input_arguments)?;

    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;

    let op = match input_arguments.log_level {
        LogLevel::Verbose => decompress_with_hash,
        _ => decompress,
    };

    match op(&mut input_stream, &mut output_stream) {
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
                    print_additional_information(&result);
                }
            }
        }
        Err(e) => {
            std::fs::remove_file(&input_arguments.output_path)?;
            // eprintln!("Terminating: {e}");
            return Err(e);
        }
    }

    Ok(())
}

/// Compress input stream.
/// # Errors
///
/// Will return `Err` if fail to read / compress data or write to stream.
pub fn decompress(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
) -> Result<ProcessingResult, UZ2LibErrors> {
    decompress_inner(input_stream, output_stream, false)
}

/// Compress input stream.
/// # Errors
///
/// Will return `Err` if fail to read / compress data or write to stream.
pub fn decompress_with_hash(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
) -> Result<ProcessingResult, UZ2LibErrors> {
    decompress_inner(input_stream, output_stream, true)
}

fn decompress_inner(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
    hash_output: bool,
) -> Result<ProcessingResult, UZ2LibErrors> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::COMPRESSED_CHUNK_SIZE];
    let mut decompress_buf: Vec<u8> = vec![0u8; compress_bound(constants::COMPRESSED_CHUNK_SIZE)];
    let mut compressed_chunk_size_b: [u8; 4] = [0u8; 4];
    let mut uncompressed_chunk_size_b: [u8; 4] = [0u8; 4];
    let inflate_config: InflateConfig = InflateConfig::default();
    let mut hasher: Option<Sha1> = if hash_output { Some(Sha1::new()) } else { None };
    let mut input_size: u64 = 0;
    let mut output_size: u64 = 0;

    let start: Instant = Instant::now();
    loop {
        // 1. read 4 bytes to get compressed chunk size
        if let Err(e) = input_stream.read_exact(&mut compressed_chunk_size_b) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(UZ2LibErrors::IOError(Error::new(
                e.kind(),
                "Failed to read compressed chunk size from input",
            )));
        }
        // 2. read 4 bytes to get uncompressed chunk size
        if let Err(e) = input_stream.read_exact(&mut uncompressed_chunk_size_b) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(UZ2LibErrors::IOError(Error::new(
                    e.kind(),
                    "Tried to read beyond end of file!",
                )));
            }
            return Err(UZ2LibErrors::IOError(Error::new(
                e.kind(),
                "Failed to read uncompressed chunk size from input",
            )));
        }
        // 1.1. get and validate `compressed` chunk size
        let compressed_chunk_size: usize = u32::from_le_bytes(compressed_chunk_size_b) as usize;
        if compressed_chunk_size > constants::COMPRESSED_CHUNK_SIZE {
            return Err(UZ2LibErrors::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "compressed_chunk_size ({}) is bigger than max allowed chunk size ({})",
                    compressed_chunk_size,
                    constants::COMPRESSED_CHUNK_SIZE
                ),
            )));
        }

        // update input size
        input_size += 8 + (compressed_chunk_size as u64);
        // 2.1. get and validate `uncompressed` chunk size
        let uncompressed_chunk_size: usize = u32::from_le_bytes(uncompressed_chunk_size_b) as usize;
        if uncompressed_chunk_size > constants::UNCOMPRESSED_CHUNK_SIZE {
            return Err(UZ2LibErrors::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "uncompressed_chunk_size ({}) is bigger than max allowed chunk size ({})",
                    uncompressed_chunk_size,
                    constants::UNCOMPRESSED_CHUNK_SIZE
                ),
            )));
        }
        // 3. read the chunk!
        if let Err(e) = input_stream.read_exact(&mut buffer[..compressed_chunk_size]) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(UZ2LibErrors::IOError(Error::new(
                e.kind(),
                "Failed to read chunk from input",
            )));
        }
        // 4. decompress the chunk
        let (decompressed_bytes, rc) = decompress_slice(
            &mut decompress_buf,
            &buffer[..compressed_chunk_size],
            inflate_config,
        );
        // shouldn't happen, but just in case
        if rc != ReturnCode::Ok {
            return Err(UZ2LibErrors::ZlibRsError);
        }
        // 5. compare decompressed result with uncompressed chunk size
        if decompressed_bytes.len() != uncompressed_chunk_size {
            return Err(UZ2LibErrors::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "The decompressed chunk has a different size ({}) than the saved value ({}). Damaged file?",
                    decompressed_bytes.len(),
                    uncompressed_chunk_size
                ),
            )));
        }

        // update output size
        output_size += decompressed_bytes.len() as u64;

        // 6. write everything to output
        output_stream.write_all(decompressed_bytes)?;
        // 7. optionally compose sha1 hash
        if let Some(ref mut sha1) = hasher {
            sha1.update(decompressed_bytes);
        }

        chunk_count += 1;
    }

    Ok(ProcessingResult {
        time: start.elapsed(),
        chunk_count,
        hasher,
        input_file_size: input_size,
        output_file_size: output_size,
    })
}
