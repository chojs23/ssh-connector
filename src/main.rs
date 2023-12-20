mod config;
mod errors;
mod renderer;
mod ssh;

use config::*;
use inquire::InquireError;
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
        match menu("Select menu", ITEMS) {
            Ok("Show configs") => {
                println!("---- Config list ----");
                if let Err(err) = show_configs() {
                    println!("Error: {}", err);
                }
            }
            Ok("Ping server") => {
                let config = select_connection()?;
                println!("Ping server...");
                ssh::ping_server(config);
            }
            Ok("Ssh into server") => {
                let config = select_connection()?;
                reset!("Ssh into server...");
                ssh::ssh_connect(config);
            }
            Ok("Configure") => {
                println!("Configure...");
                if let Err(err) = configure() {
                    println!("!!Error: {}", err);
                }
            }
            Ok("Quit") => break,
            Ok(_) => {
                println!("Unknown option");
                break;
            }
            Err(InquireError::OperationCanceled) => break,
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
