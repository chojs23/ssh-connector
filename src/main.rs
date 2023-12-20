mod config;
mod errors;
mod renderer;
mod ssh;

use config::*;
use renderer::*;

const ITEMS: &[&str] = &[
    "Show configs",
    "Ping server",
    "Ssh into server",
    "Configure",
    "Quit",
];

fn main() -> anyhow::Result<(), anyhow::Error> {
    inquire::set_global_render_config(get_render_config());
    loop {
        match menu(ITEMS) {
            "Show configs" => {
                println!("---- Config list ----");
                if let Err(err) = show_configs() {
                    println!("Error: {}", err);
                }
            }
            "Ping server" => {
                let config = select_connection()?;
                println!("Ping server...");
                ssh::ping_server(config);
            }
            "Ssh into server" => {
                let config = select_connection()?;
                reset!("Ssh into server...");
                ssh::ssh_connect(config);
            }
            "Configure" => {
                println!("Configure...");
                if let Err(err) = configure() {
                    println!("Error: {}", err);
                }
            }
            "Back" => break,
            "Quit" => break,
            err => println!("Unknown option: {}", err),
        }
    }

    Ok(())
}
