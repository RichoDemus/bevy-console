# bevy_console
[![Check](https://github.com/RichoDemus/bevy-console/actions/workflows/build.yaml/badge.svg)](https://github.com/RichoDemus/bevy-console/actions/workflows/build.yaml)

A simple *Half-Life* inspired console with support for argument parsing powered by [`clap`](https://docs.rs/clap/latest/clap/).

<p align="center">
  <img src="https://raw.githubusercontent.com/richodemus/bevy-console/main/doc/screenshot.png" width="100%">
</p>

## Usage

Add `ConsolePlugin` and optionally the resource `ConsoleConfiguration`.

```rust, ignore
use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .insert_resource(ConsoleConfiguration {
            // override config here
            ..Default::default()
        });
}
```

Create a console command struct and system and add it to your app with `.add_console_command`.
 Commands are created like `clap` commands with an additional `CommandName` trait derived via the `ConsoleCommand` derive.

Add [doc comments](https://doc.rust-lang.org/rust-by-example/meta/doc.html#doc-comments) to your command to provide help information in the console.

```rust, ignore
use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_console_command::<ExampleCommand, _>(example_command);
}

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
    /// Some message
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        // handle command
    }
}
```

Examples can be found in the [/examples](examples) directory.

```bash
cargo run --example log_command
```

- [log_command](/examples/log_command.rs)
- [raw_commands](/examples/raw_commands.rs)
- [write_to_console](/examples/write_to_console.rs)
- [change_console_key](/examples/change_console_key.rs)

## wasm

Should work in wasm, but you need to disable default features.
