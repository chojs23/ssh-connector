use anyhow::Context;
use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, StyleSheet};
use inquire::{Select, Text};

use crate::config::{add_connection, ConnectionConfig};
use crate::errors::AppError;

const CONFIG_ITEMS: &[&str] = &[
    "Add connection",
    "Edit connection",
    "Remove connection",
    "Back",
];

#[macro_export]
macro_rules! reset {
    ($s:expr) => {
        println!("\x1b[2J\x1b[1;1H{}", $s)
    };
}

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightGreen);

    render_config.help_message = StyleSheet::new().with_fg(Color::LightYellow);

    render_config.error_message = ErrorMessageRenderConfig::default_colored();

    render_config
}

pub fn get_input() -> anyhow::Result<ConnectionConfig, anyhow::Error> {
    let user = Text::new("Username:")
        .prompt()
        .context(AppError::ConfigError("Invalid username".to_string()))?;

    let host = Text::new("Hostname:")
        .prompt()
        .context(AppError::ConfigError("Invalid hostname".to_string()))?;

    let addr = Text::new("IP Address of host:")
        .prompt()
        .context(AppError::ConfigError("Invalid address".to_string()))?;

    let port = Text::new("Port:")
        .prompt()?
        .parse()
        .context(AppError::ConfigError("Invalid port".to_string()))?;

    let key_path = Text::new("(Optional)Path to key - Enter to skip:")
        .prompt()
        .context(AppError::ConfigError("Invalid path".to_string()))?;

    let config = ConnectionConfig {
        user,
        host,
        addr,
        port,
        key_path: {
            if key_path.trim().is_empty() {
                None
            } else {
                Some(key_path)
            }
        },
    };

    Ok(config)
}

pub fn menu<'a>(items: &[&'a str]) -> &'a str {
    Select::new("Select what to do", items.to_vec())
        .with_vim_mode(true)
        .with_help_message(
            "Vim keymap is enabled. Use j/k to move up/down, <Enter> to select, <Esc> to quit.",
        )
        .prompt()
        .unwrap_or_else(|err| match err {
            inquire::error::InquireError::OperationCanceled => "Back",
            inquire::error::InquireError::OperationInterrupted => "Quit",
            _ => {
                println!("Error: {}", err);
                "Quit"
            }
        })
}

pub fn configure() -> anyhow::Result<(), anyhow::Error> {
    match menu(CONFIG_ITEMS) {
        "Add connection" => match add_connection() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        "Edit connection" => {
            println!("Edit connection...");
            Ok(())
        }
        "Remove connection" => {
            println!("Remove connection...");
            Ok(())
        }
        "Back" => {
            println!("Back...");
            Ok(())
        }
        "Quit" => {
            println!("Quit...");
            Ok(())
        }
        _ => {
            println!("Unknown option");
            Ok(())
        }
    }
}
