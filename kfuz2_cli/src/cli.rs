// Author       : Shtoyan, dkanus
// Home Repo    : https://github.com/InsultingPros/KFRedirectTool
// License      : https://www.gnu.org/licenses/gpl-3.0.en.html

#[allow(clippy::struct_excessive_bools)]
// Reference: https://docs.rs/gumdrop/latest/gumdrop/
/// kfuz2_cli supported arguments. For online help check: <https://github.com/InsultingPros/KFRedirectTool>
#[derive(Debug, gumdrop::Options)]
pub struct Options {
    /// `-h` : print help information.
    #[options(help = "Prints the help message.")]
    pub help: bool,
    /// `-v` / `--verbose`: This option enables detailed operation, displaying extensive information during file processing.
    ///
    /// If both this and the `-q` option are active, the quiet mode will take precedence, suppressing the verbose output.
    #[options(
        short = "v",
        help = "This option enables detailed operation, displaying extensive information during file processing.
                            If both this and the `-q` option are active, the quiet mode will take precedence, suppressing the verbose output."
    )]
    pub verbose: bool,
    /// `-q` / `--quiet`: This option ensures silent operation, providing no feedback or information during file processing.
    ///
    /// Activating this mode will override `-v` option.
    #[options(
        short = "q",
        help = "This option ensures silent operation, providing no feedback or information during file processing.
                            Activating this mode will override `-v` option."
    )]
    pub quiet: bool,
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
        help = "Decompresses the `input_file`. If not used, the input file will be compressed."
    )]
    pub decompress: Option<String>,
    /// `--nocheck` : Disables the additional check for verifying if the input file matches KF1's format or belongs to one of its built-in packages.
    // not allowing short variant, so users won't mix it with compression case
    #[options(
        no_short,
        help = "Disables the additional check for verifying if the input file matches KF1's format or belongs to one of its built-in packages."
    )]
    pub nocheck: bool,
    /// File to compress (or decompress when '-d' option is specified).
    #[options(free)]
    pub input_file: Vec<String>,
}
