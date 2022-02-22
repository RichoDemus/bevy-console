use std::marker::PhantomData;

use bevy::{
    app::{EventReaderState, EventWriterState, Events},
    ecs::system::{
        LocalState, ResMutState, ResState, Resource, SystemMeta, SystemParam, SystemParamFetch,
        SystemParamState,
    },
    input::keyboard::KeyboardInput,
    prelude::*,
};
use bevy_console_parser::{parse_console_command, ValueRawOwned};
use bevy_egui::{
    egui::{self, Align, ScrollArea, TextEdit},
    EguiContext,
};

use crate::FromValueError;

pub trait CommandName {
    fn command_name() -> &'static str;
}

pub trait CommandArgs: Sized {
    fn from_values(values: &[ValueRawOwned]) -> Result<Self, FromValueError>;
}

pub struct ConsoleCommand<'w, 's, T> {
    command: Option<T>,
    console_line: EventWriter<'w, 's, PrintConsoleLine>,
}

impl<'w, 's, T> ConsoleCommand<'w, 's, T> {
    pub fn single(&self) -> Option<&T> {
        self.command.as_ref()
    }

    pub fn into_single(self) -> Option<T> {
        self.command
    }

    pub fn reply(&mut self, msg: impl Into<String>) {
        self.console_line.send(PrintConsoleLine::new(msg.into()));
    }

    pub fn ok(&mut self) {
        self.console_line
            .send(PrintConsoleLine::new("[ok]".to_string()));
    }

    pub fn failed(&mut self) {
        self.console_line
            .send(PrintConsoleLine::new("[failed]".to_string()));
    }
}

pub struct ConsoleCommandState<T> {
    #[allow(clippy::type_complexity)]
    event_reader: EventReaderState<
        (
            LocalState<(usize, PhantomData<ConsoleCommandEntered>)>,
            ResState<Events<ConsoleCommandEntered>>,
        ),
        ConsoleCommandEntered,
    >,
    console_line: EventWriterState<(ResMutState<Events<PrintConsoleLine>>,), PrintConsoleLine>,
    marker: PhantomData<T>,
}

impl<'w, 's, T: Resource + CommandName + CommandArgs> SystemParam for ConsoleCommand<'w, 's, T> {
    type Fetch = ConsoleCommandState<T>;
}

unsafe impl<'w, 's, T: Resource> SystemParamState for ConsoleCommandState<T> {
    type Config = ();

    fn init(world: &mut World, system_meta: &mut SystemMeta, _config: Self::Config) -> Self {
        let event_reader = EventReaderState::<
            (
                <Local<'s, (usize, PhantomData<ConsoleCommandEntered>)> as SystemParam>::Fetch,
                <Res<'w, Events<ConsoleCommandEntered>> as SystemParam>::Fetch,
            ),
            ConsoleCommandEntered,
        >::init(world, system_meta, (None, ()));

        let console_line = EventWriterState::<
            (<ResMut<'w, Events<PrintConsoleLine>> as SystemParam>::Fetch,),
            PrintConsoleLine,
        >::init(world, system_meta, ((),));

        ConsoleCommandState {
            event_reader,
            console_line,
            marker: PhantomData::default(),
        }
    }

    fn default_config() {}
}

impl<'w, 's, T: Resource + CommandName + CommandArgs> SystemParamFetch<'w, 's>
    for ConsoleCommandState<T>
{
    type Item = ConsoleCommand<'w, 's, T>;

    #[inline]
    unsafe fn get_param(
        state: &'s mut Self,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item {
        let mut event_reader =
            EventReaderState::<
                (
                    <Local<'s, (usize, PhantomData<ConsoleCommandEntered>)> as SystemParam>::Fetch,
                    <Res<'w, Events<ConsoleCommandEntered>> as SystemParam>::Fetch,
                ),
                ConsoleCommandEntered,
            >::get_param(&mut state.event_reader, system_meta, world, change_tick);

        let mut console_line =
            EventWriterState::<
                (<ResMut<'w, Events<PrintConsoleLine>> as SystemParam>::Fetch,),
                PrintConsoleLine,
            >::get_param(&mut state.console_line, system_meta, world, change_tick);

        let command = event_reader
            .iter()
            .find(|cmd| cmd.command == T::command_name())
            .map(|cmd| T::from_values(&cmd.args))
            .and_then(|result| match result {
                Ok(value) => Some(value),
                Err(err) => {
                    console_line.send(PrintConsoleLine::new(err.to_string()));
                    None
                }
            });

        ConsoleCommand {
            command,
            console_line,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleCommandEntered {
    pub command: String,
    pub args: Vec<ValueRawOwned>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintConsoleLine {
    pub line: String,
}

impl PrintConsoleLine {
    pub const fn new(line: String) -> Self {
        Self { line }
    }
}

#[derive(Default)]
pub(crate) struct ConsoleState {
    buf: String,
    show: bool,
    scrollback: Vec<String>,
}

#[derive(Copy, Clone)]
pub enum ToggleConsoleKey {
    KeyCode(KeyCode),
    ScanCode(u32),
}

#[derive(Clone)]
pub struct ConsoleConfiguration {
    pub keys: Vec<ToggleConsoleKey>,
    pub left_pos: f32,
    pub top_pos: f32,
    pub height: f32,
    pub width: f32,
    pub help: Vec<HelpCommand>,
}

impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self {
            keys: vec![ToggleConsoleKey::KeyCode(KeyCode::Grave)],
            left_pos: 200.,
            top_pos: 100.,
            height: 400.,
            width: 800.,
            help: vec![],
        }
    }
}

pub(crate) fn console_config(mut commands: Commands, config: Option<Res<ConsoleConfiguration>>) {
    if config.is_none() {
        commands.insert_resource(ConsoleConfiguration::default());
    }
}

#[derive(Clone)]
pub struct HelpCommand {
    pub cmd: String,
    pub description: String,
}

impl HelpCommand {
    pub const fn new(cmd: String, description: String) -> Self {
        Self { cmd, description }
    }
}

// todo handle default values or something
// todo console flickers on keydown
// todo dont close console if typing, maybe?
pub(crate) fn console_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<ConsoleState>,
    config: Res<ConsoleConfiguration>,
    mut command_entered: EventWriter<ConsoleCommandEntered>,
) {
    for code in keyboard_input_events.iter() {
        let code: &KeyboardInput = code;

        let pressed = console_key_pressed(code, &config.keys);

        if pressed {
            state.show = !state.show;
        }
    }
    let scroll_height = config.height - 30.;
    let mut open = state.show;
    egui::Window::new("Console")
        .open(&mut open)
        .collapsible(false)
        .fixed_rect(egui::Rect::from_two_pos(
            egui::Pos2::new(config.left_pos, config.top_pos),
            egui::Pos2::new(
                config.left_pos + config.width,
                config.top_pos + config.height,
            ),
        ))
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_min_height(config.height);
            ui.set_min_width(config.width);
            ScrollArea::vertical()
                .max_height(scroll_height)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.set_min_height(scroll_height);
                        for line in &state.scrollback {
                            ui.label(line);
                        }
                    });
                    ui.scroll_to_cursor(Align::BOTTOM);
                });

            ui.separator();
            let text_edit = TextEdit::singleline(&mut state.buf).desired_width(config.width);
            let response = ui.add(text_edit);
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if state.buf.is_empty() {
                    state.scrollback.push(String::new());
                } else if state.buf.eq("help") {
                    let mut input = state.buf.clone();
                    state.buf.clear();
                    input.insert(0, ' ');
                    input.insert(0, '$');
                    state.scrollback.push(input);
                    state.scrollback.push("available commands:".to_string());
                    for help_command in &config.help {
                        state.scrollback.push(format!(
                            "\t{} - {}",
                            help_command.cmd, help_command.description
                        ));
                    }
                } else {
                    let msg = format!("$ {}", state.buf);
                    state.scrollback.push(msg);

                    match parse_console_command(&state.buf) {
                        Ok(cmd) => {
                            let command = ConsoleCommandEntered {
                                command: cmd.command.to_string(),
                                args: cmd.args.into_iter().map(ValueRawOwned::from).collect(),
                            };

                            command_entered.send(command);
                        }
                        Err(_) => {
                            state
                                .scrollback
                                .push("[error] invalid argument(s)".to_string());
                        }
                    }

                    state.buf.clear();
                }
            }
            ui.memory().request_focus(response.id);
        });
    state.show = open;
}

