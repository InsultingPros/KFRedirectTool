#![allow(dead_code)]
/// debug executable
pub const EXE_DEBUG: &str = "..//target//debug//kfuz2_cli";
/// release executable
pub const EXE_RELEASE: &str = "..//target//release//kfuz2_cli";

/// Directory for `compress` output
pub const OUTPUT_COMPRESSED: &str = "test_files//compressed";
/// Directory for `decompress` output
pub const OUTPUT_DECOMPRESSED: &str = "test_files//decompressed";

/// `SHA1` hash for validation file
pub const SHA1_VALIDATION_FILE_U: &str = "e81de24a8d78e58c104dfffbd201da416e49218a";
/// File for validation, gathered from [here](https://github.com/InsultingPros/BitCore).
pub const VALIDATION_FILE_U: &str = "test_files//validation//BitCore.u";
/// `SHA1` hash for compressed validation file
pub const SHA1_VALIDATION_FILE_UZ2: &str = "ee5015514aa3f641017606521cce4a2994fbf065";
/// Processed file after `compression`, with `output` omitted
pub const VALIDATION_FILE_UZ2: &str = "test_files//validation//BitCore.u.uz2";

/// Processed file after `compression`
pub const FILE_COMPRESSED_OUTPUT: &str = "test_files//compressed//BitCore.u.uz2";
/// Processed file after `decompression`
pub const FILE_DECOMPRESSED_OUTPUT: &str = "test_files//decompressed//BitCore.u";

/// Incorrect file extension
pub const INCORRECT_FILE_EXT: &str = "test_files//incorrect//UCC.exe";
/// Correct extension, but incorrect package
pub const INCORRECT_FILE: &str = "test_files//incorrect//UCC.u";
/// Correct 'uz2' extension, but incorrect file
pub const INCORRECT_FILE_UZ2: &str = "test_files//incorrect//UCC.uz2";
/// Vanilla game packages must be omitted
pub const VANILLA_FILE: &str = "test_files//incorrect//KFMutators.u";
/// Empty file
pub const EMPTY_FILE: &str = "";
