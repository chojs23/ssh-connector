use std::fs::{self, File};

use serde::{Deserialize, Serialize};

use crate::renderer::get_input;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub user: String,
    pub host: String,
    pub addr: String,
    pub port: u16,
    pub key_path: Option<String>,
}

pub fn get_config_list() -> anyhow::Result<Vec<ConnectionConfig>, anyhow::Error> {
    let config_file = if let Ok(file) = fs::OpenOptions::new().read(true).open("config.json") {
        serde_json::from_reader::<File, Vec<ConnectionConfig>>(file)?
    } else {
        let _ = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("config.json")?;
        vec![]
    };

    Ok(config_file)
}

pub fn add_connection() -> anyhow::Result<(), anyhow::Error> {
    let config = get_input()?;
    println!("Config: {:?}", config);

    let mut config_file = get_config_list()?;

    config_file.push(config);

    let file = fs::OpenOptions::new()
        .write(true)
        .open("config.json")
        .unwrap();

    serde_json::to_writer_pretty(file, &config_file)?;

    Ok(())
}
