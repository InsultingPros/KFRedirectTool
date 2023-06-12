// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    constants,
    utility::{self, print_verbose_information},
};
use flate2::{write::ZlibEncoder, Compression};
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter},
    time::Instant,
};

/// Compress input file.
pub fn compress(
    mut input_stream: BufReader<File>,
    mut output_stream: BufWriter<File>,
    verbose_mode: bool,
) -> io::Result<()> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::CHUNK_SIZE_UNCOMPRESSED];
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    let mut hasher: Option<Sha1> = utility::get_sha1_hasher(verbose_mode);

    // benchmark start
    let start: Instant = Instant::now();
    // Compression for UZ2 files is done chunk-by-chunk, for more details see:
    // https://wiki.beyondunreal.com/UZ2_file#File_format
    loop {
        let bytes_read: usize = input_stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        compress_chunk(
            &buffer,
            bytes_read,
            &mut output_stream,
            &mut encoder,
            hasher.as_mut(),
        )?;
        chunk_count += 1;
    }
    // benchmark end
    println!("File compressed in {:?}", start.elapsed());
    // additional info
    if verbose_mode {
        print_verbose_information(&input_stream, &output_stream, &hasher, chunk_count)?;
    }

    Ok(())
}

/// Compress single chunk and write to `output_stream`.
fn compress_chunk(
    buffer: &[u8],
    bytes_read: usize,
    output_stream: &mut BufWriter<File>,
    encoder: &mut ZlibEncoder<Vec<u8>>,
    hasher: Option<&mut Sha1>,
) -> Result<(), io::Error> {
    let chunk_size_original: &[u8] = &buffer[..bytes_read].len().to_le_bytes()[..4];
    encoder.write_all(&buffer[..bytes_read])?;
    let compressed_chunk: Vec<u8> = encoder.reset(Vec::new())?;
    let chunk_size_compressed: &[u8] = &compressed_chunk.len().to_le_bytes()[..4];

    // 1. Compressed chunk size     :   int     :   4 Bytes        :0-33096
    output_stream.write_all(chunk_size_compressed)?;
    // 2. Uncompressed chunk size   :   int     :   4 Bytes        :0-32768
    output_stream.write_all(chunk_size_original)?;
    // 3. Compressed data           :   bytes   :   0-33096 Bytes
    output_stream.write_all(&compressed_chunk)?;

    if let Some(sha1) = hasher {
        sha1.update(chunk_size_compressed);
        sha1.update(chunk_size_original);
        sha1.update(&compressed_chunk);
    }

    Ok(())
}
