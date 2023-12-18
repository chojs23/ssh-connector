use std::process;

use nix::{
    sys::wait,
    unistd::{fork, ForkResult},
};

pub fn ssh_connect() {
    unsafe {
        let pid = fork();

        match pid {
            Ok(ForkResult::Child) => {
                println!("Excute child process");
                let err = exec::Command::new("ssh")
                    .arg("root@localhost")
                    .arg("-p 2200")
                    .exec();
                println!("Error: {}", err);
                process::exit(1);
            }
            _ => {
                println!("Parent process");
                let _ = wait::wait();
                println!("Child process finished");
            }
        }
    }
}

pub fn ping_server() {
    unsafe {
        let pid = fork();

        match pid {
            Ok(ForkResult::Child) => {
                println!("Excute child process");
                let err = exec::Command::new("ping")
                    .args(&["-c", "3", "localhost"])
                    .exec();
                println!("Error: {}", err);
                process::exit(1);
            }
            _ => {
                println!("Parent process");
                let _ = wait::wait();
                println!("Child process finished");
            }
        }
    }
}
