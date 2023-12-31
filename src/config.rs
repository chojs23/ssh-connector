use std::{
    fs::{self, File},
    process,
};

use homedir::get_my_home;
use inquire::InquireError;
use serde::{Deserialize, Serialize};

use crate::renderer::{get_input, menu};

const CONFIG_ITEMS: &[&str] = &[
    "Add connection",
    "Edit connection",
    "Remove connection",
    "Back",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub user: String,
    pub host: String,
    pub port: u16,
    pub key_path: Option<String>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            user: "".to_string(),
            host: "".to_string(),
            port: 22,
            key_path: None,
        }
    }
}

pub fn get_config_path() -> std::path::PathBuf {
    let home = get_my_home().unwrap().unwrap();
    let home = home.as_path();

    let ssh_dir = home.join(".ssh-connector/");
    if !ssh_dir.exists() {
        println!("Creating .ssh-connector directory at {:?}", home);
        fs::create_dir(&ssh_dir).expect("Failed to create .ssh-connector directory");
    }

    ssh_dir.join("config.json")
}

pub fn configure() -> anyhow::Result<(), anyhow::Error> {
    match menu("Configuration", CONFIG_ITEMS) {
        Ok("Add connection") => match add_connection() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Ok("Edit connection") => match edit_connection() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Ok("Remove connection") => match remove_connection() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Ok("Back") => {
            println!("Back...");
            Ok(())
        }
        Err(InquireError::OperationCanceled) => Ok(()),
        Err(InquireError::OperationInterrupted) => {
            process::exit(1);
        }
        _ => {
            println!("Unknown option");
            process::exit(1);
        }
    }
}

pub fn get_config_list() -> anyhow::Result<Vec<ConnectionConfig>, anyhow::Error> {
    let config_file = if let Ok(file) = fs::OpenOptions::new().read(true).open(get_config_path()) {
        serde_json::from_reader::<File, Vec<ConnectionConfig>>(file)?
    } else {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(get_config_path())
            .expect("Failed to create config file");
        println!("Empty config file created");

        serde_json::to_writer_pretty(file, &Vec::<ConnectionConfig>::new())?;

        Vec::<ConnectionConfig>::new()
    };

    Ok(config_file)
}

pub fn show_configs() -> anyhow::Result<(), anyhow::Error> {
    let config_file = get_config_list()?;

    for (idx, config) in config_file.iter().enumerate() {
        println!("{}", idx + 1);
        println!("Username: {}", config.user);
        println!("Hostname or IP address: {}", config.host);
        println!("Port: {}", config.port);
        println!(
            "Key Path: {}",
            config.key_path.as_ref().unwrap_or(&"".to_string())
        );
        println!();
    }

    Ok(())
}

pub fn add_connection() -> anyhow::Result<(), anyhow::Error> {
    let config = get_input(None)?;
    println!("Config: {:?}", config);

    let mut config_file = get_config_list()?;

    config_file.push(config);

    write_config(config_file)?;

    Ok(())
}

fn write_config(config_file: Vec<ConnectionConfig>) -> anyhow::Result<(), anyhow::Error> {
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_config_path())
        .unwrap();

    serde_json::to_writer_pretty(file, &config_file)?;

    Ok(())
}

pub fn select_connection() -> anyhow::Result<ConnectionConfig, anyhow::Error> {
    let config_file = get_config_list()?;

    if config_file.is_empty() {
        println!("No config found");
        return Err(anyhow::Error::msg("No config found"));
    }

    let selections = config_file
        .iter()
        .enumerate()
        .map(|(idx, config)| (format!("{}@{}", config.user, config.host), idx))
        .collect::<std::collections::HashMap<String, usize>>();

    let options = config_file
        .iter()
        .map(|config| format!("{}@{}", config.user, config.host))
        .collect::<Vec<String>>();

    let options = options
        .iter()
        .map(|opt| opt.as_str())
        .collect::<Vec<&str>>();

    let selected = menu("Select connection", &options)?;

    let idx = selections.get(selected).unwrap();

    Ok(config_file[*idx].clone())
}

pub fn edit_connection() -> anyhow::Result<(), anyhow::Error> {
    let mut config_file = get_config_list()?;

    if config_file.is_empty() {
        println!("No config found");
        return Ok(());
    }

    let selections = config_file
        .iter()
        .enumerate()
        .map(|(idx, config)| (format!("{}@{}", config.user, config.host), idx))
        .collect::<std::collections::HashMap<String, usize>>();

    let options = config_file
        .iter()
        .map(|config| format!("{}@{}", config.user, config.host))
        .collect::<Vec<String>>();

    let options = options
        .iter()
        .map(|opt| opt.as_str())
        .collect::<Vec<&str>>();

    let selected = menu("Select connection to edit", &options)?;

    let idx = selections.get(selected).unwrap();

    let original = &config_file[*idx];

    let config = get_input(Some(original.clone()))?;

    config_file[*idx] = config;

    write_config(config_file)?;

    Ok(())
}

pub fn remove_connection() -> anyhow::Result<(), anyhow::Error> {
    let mut config_file = get_config_list().unwrap();

    if config_file.is_empty() {
        println!("No config found");
        return Ok(());
    }

    let selections = config_file
        .iter()
        .enumerate()
        .map(|(idx, config)| (format!("{}@{}", config.user, config.host), idx))
        .collect::<std::collections::HashMap<String, usize>>();

    let options = config_file
        .iter()
        .map(|config| format!("{}@{}", config.user, config.host))
        .collect::<Vec<String>>();

    let options = options
        .iter()
        .map(|opt| opt.as_str())
        .collect::<Vec<&str>>();

    let selected = menu("Select connection to remove", &options).unwrap();

    let idx = selections.get(selected).unwrap();

    config_file.remove(*idx);

    write_config(config_file)?;

    println!("Connection removed");

    Ok(())
}
