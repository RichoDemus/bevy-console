use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_egui::egui::{Align, ScrollArea};
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ConsoleState::default());
        app.add_event::<ConsoleCommandEntered>();
        app.add_event::<PrintConsoleLine>();
        app.add_plugin(EguiPlugin);
        // if there's other egui code we need to make sure they don't run at the same time
        app.add_system(console_system.exclusive_system());
        app.add_system(receive_console_line.system());
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsoleCommandEntered {
    pub command: String, // todo maybe enum?
    pub args: String,    // todo actual arg parsing probably
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

impl From<String> for ConsoleCommandEntered {
    fn from(str: String) -> Self {
        let separator = str.find(' ');

        let index = match separator {
            None => {
                return Self {
                    command: str,
                    args: "".to_string(),
                };
            }
            Some(index) => index,
        };

        let (cmd, args) = str.split_at(index);
        let mut args = args.to_string();
        args.replace_range(0..1, "");

        Self {
            command: cmd.to_string(),
            args,
        }
    }
}

#[derive(Default)]
struct ConsoleState {
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
    pub key: ToggleConsoleKey,
    pub left_pos: f32,
    pub top_pos: f32,
    pub height: f32,
    pub width: f32,
    pub help: Vec<HelpCommand>,
}

impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self {
            key: ToggleConsoleKey::KeyCode(KeyCode::Grave),
            left_pos: 200.,
            top_pos: 100.,
            height: 400.,
            width: 800.,
            help: vec![],
        }
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
fn console_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    egui_context: Res<EguiContext>,
    mut state: ResMut<ConsoleState>,
    config: Res<ConsoleConfiguration>,
    mut command_entered: EventWriter<ConsoleCommandEntered>,
) {
    for code in keyboard_input_events.iter() {
        let code: &KeyboardInput = code;

        let is_right_key = match config.key {
            ToggleConsoleKey::KeyCode(key) => match code.key_code {
                None => false,
                Some(pressed_key) => key == pressed_key,
            },
            ToggleConsoleKey::ScanCode(pressed_key) => code.scan_code == pressed_key,
        };

        if is_right_key && code.state.is_pressed() {
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
        .show(egui_context.ctx(), |ui| {
            ui.set_min_height(config.height);
            ui.set_min_width(config.width);
            ScrollArea::from_max_height(scroll_height).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_min_height(scroll_height);
                    for line in &state.scrollback {
                        ui.label(line);
                    }
                });
                ui.scroll_to_cursor(Align::BOTTOM);
            });

            ui.separator();
            let response = ui.text_edit_singleline(&mut state.buf);
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if state.buf.is_empty() {
                    state.scrollback.push(String::new());
                } else {
                    if state.buf.eq("help") {
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
                        let mut input = state.buf.clone();
                        state.buf.clear();
                        let command = input.clone().into(); // todo dont clone
                        command_entered.send(command);
                        input.insert(0, ' ');
                        input.insert(0, '$');
                        state.scrollback.push(input);
                    }
                }
            }
            ui.memory().request_focus(response.id);
        });
    state.show = open;
}

fn receive_console_line(
    mut console_state: ResMut<ConsoleState>,
    mut events: EventReader<PrintConsoleLine>,
) {
    for event in events.iter() {
        let event: &PrintConsoleLine = event;
        console_state.scrollback.push(event.line.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command() {
        let expected = ConsoleCommandEntered {
            command: "my-cmd".to_string(),
            args: "arg1 arg2".to_string(),
        };

        let input = "my-cmd arg1 arg2".to_string();

        let result: ConsoleCommandEntered = input.into();

        assert_eq!(result, expected);
    }
}
