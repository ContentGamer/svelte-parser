use std::{env, fs, process::Command};

use crate::config::NODEJS_PLATFORM;

/**
 * Run the frontend to generate static files.
 */
pub fn run() {
    // cd build && bun src/generate.ts && bun run build && bun src/html.ts

    let root_dir = env!("CARGO_MANIFEST_DIR");

    if (!fs::metadata(format!("{}/build/node_modules", root_dir))
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false))
    {
        let _ = match cfg!(windows) {
            true => Command::new("cmd")
                .arg("/C")
                .arg(format!("cd {}/build && {} i", root_dir, unsafe {
                    NODEJS_PLATFORM
                }))
                .status(),
            false => Command::new("sh")
                .arg("-c")
                .arg(format!("cd {}/build && {} i", root_dir, unsafe {
                    NODEJS_PLATFORM
                }))
                .status(),
        };
    }

    let target: String = format!(
        "cd {}/build && {} src/generate.ts && {} run build && {} src/html.ts",
        root_dir,
        unsafe { NODEJS_PLATFORM },
        unsafe { NODEJS_PLATFORM },
        unsafe { NODEJS_PLATFORM }
    );
    let command = if cfg!(windows) {
        Command::new("cmd").arg("/C").arg(target).status()
    } else {
        Command::new("sh").arg("-c").arg(target).status()
    };

    match command {
        Ok(_) => {}
        Err(err) => println!("ERR at copier: {}", err),
    }
}

/**
 * Clean the directories before running the application again (keep node_modules)
 */
pub fn clean() {
    let root_dir = env!("CARGO_MANIFEST_DIR");

    let to_clean: Vec<String> = vec![
        "dist".to_string(),
        format!("{}/build/dist", root_dir),
        format!("{}/build/src/pages", root_dir),
        format!("{}/build/src/svelte", root_dir),
    ];

    for clean in to_clean {
        let entries = fs::read_dir(&clean);
        if (entries.is_err()) {
            continue;
        }
        for entry in entries.unwrap() {
            let entry = entry.unwrap();

            let mut path = std::path::PathBuf::from(clean.as_str());
            path.push(entry.file_name());

            fs::remove_file(path).unwrap();
        }
        if (clean.ends_with("/src/pages") || clean.ends_with("/src/svelte")) {
            fs::File::create_new(format!("{}/SKIP", clean)).unwrap();
        }
    }
}

/**
 * Copy the a target directory into a destination path.
 */
pub fn copy(target: String, destination: String) {
    let entries = fs::read_dir(&target).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let source_path = entry.path();

        if (entry.file_name() == "SKIP") {
            continue;
        }

        let mut destination_path = std::path::PathBuf::from(destination.as_str());
        destination_path.push(entry.file_name());

        fs::copy(&source_path, &destination_path).unwrap();
    }
}
