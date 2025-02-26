use crate::ServerErrors;
use bytes::Bytes;
use flate2::{Compression, write::ZlibEncoder};
use futures_util::{Stream, StreamExt as _};
use std::io::BufRead;
use std::{io, vec};
use std::{
    io::{Read, Write},
    time::Instant,
};
use tokio::io::{AsyncRead, AsyncWriteExt as _, BufStream};
use tokio_util::io::ReaderStream;

/// Size of uncompressed chunks - 4 bytes, 0-32768
pub const UNCOMPRESSED_CHUNK_SIZE: usize = 32768;

/// Compresses the given reader stream in blocks and returns a stream of compressed data.
///
/// This is a stub function. Replace the TODO with your block-based compression
/// that processes the input stream and yields compressed chunks as they become available.
/// # Errors
///
#[allow(clippy::unused_async)]
pub async fn compress_file_stream<R>(
    reader: R,
) -> Result<impl Stream<Item = Result<Bytes, io::Error>> + Send, io::Error>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    // For demonstration, we simply create a stream from the reader that passes data unmodified.
    // TODO: Implement actual compression logic here, processing in blocks.
    let stream = ReaderStream::new(reader).map(|res| {
        // res.map(|chunk| {
        //     // TODO: Compress each chunk here before returning.
        //     chunk
        // })
        res
    });
    Ok(stream)
}

/// Compress input stream.
/// # Errors
///
/// Will return `Err` if fail to read / compress data or write to stream.
pub async fn compress<R>(
    input_stream: &mut R,
    output_stream: &mut BufStream<tokio::fs::File>,
) -> Result<Vec<u8>, ServerErrors>
where
    R: BufRead + ?Sized + Unpin,
{
    let mut buffer: Vec<u8> = vec![0u8; UNCOMPRESSED_CHUNK_SIZE];
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    // allocate more than enough space for result vec
    let mut result: Vec<u8> = Vec::with_capacity(input_stream.bytes().count());

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
        chunk_size_compressed.to_vec();
        // 2. Uncompressed chunk size   :   int     :   4 Bytes        :0-32768
        chunk_size_original.to_vec();
        // 3. Compressed data           :   bytes   :   0-33096 Bytes
        // compressed_bytes.to_vec();

        result.extend(chunk_size_compressed);
        result.extend(chunk_size_original);
        result.extend(compressed_bytes);
        output_stream.get_mut().write_all(&result).await?;
    }
    println!("Time elapsed: {:?}", start.elapsed());

    Ok(result)
}

#[inline]
fn compress_single_chunk(
    buffer: &[u8],
    encoder: &mut ZlibEncoder<Vec<u8>>,
) -> Result<Vec<u8>, ServerErrors> {
    // compress
    encoder.write_all(buffer)?;
    // flush contents and reset
    let compressed_chunk: Vec<u8> = encoder.reset(Vec::new())?;

    Ok(compressed_chunk)
}
