/// Define application exit codes, specific to each platforms
///
/// Reference: <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499->
#[cfg(target_family = "windows")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    pub const ERROR_CANNOT_MAKE: u8 = 82;
    pub const ERROR_BAD_ARGUMENTS: u8 = 160;
}

/// Define application exit codes, specific to each platform
///
/// Reference: <https://unix.stackexchange.com/a/254747>
#[cfg(target_family = "unix")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    pub const ERROR_CANNOT_MAKE: u8 = 1;
    pub const ERROR_BAD_ARGUMENTS: u8 = 128;
}
