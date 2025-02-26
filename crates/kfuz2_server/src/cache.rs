use bytes::Bytes;
use futures_util::{Stream, future::BoxFuture, stream};
// use futures::FutureExt;
// use futures::future::BoxFuture;
// use futures::stream::Stream;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tokio::sync::Mutex as AsyncMutex;

/// A simple cache system that centralizes file compression and caches the result
/// in memory (and optionally on disk) to avoid duplicate work.
pub struct Cache {
    memory_limit: usize,
    disk_limit: usize,
    /// In-memory cache: maps a file path to its compressed data.
    /// (For simplicity, we assume the entire compressed file fits in memory.)
    memory_cache: AsyncMutex<HashMap<PathBuf, Bytes>>,
    /// In-progress compressions: maps a file path to the compression future.
    in_progress: AsyncMutex<
        HashMap<
            PathBuf,
            BoxFuture<
                'static,
                Result<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>, io::Error>,
            >,
        >,
    >,
    // TODO: Add disk cache structures if you want to dump compressed files to disk.
}

impl Cache {
    /// Creates a new cache with the given memory and disk space limits.
    pub fn new(memory_limit: usize, disk_limit: usize) -> Self {
        Cache {
            memory_limit,
            disk_limit,
            memory_cache: AsyncMutex::new(HashMap::new()),
            in_progress: AsyncMutex::new(HashMap::new()),
        }
    }

    // Returns a cached compressed stream if available or uses the provided
    // compressor function to generate it. Multiple simultaneous requests for
    // the same file share the same in-progress compression.
    // pub async fn get_or_compress<F>(
    //     &self,
    //     file_path: PathBuf,
    //     compressor: F,
    // ) -> Result<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>, io::Error>
    // where
    //     F: FnOnce() -> BoxFuture<
    //             'static,
    //             Result<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>, io::Error>,
    //         > + Send
    //         + 'static,
    // {
    //     // First, check the in-memory cache.
    //     {
    //         let cache = self.memory_cache.lock().await;
    //         if let Some(data) = cache.get(&file_path) {
    //             let stream = stream::once(async move { Ok(data.clone()) });
    //             return Ok(
    //                 Box::new(stream) as Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>
    //             );
    //         }
    //     }

    //     // Check if a compression is already in progress.
    //     {
    //         let mut in_progress = self.in_progress.lock().await;
    //         // if let Some(fut) = in_progress.get(&file_path) {
    //         //     // Await the in-progress compression and return its stream.
    //         //     let stream = fut.await?;
    //         //     return Ok(stream);
    //         // }
    //         // Start a new compression task.
    //         let fut = compressor();
    //         in_progress.insert(file_path.clone(), fut);
    //     }

    //     // Wait for the compression to finish.
    //     let stream = {
    //         let mut in_progress = self.in_progress.lock().await;
    //         let fut = in_progress
    //             .remove(&file_path)
    //             .expect("Compression future must exist");
    //         fut.await?
    //     };

    //     // TODO: Optionally collect the compressed data and store it in memory (or on disk)
    //     // if it fits within the configured limits.

    //     Ok(stream)
    // }
}
