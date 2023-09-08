// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    constants, errors::CompressStreamError, helper::get_sha1_hasher, types::InputArguments,
    types::ProcessingResult,
};
use flate2::{write::ZlibEncoder, Compression};
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    time::Instant,
};

/// Compress input stream.
pub fn compress(
    input_stream: &mut BufReader<File>,
    output_stream: &mut BufWriter<File>,
    input_arguments: &InputArguments,
) -> Result<ProcessingResult, CompressStreamError> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::UNCOMPRESSED_CHUNK_SIZE];
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    let mut hasher: Option<Sha1> = get_sha1_hasher(&input_arguments.log_level);

    let start: Instant = Instant::now();
    // Compression for UZ2 files is done chunk-by-chunk, for more details see:
    // https://wiki.beyondunreal.com/UZ2_file#File_format
    loop {
        let bytes_read: usize = input_stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk_size_original: &[u8] = &buffer[..bytes_read].len().to_le_bytes()[..4];
        let compressed_bytes: Vec<u8> = compress_single_chunk(&buffer[..bytes_read], &mut encoder)?;
        let chunk_size_compressed: &[u8] = &compressed_bytes.len().to_le_bytes()[..4];

        // 1. Compressed chunk size     :   int     :   4 Bytes        :0-33096
        output_stream.write_all(chunk_size_compressed)?;
        // 2. Uncompressed chunk size   :   int     :   4 Bytes        :0-32768
        output_stream.write_all(chunk_size_original)?;
        // 3. Compressed data           :   bytes   :   0-33096 Bytes
        output_stream.write_all(&compressed_bytes)?;

        if let Some(ref mut sha1) = hasher {
            sha1.update(chunk_size_compressed);
            sha1.update(chunk_size_original);
            sha1.update(&compressed_bytes);
        }

        chunk_count += 1;
    }

    Ok(ProcessingResult {
        time: start.elapsed(),
        chunk_count,
        hasher,
        // input_stream_ref: input_stream,
        // output_stream_ref: output_stream,
    })
}

fn compress_single_chunk(
    buffer: &[u8],
    encoder: &mut ZlibEncoder<Vec<u8>>,
) -> Result<Vec<u8>, CompressStreamError> {
    // compress
    encoder.write_all(buffer)?;
    // flush contents and reset
    let compressed_chunk: Vec<u8> = encoder.reset(Vec::new())?;

    Ok(compressed_chunk)
}
