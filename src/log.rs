use std::{
    io::{BufRead, Write},
    sync::{Arc, Mutex},
};

use bevy::{
    app::{App, Update},
    log::tracing_subscriber::{self, EnvFilter, Layer, Registry},
    prelude::{EventWriter, IntoSystemConfigs, ResMut, Resource},
};

use crate::{ConsoleSet, PrintConsoleLine};

/// Buffers logs written by bevy at runtime
#[derive(Resource)]
pub struct BevyLogBuffer(Arc<Mutex<std::io::Cursor<Vec<u8>>>>);

/// Writer implementation which writes into a buffer resource inside the bevy world
pub struct BevyLogBufferWriter(Arc<Mutex<std::io::Cursor<Vec<u8>>>>);

impl Write for BevyLogBufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let lock = self.0.upgrade().unwrap();
        let mut lock = self.0.lock().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock buffer: {}", e),
            )
        })?;
        lock.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // let lock = self.0.upgrade().unwrap();
        let mut lock = self.0.lock().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock buffer: {}", e),
            )
        })?;
        lock.flush()
    }
}

/// Flushes the log buffer and sends its content to the console
pub fn send_log_buffer_to_console(
    buffer: ResMut<BevyLogBuffer>,
    mut console_lines: EventWriter<PrintConsoleLine>,
) {
    let mut buffer = buffer.0.lock().unwrap();
    // read and clean buffer
    let buffer = buffer.get_mut();
    for line in buffer.lines().map_while(Result::ok) {
        console_lines.send(PrintConsoleLine { line });
    }
    buffer.clear();
}

/// Creates a tracing layer which writes logs into a buffer resource inside the bevy world
/// This is used by the console plugin to capture logs written by bevy
/// Use [make_filtered_layer] for more customization options.
pub fn make_layer(
    app: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>>
{
    setup_layer(app, None)
}

/// Creates a tracing layer which writes logs into a buffer resource inside the bevy world
/// Uses a custom [EnvFilter] string, allowing for a different subset of log entries to be
/// captured by the console.
/// This is used by the console plugin to capture logs written by bevy
///
/// ## Example
/// ```ignore
/// DefaultPlugins.set(LogPlugin {
///    filter: log::DEFAULT_FILTER.to_string(),
///    level: log::Level::INFO,
///    custom_layer: |app: &mut App| make_filtered_layer(app,
///        "mygame=info,warn,debug,error".to_string())
///})
///```
pub fn make_filtered_layer(
    app: &mut App,
    filter: String,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>>
{
    let env_filter = EnvFilter::builder().parse_lossy(filter);
    setup_layer(app, Some(env_filter))
}

/// Performs common layer setup
fn setup_layer(
    app: &mut App,
    filter: Option<EnvFilter>,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>>
{
    let buffer = Arc::new(Mutex::new(std::io::Cursor::new(Vec::new())));
    app.insert_resource(BevyLogBuffer(buffer.clone()));
    app.add_systems(
        Update,
        send_log_buffer_to_console.in_set(ConsoleSet::PostCommands),
    );

    let layer: Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>;
    layer = if let Some(filter) = filter {
        // Layer::with_filter() returns a different impl, thus the split
        Box::new(tracing_subscriber::fmt::Layer::new()
            .with_target(false)
            .with_ansi(true)
            .with_writer(move || BevyLogBufferWriter(buffer.clone()))
            .with_filter(filter))
    } else {
        Box::new(tracing_subscriber::fmt::Layer::new()
            .with_target(false)
            .with_ansi(true)
            .with_writer(move || BevyLogBufferWriter(buffer.clone())))
    };

    Some(layer)
}
