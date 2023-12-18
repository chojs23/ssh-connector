mod config;
mod errors;
mod renderer;
mod ssh;

use renderer::*;

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
