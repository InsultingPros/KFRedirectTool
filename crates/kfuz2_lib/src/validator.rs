// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

use crate::InputArguments;
use crate::constants;
use crate::errors::UZ2LibErrors;
use std::path::Path;
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
    fn open_input_ue_stream_with_checks(&self) -> Result<BufReader<File>, UZ2LibErrors>;
}

impl<T: AsRef<Path>> PathChecks for T {
    fn append_compressed_ext(&mut self) -> bool {
        let mut path_buf = self.as_ref().to_path_buf();
        let Some(old_extension) = path_buf.extension().and_then(OsStr::to_str) else {
            return false;
        };

        path_buf.set_extension(format!(
            "{}.{}",
            old_extension,
            constants::COMPRESSED_EXTENSION
        ))
    }

    fn get_file_name(&self) -> Option<&str> {
        self.as_ref().file_name().and_then(OsStr::to_str)
    }

    fn is_vanilla_package(&self) -> bool {
        let Some(file_name) = self.get_file_name() else {
            return false;
        };

        constants::KF_DEFAULT_PACKAGES.contains(file_name.to_lowercase().as_str())
    }

    fn is_default_kf_extension(&self) -> bool {
        self.as_ref()
            .extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| constants::DEFAULT_EXTENSIONS.contains(&extension))
    }

    fn has_uz2_extension(&self) -> bool {
        self.as_ref()
            .extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension.to_lowercase() == constants::COMPRESSED_EXTENSION)
    }

    fn open_output_ue_stream(&self) -> Result<BufWriter<File>, io::Error> {
        Ok(BufWriter::new(File::create(self)?))
    }

    fn open_input_ue_stream(&self) -> Result<BufReader<File>, io::Error> {
        Ok(BufReader::new(File::open(self)?))
    }

    fn open_input_ue_stream_with_checks(&self) -> Result<BufReader<File>, UZ2LibErrors> {
        let mut reader = BufReader::new(File::open(self)?);

        match reader.file_header_is_correct() {
            Ok(()) => Ok(reader),
            Err(_) => Err(UZ2LibErrors::InvalidPackage(self.as_ref().to_path_buf())),
        }
    }
}

pub trait FileCheck {
    /// Check if this file is a valid UE package.
    /// # Errors
    ///
    /// Will return `Err` if fail to read / rewind or signature doesn't match.
    fn file_header_is_correct(&mut self) -> Result<(), UZ2LibErrors>;
}

impl<T: Read + Seek> FileCheck for T {
    fn file_header_is_correct(&mut self) -> Result<(), UZ2LibErrors> {
        let mut buf_file_header: Vec<u8> = vec![0u8; 4];
        self.read_exact(&mut buf_file_header)?;
        self.rewind()?;

        if buf_file_header == constants::KF_SIGNATURE {
            Ok(())
        } else {
            Err(UZ2LibErrors::InvalidFileHeader)
        }
    }
}

/// Validate path before compression attempt.
/// # Errors
///
/// Will return `Err` if one of checks fail.
pub fn validate_compressible_paths(
    input_arguments: &mut InputArguments,
) -> Result<(), UZ2LibErrors> {
    // input is a directory
    if !input_arguments.input_path.is_file() {
        return Err(UZ2LibErrors::FileDoesntExist(
            input_arguments.input_path.clone(),
        ));
    }
    // input has `uz2` extension
    if input_arguments.input_path.has_uz2_extension() {
        return Err(UZ2LibErrors::FileAlreadyCompressed(
            input_arguments.input_path.clone(),
        ));
    }
    // ignore core kf1 files or not
    if input_arguments.ignore_kf_files {
        if !input_arguments.input_path.is_default_kf_extension() {
            return Err(UZ2LibErrors::NotKFExtension(
                input_arguments.input_path.clone(),
            ));
        }
        if input_arguments.input_path.is_vanilla_package() {
            return Err(UZ2LibErrors::IsKFPackage(
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
                UZ2LibErrors::CreateDirError(e, input_arguments.output_path.clone())
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
            return Err(UZ2LibErrors::FileNameError(
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
pub fn validate_decompressible_paths(
    input_arguments: &mut InputArguments,
) -> Result<(), UZ2LibErrors> {
    // input is a directory
    if !input_arguments.input_path.is_file() {
        return Err(UZ2LibErrors::FileDoesntExist(
            input_arguments.input_path.clone(),
        ));
    }
    // input has `uz2` extension
    if !input_arguments.input_path.has_uz2_extension() {
        return Err(UZ2LibErrors::FileAlreadyDecompressed(
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
                UZ2LibErrors::CreateDirError(e, input_arguments.output_path.clone())
            })?;
        }

        if let Some(input_file_name) = input_arguments.input_path.get_file_name() {
            input_arguments.output_path = input_arguments.output_path.join(input_file_name);
            input_arguments.output_path.set_extension("");
        } else {
            return Err(UZ2LibErrors::FileNameError(
                input_arguments.output_path.clone(),
            ));
        }
    }

    Ok(())
}
