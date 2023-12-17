use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, StyleSheet};

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightGreen);

    render_config.help_message = StyleSheet::new().with_fg(Color::LightYellow);

    render_config.error_message = ErrorMessageRenderConfig::default_colored();

    render_config
}
