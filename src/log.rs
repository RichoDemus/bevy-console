use std::{
    io::{BufRead, Write},
    sync::{Arc, Mutex},
};

use bevy::{
    app::App,
    log::tracing_subscriber::{self, Registry},
    prelude::{EventWriter, ResMut, Resource},
};

use crate::PrintConsoleLine;

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
pub fn make_layer(
    app: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> {
    let buffer = Arc::new(Mutex::new(std::io::Cursor::new(Vec::new())));
    app.insert_resource(BevyLogBuffer(buffer.clone()));

    Some(Box::new(
        tracing_subscriber::fmt::Layer::new()
            .with_target(false)
            .with_ansi(true)
            .with_writer(move || BevyLogBufferWriter(buffer.clone())),
    ))
}
