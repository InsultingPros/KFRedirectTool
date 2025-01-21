// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

/// `kfuz2_lib` log levels.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Verbose,
    #[default]
    Default,
    Minimal,
}

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

pub struct ProcessingResult {
    pub time: std::time::Duration,
    pub chunk_count: u32,
    pub hasher: Option<sha1_smol::Sha1>,
    pub input_file_size: u64,
    pub output_file_size: u64,
}
