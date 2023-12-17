use std::process;

use nix::libc::{fork, wait};

pub fn ssh_connect() {
    unsafe {
        let pid = fork();

        match pid {
            0 => {
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
                wait(&mut 0 as *mut i32);
                println!("Child process finished");
            }
        }
    }
}
