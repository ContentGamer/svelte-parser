# Svelte Parser

a Svelte Parser for the `expressrs` crate.

## Source code

This is a source code to get you started into making your own `expressrs` plugin, If you want to start making a plugin, clone this source code and make sure you have rust installed, then navigate to `src/constants.rs` so you can change the Name, ID (should be lowercase), and Plugin-ID into your perspective choice.

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
