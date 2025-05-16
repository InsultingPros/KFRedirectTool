// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use std::{io::IntoInnerError, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum UZ2LibErrors {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    InnerError(#[from] IntoInnerError<std::io::BufWriter<std::fs::File>>),
    #[error("{:#?}: file IS NOT a kf package!", .0)]
    InvalidPackage(PathBuf),
    #[error("Input `{:?}` doens't exist!", .0)]
    FileDoesntExist(PathBuf),
    #[error("Input `{:?}` is already compressed!", .0)]
    FileAlreadyCompressed(PathBuf),
    #[error("Input `{:?}` has wront extension, ignoring it!", .0)]
    NotKFExtension(PathBuf),
    #[error("Input `{:?}` is core KF1 file, ignoring it!", .0)]
    IsKFPackage(PathBuf),
    #[error("Unable to create output directory `{:?}`!", .0)]
    CreateDirError(#[source] std::io::Error, PathBuf),
    #[error("Unable to extract file name from {:?}", .0)]
    FileNameError(PathBuf),
    #[error("Failed to compress file {:?}", .0)]
    FailedToCompress(PathBuf),
    #[error("Processing canceled")]
    Canceled,
    #[error("Error while decompressing. Invalid data!")]
    InvalidData,
    #[error("Input `{:?}` is already decompressed!", .0)]
    FileAlreadyDecompressed(PathBuf),
    #[error("Incorrect file header!")]
    InvalidFileHeader,
}
