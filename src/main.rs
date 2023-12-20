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

fn main() {
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
                println!("Ping server...");
                ssh::ping_server();
            }
            "Ssh into server" => {
                reset!("Ssh into server...");
                ssh::ssh_connect();
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
}