pub(crate) fn receive_console_line(
    mut console_state: ResMut<ConsoleState>,
    mut events: EventReader<PrintConsoleLine>,
) {
    for event in events.iter() {
        let event: &PrintConsoleLine = event;
        console_state.scrollback.push(event.line.clone());
    }
}

fn console_key_pressed(
    keyboard_input: &KeyboardInput,
    configured_keys: &[ToggleConsoleKey],
) -> bool {
    if !keyboard_input.state.is_pressed() {
        return false;
    }

    for configured_key in configured_keys {
        match configured_key {
            ToggleConsoleKey::KeyCode(configured_key_code) => match keyboard_input.key_code {
                None => continue,
                Some(pressed_key) => {
                    if configured_key_code == &pressed_key {
                        return true;
                    }
                }
            },
            ToggleConsoleKey::ScanCode(configured_scan_code) => {
                if &keyboard_input.scan_code == configured_scan_code {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ElementState;

    #[test]
    fn test_console_key_pressed_scan_code() {
        let input = KeyboardInput {
            scan_code: 41,
            key_code: None,
            state: ElementState::Pressed,
        };

        let config = vec![ToggleConsoleKey::ScanCode(41)];

        let result = console_key_pressed(&input, &config);
        assert!(result);
    }

    #[test]
    fn test_console_wrong_key_pressed_scan_code() {
        let input = KeyboardInput {
            scan_code: 42,
            key_code: None,
            state: ElementState::Pressed,
        };

        let config = vec![ToggleConsoleKey::ScanCode(41)];

        let result = console_key_pressed(&input, &config);
        assert!(!result);
    }

    #[test]
    fn test_console_key_pressed_key_code() {
        let input = KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::Grave),
            state: ElementState::Pressed,
        };

        let config = vec![ToggleConsoleKey::KeyCode(KeyCode::Grave)];

        let result = console_key_pressed(&input, &config);
        assert!(result);
    }

    #[test]
    fn test_console_wrong_key_pressed_key_code() {
        let input = KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::A),
            state: ElementState::Pressed,
        };

        let config = vec![ToggleConsoleKey::KeyCode(KeyCode::Grave)];

        let result = console_key_pressed(&input, &config);
        assert!(!result);
    }

    #[test]
    fn test_console_key_right_key_but_not_pressed() {
        let input = KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::Grave),
            state: ElementState::Released,
        };

        let config = vec![ToggleConsoleKey::KeyCode(KeyCode::Grave)];

        let result = console_key_pressed(&input, &config);
        assert!(!result);
    }
}
