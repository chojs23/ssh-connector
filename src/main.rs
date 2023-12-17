mod config;
mod ssh;

use config::get_render_config;
use inquire::Select;

macro_rules! reset {
    ($s:expr) => {
        println!("\x1b[2J\x1b[1;1H{}", $s)
    };
}

fn menu<'a>(items: &[&'a str]) -> &'a str {
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

const ITEMS: &[&str] = &["Ping server", "Ssh into server", "Configure", "Quit"];

fn main() {
    inquire::set_global_render_config(get_render_config());
    loop {
        match menu(ITEMS) {
            "Ping server" => {
                println!("Ping server...");
                ssh::ping_server();
            }
            "Ssh into server" => {
                reset!("Ssh into server...");
                ssh::ssh_connect();
            }
            "Configure" => println!("Configuring..."),
            "Quit" => break,
            err => println!("Unknown option: {}", err),
        }
    }
}
