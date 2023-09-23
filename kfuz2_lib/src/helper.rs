// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::constants;
use crate::errors::{CompressStreamError, DecompressStreamError, OtherErrors};
use crate::types::{InputArguments, LogLevel, ProcessingResult};
use sha1_smol::Sha1;
use std::fs;
use std::path::PathBuf;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Seek},
};

pub trait PathChecks {
    /// Add `uz2` extension to self.
    fn append_compressed_ext(&mut self) -> bool;
    /// Get the file name.
    fn get_file_name(&self) -> Option<&str>;
    /// Check if the file is a part of core KF1.
    fn is_vanilla_package(&self) -> bool;
    /// Check if file extension matches with default list: `u`, `utx`, `usx`, `ukx`, `uax`, `rom`
    fn is_default_kf_extension(&self) -> bool;
    /// Check if file extension is `uz2`.
    fn has_uz2_extension(&self) -> bool;
    /// Create `BufWriter` for output stream.
    fn open_output_ue_stream(&self) -> Result<BufWriter<File>, io::Error>;
    /// Create `BufReader` for input stream.
    fn open_input_ue_stream(&self) -> Result<BufReader<File>, io::Error>;
    /// Check the input's UE header and create `BufReader`.
    fn open_input_ue_stream_with_checks(&self) -> Result<BufReader<File>, CompressStreamError>;
}

impl PathChecks for PathBuf {
    fn append_compressed_ext(&mut self) -> bool {
        let Some(old_extension) = self.extension().and_then(OsStr::to_str) else {
            return false;
        };

        self.set_extension(format!(
            "{}.{}",
            old_extension,
            constants::COMPRESSED_EXTENSION
        ))
    }

    fn get_file_name(&self) -> Option<&str> {
        self.file_name().and_then(OsStr::to_str)
    }

    fn is_vanilla_package(&self) -> bool {
        let Some(file_name) = self.get_file_name() else {
            return false;
        };

        constants::KF_DEFAULT_PACKAGES.contains(&file_name.to_lowercase().as_str())
    }

    fn is_default_kf_extension(&self) -> bool {
        match self.extension().and_then(OsStr::to_str) {
            Some(extension) => constants::DEFAULT_EXTENSIONS.contains(&extension),
            _ => false,
        }
    }

    fn has_uz2_extension(&self) -> bool {
        match self.extension().and_then(OsStr::to_str) {
            Some(extension) => extension.to_lowercase() == constants::COMPRESSED_EXTENSION,
            _ => false,
        }
    }

    fn open_output_ue_stream(&self) -> Result<BufWriter<File>, io::Error> {
        Ok(BufWriter::new(File::create(self)?))
    }

    fn open_input_ue_stream(&self) -> Result<BufReader<File>, io::Error> {
        Ok(BufReader::new(File::open(self)?))
    }

    fn open_input_ue_stream_with_checks(&self) -> Result<BufReader<File>, CompressStreamError> {
        let mut reader = BufReader::new(File::open(self)?);

        match reader.file_header_is_correct() {
            Ok(_) => Ok(reader),
            Err(_) => Err(CompressStreamError::InvalidPackage(self.to_owned())),
        }
    }
}

pub trait FileCheck {
    /// Check if this file is a valid UE package.
    fn file_header_is_correct(&mut self) -> Result<(), OtherErrors>;
}

impl FileCheck for BufReader<File> {
    fn file_header_is_correct(&mut self) -> Result<(), OtherErrors> {
        let mut buf_file_header: Vec<u8> = vec![0u8; 4];
        self.read_exact(&mut buf_file_header)?;
        self.rewind()?;

        if buf_file_header == constants::KF_SIGNATURE {
            Ok(())
        } else {
            Err(OtherErrors::InvalidFileHeader)
        }
    }
}

