// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![allow(clippy::cast_precision_loss)]
use crate::types::{InputArguments, LogLevel, ProcessingResult};
use crate::{constants, errors};
use sha1_smol::Sha1;
use std::path::PathBuf;
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Seek},
};
use std::{fs, io};

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
    /// # Errors
    ///
    /// Will return `Err` if fail to create output stream.
    fn open_output_ue_stream(&self) -> Result<BufWriter<File>, io::Error>;
    /// Create `BufReader` for input stream.
    /// # Errors
    ///
    /// Will return `Err` if fail to create input stream.
    fn open_input_ue_stream(&self) -> Result<BufReader<File>, io::Error>;
    /// Check the input's UE header and create `BufReader`.
    /// # Errors
    ///
    /// Will return `Err` if fail to create input stream.
    fn open_input_ue_stream_with_checks(
        &self,
    ) -> Result<BufReader<File>, errors::CompressStreamError>;
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
        self.extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| constants::DEFAULT_EXTENSIONS.contains(&extension))
    }

    fn has_uz2_extension(&self) -> bool {
        self.extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension.to_lowercase() == constants::COMPRESSED_EXTENSION)
    }

    fn open_output_ue_stream(&self) -> Result<BufWriter<File>, io::Error> {
        Ok(BufWriter::new(File::create(self)?))
    }

    fn open_input_ue_stream(&self) -> Result<BufReader<File>, io::Error> {
        Ok(BufReader::new(File::open(self)?))
    }

    fn open_input_ue_stream_with_checks(
        &self,
    ) -> Result<BufReader<File>, errors::CompressStreamError> {
        let mut reader = BufReader::new(File::open(self)?);

        match reader.file_header_is_correct() {
            Ok(()) => Ok(reader),
            Err(_) => Err(errors::CompressStreamError::InvalidPackage(self.clone())),
        }
    }
}

pub trait FileCheck {
    /// Check if this file is a valid UE package.
    /// # Errors
    ///
    /// Will return `Err` if fail to read / rewind or signature doesn't match.
    fn file_header_is_correct(&mut self) -> Result<(), errors::OtherError>;
}

impl FileCheck for BufReader<File> {
    fn file_header_is_correct(&mut self) -> Result<(), errors::OtherError> {
        let mut buf_file_header: Vec<u8> = vec![0u8; 4];
        self.read_exact(&mut buf_file_header)?;
        self.rewind()?;

        if buf_file_header == constants::KF_SIGNATURE {
            Ok(())
        } else {
            Err(errors::OtherError::InvalidFileHeader)
        }
    }
}

/// Validate path before compression attempt.
/// # Errors
///
/// Will return `Err` if one of checks fail.
pub fn validate_compressible_path(
    input_arguments: &mut InputArguments,
) -> Result<(), errors::CompressStreamError> {
    // input is a directory
    if !input_arguments.input_path.is_file() {
        return Err(errors::CompressStreamError::FileDoesntExist(
            input_arguments.input_path.clone(),
        ));
    }
    // input has `uz2` extension
    if input_arguments.input_path.has_uz2_extension() {
        return Err(errors::CompressStreamError::FileAlreadyCompressed(
            input_arguments.input_path.clone(),
        ));
    }
    // ignore core kf1 files or not
    if input_arguments.ignore_kf_files {
        if !input_arguments.input_path.is_default_kf_extension() {
            return Err(errors::CompressStreamError::NotKFExtension(
                input_arguments.input_path.clone(),
            ));
        }
        if input_arguments.input_path.is_vanilla_package() {
            return Err(errors::CompressStreamError::IsKFPackage(
                input_arguments.input_path.clone(),
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
                errors::CompressStreamError::CreateDirError(e, input_arguments.output_path.clone())
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
            return Err(errors::CompressStreamError::FileNameError(
                input_arguments.output_path.clone(),
            ));
        }
    }

    Ok(())
}

/// Validate path before decompression attempt.
/// # Errors
///
/// Will return `Err` if one of checks fail.
pub fn validate_decompressible_path(
    input_arguments: &mut InputArguments,
) -> Result<(), errors::DecompressStreamError> {
    // input is a directory
    if !input_arguments.input_path.is_file() {
        return Err(errors::DecompressStreamError::FileDoesntExist(
            input_arguments.input_path.clone(),
        ));
    }
    // input has `uz2` extension
    if !input_arguments.input_path.has_uz2_extension() {
        return Err(errors::DecompressStreamError::FileAlreadyDecompressed(
            input_arguments.input_path.clone(),
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
                errors::DecompressStreamError::CreateDirError(
                    e,
                    input_arguments.output_path.clone(),
                )
            })?;
        }

        if let Some(input_file_name) = input_arguments.input_path.get_file_name() {
            input_arguments.output_path = input_arguments.output_path.join(input_file_name);
            input_arguments.output_path.set_extension("");
        } else {
            return Err(errors::DecompressStreamError::FileNameError(
                input_arguments.output_path.clone(),
            ));
        }
    }

    Ok(())
}

/// Spawn and return`sha1` hasher.
#[must_use]
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
/// ``` text
/// BitCore.u compressed in 334.3411ms
/// |-- SHA1: ee5015514aa3f641017606521cce4a2994fbf065
/// `-- Size 7491kb -> 5531kb (ratio 0.74), chunk count: 235
/// ```
pub fn additional_processing_information(info: &ProcessingResult) {
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
}
