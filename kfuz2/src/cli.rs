// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

// Reference: https://docs.rs/gumdrop/latest/gumdrop/
/// kfuz2 supported arguments. For online help check: <https://github.com/InsultingPros/KFRedirectTool>
#[derive(Debug, gumdrop::Options)]
pub struct Options {
    /// `-h` : print help information.
    #[options(help = "Prints the help message.")]
    pub help: bool,
    /// `-v` : print lots of additional information during file processing.
    #[options(
        short = "v",
        help = "Displays additional information during file processing."
    )]
    pub verbose: bool,
    /// `-o <dir>` : output directory for processed files.
    ///
    /// If omitted, processed files will be saved in the same directory as input file.
    #[options(
        short = "o",
        meta = "<directory>",
        help = "Specifies the target directory. If not provided, processed files will be saved in the same directory as the input file."
    )]
    pub output: Option<String>,
    /// `-d <file>`: decompress input file.
    #[options(
        short = "d",
        meta = "<file>",
        help = "Decompresses the input file. If not used, the input file will be compressed."
    )]
    pub decompress: Option<String>,
    /// `--nocheck` : Disables the additional check for verifying if the input file matches KF1's format or belongs to one of its built-in packages.
    // not allowing short variant, so users won't mix it with compression case
    #[options(
        no_short,
        help = "Disables the additional check for verifying if the input file matches KF1's format or belongs to one of its built-in packages."
    )]
    pub nocheck: bool,
    /// Not argumented input (`<file>`) will be used for compression.
    #[options(free)]
    pub free: Vec<String>,
}
