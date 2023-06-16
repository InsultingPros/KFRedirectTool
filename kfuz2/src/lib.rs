// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

pub mod cli;
pub mod compressor;
pub mod constants;
pub mod decompressor;
pub mod utility;

/// Application states.
pub enum State {
    Compression,
    Decompression,
}

pub struct InputArguments<'a> {
    pub input_path: std::path::PathBuf,
    pub output_path: std::path::PathBuf,
    /// Input file argument, only used to cast `input_path`
    pub input_file_str: String,
    /// Output directory argument, only used to cast `output_path`
    pub output_file_str: Option<String>,
    pub app_state: &'a State,
    pub verbose: bool,
    pub nocheck: bool,
}
