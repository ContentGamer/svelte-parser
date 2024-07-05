use std::process::{Command, Stdio};

use crate::{config::NODEJS_PLATFORM, helpers::logger::CRITICAL};

/**
 * Check if nodejs was installed to the current computer.
 */
pub fn check() {
    let commands: Vec<&str> = vec!["npm", "yarn", "pnpm", "bun"];

    for command in commands {
        let status = Command::new(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(status) => {
                if (status.success()) {
                    unsafe {
                        NODEJS_PLATFORM = command;
                    }
                    return;
                } else {
                    continue;
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    CRITICAL("NodeJS was not installed in your computer, please install it to continue.");
}
