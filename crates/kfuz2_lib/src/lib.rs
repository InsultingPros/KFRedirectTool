// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#![doc = include_str!("../README.md")]

pub mod compressor;
pub mod constants;
pub mod decompressor;
pub mod errors;
pub mod validator;

/// `kfuz2_lib` log levels.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Verbose,
    #[default]
    Default,
    Minimal,
}

pub struct ProcessingResult {
    pub time: std::time::Duration,
    pub chunk_count: u32,
    pub hasher: Option<sha1_smol::Sha1>,
    pub input_file_size: u64,
    pub output_file_size: u64,
}
//todo add display / print

/// Arguments for internal use.
#[derive(Debug, Default, Clone)]
pub struct InputArguments {
    /// input file's path
    pub input_path: std::path::PathBuf,
    /// output file's path
    pub output_path: std::path::PathBuf,
    /// how much to print
    pub log_level: LogLevel,
    /// ignore KF1 vanilla files
    pub ignore_kf_files: bool,
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
#[allow(clippy::cast_precision_loss)]
pub fn print_additional_information(info: &ProcessingResult) {
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
