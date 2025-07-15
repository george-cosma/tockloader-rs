// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::ui_management::pages::Component;
use bytes::Bytes;
use crossterm::event::{KeyCode, KeyEventKind, MouseEventKind};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::block::Block;
use ratatui::widgets::{Borders, Paragraph};
use tokio::sync::mpsc::UnboundedSender;
use tui_term::vt100::Parser;
use tui_term::widget::PseudoTerminal;

use crate::state_store::{Action, AppData, State};
use crate::ui_management::components::ComponentRender;
use crate::ui_management::pages::terminal_page::section::SectionActivation;

#[derive(Clone)]
struct Properties {
    /// Active application that is displayed to the user
    active_apps: Vec<(usize, Option<String>)>,
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    app_data_map: HashMap<String, AppData>,
    apps_parsers_map: HashMap<String, Arc<Mutex<Parser>>>,
}

impl From<&State> for Properties {
    fn from(state: &State) -> Self {
        Self {
            active_apps: state.active_apps.clone(),
            app_data_map: state.apps_data_map.clone(),
            // apps_parsers_map: state.apps_parsers_map.clone()
            apps_parsers_map: state.apps_parsers_map.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TerminalBox {
    action_sender: UnboundedSender<Action>,
    screen_idx: usize,
    properties: Properties,
    scrollback_lines: usize,
    // input: String,
}

impl TerminalBox {
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    fn get_app_data(&self, name: &str) -> Option<&AppData> {
        self.properties.app_data_map.get(name)
    }

    pub fn set_screen_idx(&mut self, index: usize) {
        self.screen_idx = index;
    }
}

impl Component for TerminalBox {
    fn new(
        state: &crate::state_store::State,
        screen_idx: Option<usize>,
        action_sender: UnboundedSender<Action>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            action_sender: action_sender.clone(),
            screen_idx: screen_idx.unwrap_or_default(),
            properties: Properties::from(state),
            scrollback_lines: 0,
        }
    }

    fn update_with_state(self, state: &crate::state_store::State) -> Self
    where
        Self: Sized,
    {
        Self {
            properties: Properties::from(state),
            ..self
        }
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(to_insert) => {
                let _ = self.action_sender.send(Action::SendMessage {
                    content: Bytes::from(to_insert.to_string().into_bytes()),
                });
            }
            KeyCode::Enter => {
                #[cfg(unix)]
                let _ = self.action_sender.send(Action::SendMessage {
                    content: Bytes::from(vec![b'\n']),
                });
                #[cfg(windows)]
                let _ = self.action_sender.send(Action::SendMessage {
                    content: Bytes::from(vec![b'\r', b'\n']),
                });
            }
            KeyCode::Backspace => {
                let _ = self.action_sender.send(Action::SendMessage {
                    content: Bytes::from(vec![8]),
                });
            }
            KeyCode::Left => self
                .action_sender
                .send(Action::SendMessage {
                    content: Bytes::from(vec![27, 91, 68]),
                })
                .unwrap(),
            KeyCode::Right => self
                .action_sender
                .send(Action::SendMessage {
                    content: Bytes::from(vec![27, 91, 67]),
                })
                .unwrap(),
            KeyCode::Up => self
                .action_sender
                .send(Action::SendMessage {
                    content: Bytes::from(vec![27, 91, 65]),
                })
                .unwrap(),
            KeyCode::Down => self
                .action_sender
                .send(Action::SendMessage {
                    content: Bytes::from(vec![27, 91, 66]),
                })
                .unwrap(),
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) {
        match event.kind {
            MouseEventKind::ScrollDown => {
                let mut active_parser = self
                    .properties
                    .apps_parsers_map
                    .get(
                        self.properties
                            .active_apps
                            .iter()
                            .find(|(idx, _)| idx == &self.screen_idx)
                            .map(|(_, app)| app.as_ref().unwrap())
                            .unwrap()
                            .as_str(),
                    )
                    .unwrap()
                    .lock()
                    .unwrap();

                self.scrollback_lines =
                    if self.scrollback_lines < active_parser.screen().size().0 as usize {
                        self.scrollback_lines + 1
                    } else {
                        self.scrollback_lines
                    };
                active_parser.set_scrollback(self.scrollback_lines);
            }
            MouseEventKind::ScrollUp => {
                let active_parser = self
                    .properties
                    .apps_parsers_map
                    .get(
                        self.properties
                            .active_apps
                            .iter()
                            .find(|(idx, _)| idx == &self.screen_idx)
                            .map(|(_, app)| app.as_ref().unwrap())
                            .unwrap()
                            .as_str(),
                    )
                    .unwrap();

                self.scrollback_lines = self
                    .scrollback_lines
                    .checked_sub(1)
                    .unwrap_or(self.scrollback_lines);
                active_parser
                    .lock()
                    .unwrap()
                    .set_scrollback(self.scrollback_lines);
            }
            _ => {}
        }
    }
}

impl SectionActivation for TerminalBox {
    fn activate(&mut self) {}

    fn deactivate(&mut self) {}
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    pub show_cursor: bool,
}

impl ComponentRender<RenderProps> for TerminalBox {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, properties: RenderProps) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    ratatui::layout::Constraint::Percentage(100),
                    ratatui::layout::Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(properties.area);

        if let Some(Some(active_app)) = self
            .properties
            .active_apps
            .iter()
            .find(|(idx, _)| idx == &self.screen_idx)
            .map(|(_, app)| app)
        {
            // writeln!(file, "We have active app when rendering the terminal");
            if let Some(parser) = self.properties.apps_parsers_map.get(active_app) {
                let mut parser = parser.lock().unwrap();
                parser.set_size(
                    (properties.area.rows().count() as u16) - 5,
                    properties.area.columns().count() as u16,
                );

                let pseudo_terminal = PseudoTerminal::new(parser.screen()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::new().fg(properties.border_color))
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                );
                frame.render_widget(pseudo_terminal, chunks[0]);
            }
        };

        let explanation = "Press q to exit".to_string();
        let explanation = Paragraph::new(explanation)
            .style(Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED))
            .alignment(Alignment::Center);

        frame.render_widget(explanation, chunks[1]);
    }
}
