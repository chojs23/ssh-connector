use std::process;

use nix::{
    sys::wait,
    unistd::{fork, ForkResult},
};

use crate::config::ConnectionConfig;

pub fn ssh_connect(config: ConnectionConfig) {
    unsafe {
        let pid = fork();

        match pid {
            Ok(ForkResult::Child) => {
                let _err = exec::Command::new("ssh")
                    .arg("-i")
                    .arg(config.key_path.unwrap_or_default())
                    .arg("-p")
                    .arg(config.port.to_string())
                    .arg(format!("{}@{}", config.user, config.host))
                    .exec();
                process::exit(1);
            }
            _ => {
                let _ = wait::wait();
            }
        }
    }
}

pub fn ping_server(config: ConnectionConfig) {
    unsafe {
        let pid = fork();

        match pid {
            Ok(ForkResult::Child) => {
                let err = exec::Command::new("ping")
                    .args(&["-c", "3"])
                    .arg(config.host)
                    .exec();
                println!("Error: {}", err);
                process::exit(1);
            }
            _ => {
                let _ = wait::wait();
            }
        }
    }
}
