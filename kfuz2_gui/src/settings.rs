use std::{fs::File, path::PathBuf};

const CONFIG_FILE_NAME: &str = "kfuz2_gui.toml";

pub fn get_config_content() -> Result<[String; 1], ()> {
    Ok(["test".to_string()])
}

fn config_exists() -> bool {
    PathBuf::from(CONFIG_FILE_NAME).exists()
}

fn create_config() -> Result<File, std::io::Error> {
    File::create(CONFIG_FILE_NAME)
}

// #[cfg(test)]
// mod test_config {
//     use super::*;
//     use serial_test::serial;
//     use std::path::PathBuf;

//     const EXECUTABLE: &str = "..//target//debug//kfuz2_gui";

//     #[test]
//     #[serial]
//     fn config_exists() {
//         let config_path = PathBuf::from(EXECUTABLE)
//             .parent()
//             .unwrap()
//             .join(CONFIG_FILE_NAME);

//         assert!(config_path.exists());
//     }
// }
