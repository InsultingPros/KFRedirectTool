// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    InputArguments, LogLevel, ProcessingResult, constants,
    errors::UZ2LibErrors,
    print_additional_information,
    validator::{PathChecks as _, validate_compressible_paths},
};
use sha1_smol::Sha1;
use std::{
    io::{Read, Write},
    time::Instant,
};
use zlib_rs::{DeflateConfig, ReturnCode, compress_bound, compress_slice};

/// Try to compress given file.
/// # Errors
///
/// Will return `Err` if fail to create input-output streams, correctly compress the data or remove file on failure.
pub fn run_compression(input_arguments: &mut InputArguments) -> Result<(), UZ2LibErrors> {
    validate_compressible_paths(input_arguments)?;

    // create streams
    let mut output_stream = input_arguments.output_path.open_output_ue_stream()?;
    let mut input_stream = input_arguments.input_path.open_input_ue_stream()?;

    let op = match input_arguments.log_level {
        LogLevel::Verbose => compress_with_hash,
        _ => compress,
    };

    match op(&mut input_stream, &mut output_stream) {
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
pub fn compress(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
) -> Result<ProcessingResult, UZ2LibErrors> {
    compress_inner(input_stream, output_stream, false)
}

/// Compress input stream.
/// # Errors
///
/// Will return `Err` if fail to read / compress data or write to stream.
pub fn compress_with_hash(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
) -> Result<ProcessingResult, UZ2LibErrors> {
    compress_inner(input_stream, output_stream, true)
}

fn compress_inner(
    input_stream: &mut impl Read,
    output_stream: &mut impl Write,
    hash_output: bool,
) -> Result<ProcessingResult, UZ2LibErrors> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::UNCOMPRESSED_CHUNK_SIZE];
    let mut compress_buf: Vec<u8> = vec![0u8; compress_bound(constants::UNCOMPRESSED_CHUNK_SIZE)];
    let deflate_config: DeflateConfig = DeflateConfig::default();
    let mut hasher: Option<Sha1> = if hash_output { Some(Sha1::new()) } else { None };
    let mut input_size: u64 = 0;
    let mut output_size: u64 = 0;

    let start: Instant = Instant::now();
    // Compression for UZ2 files is done chunk-by-chunk, for more details see:
    // https://wiki.beyondunreal.com/UZ2_file#File_format
    loop {
        let bytes_read: usize = input_stream.read(&mut buffer)?;
        // update input size
        input_size += bytes_read as u64;
        if bytes_read == 0 {
            break;
        }

        let chunk_size_original: &[u8] = &buffer[..bytes_read].len().to_le_bytes()[..4];
        // let compressed_bytes: Vec<u8> = compress_single_chunk(&buffer[..bytes_read], &mut encoder)?;
        let (compressed_bytes, rc) =
            compress_slice(&mut compress_buf, &buffer[..bytes_read], deflate_config);
        // shouldn't happen, but just in case
        if rc != ReturnCode::Ok {
            return Err(UZ2LibErrors::ZlibRsError);
        }
        let chunk_size_compressed: &[u8] = &compressed_bytes.len().to_le_bytes()[..4];

        // update output size
        output_size += 8 + (compressed_bytes.len() as u64);

        // 1. Compressed chunk size     :   int     :   4 Bytes        :0-33096
        output_stream.write_all(chunk_size_compressed)?;
        // 2. Uncompressed chunk size   :   int     :   4 Bytes        :0-32768
        output_stream.write_all(chunk_size_original)?;
        // 3. Compressed data           :   bytes   :   0-33096 Bytes
        output_stream.write_all(compressed_bytes)?;

        if let Some(ref mut sha1) = hasher {
            sha1.update(chunk_size_compressed);
            sha1.update(chunk_size_original);
            sha1.update(compressed_bytes);
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
