# Svelte Parser

a Svelte Parser for the `expressrs` crate.

## How can i use this?

First you'll need to install the `expressrs` crate
then in your code type this:

```rust

use expressrs::ExpressLib;
use svelte_parser::svelte;

fn main() {
	let express = ExpressLib::new();
	let app = express();

	app.plugins.push(svelte());
	// Serve a directory called 'public' that has .svelte files
	app.serve_directory(
        "public",
        Some(DirectoryOptions {
            plugin: "svelte".to_string(),
        }),
    );
}

```