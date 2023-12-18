use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, StyleSheet};
use inquire::Select;

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

pub fn menu<'a>(items: &[&'a str]) -> &'a str {
    Select::new("Select what to do", items.to_vec())
        .with_vim_mode(true)
        .with_help_message(
            "Vim keymap is enabled. Use j/k to move up/down, <Enter> to select, <Esc> to quit.",
        )
        .prompt()
        .unwrap_or_else(|err| {
            println!("Error: {}", err);
            "Quit"
        })
}