/// Validate path before compression attempt.
pub fn validate_compressible_path(
    input_arguments: &mut InputArguments,
) -> Result<(), CompressStreamError> {
    if !input_arguments.input_path.is_file() {
        return Err(CompressStreamError::FileDoesntExist(
            input_arguments.input_path.to_owned(),
        ));
    }

    if input_arguments.input_path.has_uz2_extension() {
        return Err(CompressStreamError::FileAlreadyCompressed(
            input_arguments.input_path.to_owned(),
        ));
    }

    if !input_arguments.disable_checks {
        if !input_arguments.input_path.is_default_kf_extension() {
            return Err(CompressStreamError::NotKFExtension(
                input_arguments.input_path.to_owned(),
            ));
        }
        if input_arguments.input_path.is_vanilla_package() {
            return Err(CompressStreamError::IsKFPackage(
                input_arguments.input_path.to_owned(),
            ));
        }
    }

    // no output specified
    if input_arguments.input_path == input_arguments.output_path {
        input_arguments.output_path.append_compressed_ext();
    }
    // with output
    else {
        // create directory if it doesn't exist
        if !input_arguments.output_path.exists() {
            fs::create_dir(&input_arguments.output_path).map_err(|e| {
                CompressStreamError::CreateDirError(e, input_arguments.output_path.to_owned())
            })?;
        }
        // convert directory path to final file path
        if let Some(input_file_name) = input_arguments.input_path.get_file_name() {
            input_arguments.output_path = input_arguments.output_path.join(format!(
                "{}.{}",
                input_file_name,
                constants::COMPRESSED_EXTENSION
            ));
        } else {
            return Err(CompressStreamError::FileNameError(
                input_arguments.output_path.to_owned(),
            ));
        }
    }

    Ok(())
}

/// Validate path before decompression attempt.
pub fn validate_decompressible_path(
    input_arguments: &mut InputArguments,
) -> Result<(), DecompressStreamError> {
    if !input_arguments.input_path.is_file() {
        return Err(DecompressStreamError::FileDoesntExist(
            input_arguments.input_path.to_owned(),
        ));
    }

    if !input_arguments.input_path.has_uz2_extension() {
        return Err(DecompressStreamError::FileAlreadyDecompressed(
            input_arguments.input_path.to_owned(),
        ));
    }

    // no output specified
    if input_arguments.input_path == input_arguments.output_path {
        input_arguments.output_path.set_extension("");
    }
    // with output
    else {
        if !input_arguments.output_path.exists() {
            fs::create_dir(&input_arguments.output_path).map_err(|e| {
                DecompressStreamError::CreateDirError(e, input_arguments.output_path.to_owned())
            })?;
        }

        if let Some(input_file_name) = input_arguments.input_path.get_file_name() {
            input_arguments.output_path = input_arguments.output_path.join(input_file_name);
            input_arguments.output_path.set_extension("");
        } else {
            return Err(DecompressStreamError::FileNameError(
                input_arguments.output_path.to_owned(),
            ));
        }
    }

    Ok(())
}

/// Spawn and return`sha1` hasher.
pub fn get_sha1_hasher(log_level: &LogLevel) -> Option<Sha1> {
    match log_level {
        LogLevel::Verbose => Some(Sha1::new()),
        _ => None,
    }
}

/// Print processed file's SHA1, chunks, file sizes and ratio.
///
/// ## Example
///
/// ``` ignore
/// BitCore.u compressed in 334.3411ms
/// |-- SHA1: ee5015514aa3f641017606521cce4a2994fbf065
/// `-- Size 7491kb -> 5531kb (ratio 0.74), chunk count: 235
/// ```
pub fn additional_processing_information(info: &ProcessingResult) -> io::Result<()> {
    if let Some(sha1) = &info.hasher {
        println!("|-- SHA1: {}", sha1.digest());
    }

    let size_info: String = format!(
        "Size {:.5}kb -> {:.5}kb (ratio {:.2})",
        info.input_file_size / 1024,
        info.output_file_size / 1024,
        info.output_file_size as f64 / info.input_file_size as f64
    );

    println!("`-- {}, chunk count: {}", &size_info, info.chunk_count);

    Ok(())
}
