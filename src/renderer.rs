use anyhow::Context;
use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, StyleSheet};
use inquire::{InquireError, Select, Text};

use crate::config::ConnectionConfig;
use crate::errors::AppError;

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

pub fn get_input(
    original: Option<ConnectionConfig>,
) -> anyhow::Result<ConnectionConfig, anyhow::Error> {
    let initial = original.unwrap_or_default();

    let user = Text::new("Username:")
        .with_default(&initial.user)
        .prompt()
        .context(AppError::ConfigError("Invalid username".to_string()))?;

    let host = Text::new("Hostname or IP address:")
        .with_default(&initial.host)
        .prompt()
        .context(AppError::ConfigError("Invalid hostname".to_string()))?;

    let port = Text::new("Port:")
        .with_default(&initial.port.to_string())
        .prompt()?
        .parse()
        .context(AppError::ConfigError("Invalid port".to_string()))?;

    let key_path = Text::new("(Optional)Path to key - Enter to skip:")
        .with_default(&initial.key_path.unwrap_or_default().to_string())
        .prompt()
        .context(AppError::ConfigError("Invalid path".to_string()))?;

    let config = ConnectionConfig {
        user,
        host,
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

pub fn menu<'a>(message: &str, items: &[&'a str]) -> Result<&'a str, InquireError> {
    let selection = Select::new(message, items.to_vec())
        .with_vim_mode(true)
        .with_help_message(
            "Vim keymap is enabled. Use j/k to move up/down, <Enter> to select, <Esc> to quit.",
        )
        .prompt()?;

    Ok(selection)
}
