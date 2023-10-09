// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    constants,
    errors::DecompressStreamError,
    helper::get_sha1_hasher,
    types::{InputArguments, ProcessingResult},
};
use flate2::write::ZlibDecoder;
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Write},
    time::Instant,
};

/// Decompress input file.
/// # Errors
///
/// Will return `Err` if fail to read / decompress data or write to stream.
pub fn decompress(
    input_stream: &mut BufReader<File>,
    output_stream: &mut BufWriter<File>,
    input_arguments: &InputArguments,
) -> Result<ProcessingResult, DecompressStreamError> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::COMPRESSED_CHUNK_SIZE];
    let mut compressed_chunk_size_b: [u8; 4] = [0u8; 4];
    let mut uncompressed_chunk_size_b: [u8; 4] = [0u8; 4];
    let mut decoder: ZlibDecoder<Vec<u8>> = ZlibDecoder::new(Vec::new());
    let mut hasher: Option<Sha1> = get_sha1_hasher(&input_arguments.log_level);

    let start: Instant = Instant::now();
    loop {
        // 1. read 4 bytes to get compressed chunk size
        match input_stream.read_exact(&mut compressed_chunk_size_b) {
            Ok(()) => {}
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(DecompressStreamError::IOError(Error::new(
                    e.kind(),
                    "Failed to read compressed chunk size from input",
                )));
            }
        };
        // 2. read 4 bytes to get uncompressed chunk size
        match input_stream.read_exact(&mut uncompressed_chunk_size_b) {
            Ok(()) => {}
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    return Err(DecompressStreamError::IOError(Error::new(
                        e.kind(),
                        "Tried to read beyond end of file!",
                    )));
                }
                return Err(DecompressStreamError::IOError(Error::new(
                    e.kind(),
                    "Failed to read uncompressed chunk size from input",
                )));
            }
        };
        // 1.1. get and validate `compressed` chunk size
        let compressed_chunk_size: usize = u32::from_le_bytes(compressed_chunk_size_b) as usize;
        if compressed_chunk_size > constants::COMPRESSED_CHUNK_SIZE {
            return Err(DecompressStreamError::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "compressed_chunk_size ({}) is bigger than max allowed chunk size ({})",
                    compressed_chunk_size,
                    constants::COMPRESSED_CHUNK_SIZE
                ),
            )));
        }
        // 2.1. get and validate `uncompressed` chunk size
        let uncompressed_chunk_size: usize = u32::from_le_bytes(uncompressed_chunk_size_b) as usize;
        if uncompressed_chunk_size > constants::UNCOMPRESSED_CHUNK_SIZE {
            return Err(DecompressStreamError::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "uncompressed_chunk_size ({}) is bigger than max allowed chunk size ({})",
                    uncompressed_chunk_size,
                    constants::UNCOMPRESSED_CHUNK_SIZE
                ),
            )));
        }
        // 3. read the chunk!
        match input_stream.read_exact(&mut buffer[..compressed_chunk_size]) {
            Ok(()) => {}
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(DecompressStreamError::IOError(Error::new(
                    e.kind(),
                    "Failed to read chunk from input",
                )));
            }
        };
        // 4. decompress the chunk
        let decompressed_bytes: Vec<u8> =
            decompress_single_chunk(&buffer[..compressed_chunk_size], &mut decoder)?;
        // 5. compare decompressed result with uncompressed chunk size
        if decompressed_bytes.len() != uncompressed_chunk_size {
            return Err(DecompressStreamError::IOError(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "The decompressed chunk has a different size ({}) than the saved value ({}). Damaged file?",
                    decompressed_bytes.len(),
                    uncompressed_chunk_size
                ),
            )));
        }
        // 6. write everything to output
        output_stream.write_all(&decompressed_bytes)?;
        // 7. optionally compose sha1 hash
        if let Some(ref mut sha1) = hasher {
            sha1.update(&decompressed_bytes);
        }

        chunk_count += 1;
    }

    // this also must throw error
    let input_size: u64 = input_stream.get_mut().metadata()?.len();
    let output_size: u64 = output_stream.get_mut().metadata()?.len();

    Ok(ProcessingResult {
        time: start.elapsed(),
        chunk_count,
        hasher,
        input_file_size: input_size,
        output_file_size: output_size,
    })
}

fn decompress_single_chunk(
    buffer: &[u8],
    decoder: &mut ZlibDecoder<Vec<u8>>,
) -> Result<Vec<u8>, DecompressStreamError> {
    decoder.write_all(buffer)?;
    let decompressed_chunk: Vec<u8> = decoder.reset(Vec::new())?;

    Ok(decompressed_chunk)
}
