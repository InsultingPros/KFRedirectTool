// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::{
    constants,
    utility::{self, print_verbose_information},
};

use byteorder::ByteOrder;
use flate2::write::ZlibDecoder;
use sha1_smol::Sha1;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter},
    time::Instant,
};

/// Decompress input file.
pub fn decompress(
    mut input_stream: BufReader<File>,
    mut output_stream: BufWriter<File>,
    verbose_mode: bool,
) -> io::Result<()> {
    let mut chunk_count: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; constants::CHUNK_SIZE_COMPRESSED];
    let mut first_4byte_header: Vec<u8> = vec![0u8; 4];
    let mut second_4byte_header: Vec<u8> = vec![0u8; 4];
    let mut decoder: ZlibDecoder<Vec<u8>> = ZlibDecoder::new(Vec::new());
    let mut hasher: Option<Sha1> = utility::get_sha1_hasher(verbose_mode);

    // benchmark start
    let start: Instant = Instant::now();
    // iterate over binary file in chunks!
    loop {
        let first_header_read: usize = input_stream.read(&mut first_4byte_header)?;
        if first_header_read == 0 {
            break;
        }
        validate_buffer_len(&first_4byte_header)?;

        let second_header_read: usize = input_stream.read(&mut second_4byte_header)?;
        if second_header_read == 0 {
            break;
        }
        validate_buffer_len(&second_4byte_header)?;

        let chunk_size_compressed: usize =
            byteorder::LittleEndian::read_u32(&first_4byte_header) as usize;
        if chunk_size_compressed > constants::CHUNK_SIZE_COMPRESSED {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while decompressing. Broken data?",
            ));
        }
        let chunk_size_original: usize =
            byteorder::LittleEndian::read_u32(&second_4byte_header) as usize;

        input_stream.read_exact(&mut buffer[..chunk_size_compressed])?;
        if buffer.is_empty() {
            break;
        }

        decompress_chunk(
            &buffer,
            chunk_size_compressed,
            chunk_size_original,
            &mut output_stream,
            &mut decoder,
            hasher.as_mut(),
        )?;

        chunk_count += 1;
    }
    // benchmark end
    println!("File decompressed in {:?}", start.elapsed());
    // additional info
    if verbose_mode {
        print_verbose_information(&input_stream, &output_stream, &hasher, chunk_count)?;
    }

    Ok(())
}

// throw an error if length < 4
fn validate_buffer_len(input_buffer: &Vec<u8>) -> Result<(), io::Error> {
    if input_buffer.len() < 4 {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error while reading header buffer. It has an incorrect lenght!",
        ))
    } else {
        Ok(())
    }
}

/// Decompress single chunk and write to `output_stream`.
fn decompress_chunk(
    buffer: &[u8],
    chunk_size_compressed: usize,
    chunk_size_original: usize,
    output_stream: &mut BufWriter<File>,
    decoder: &mut ZlibDecoder<Vec<u8>>,
    hasher: Option<&mut Sha1>,
) -> Result<(), io::Error> {
    decoder.write_all(&buffer[..chunk_size_compressed])?;
    let writer: &Vec<u8> = &decoder.reset(Vec::new())?;

    if writer.len() == chunk_size_original {
        output_stream.write_all(writer)?;
        if let Some(sha1) = hasher {
            sha1.update(writer);
        }
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error while decompressing. Invalid archive.",
        ))
    }
}
