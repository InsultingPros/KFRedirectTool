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
