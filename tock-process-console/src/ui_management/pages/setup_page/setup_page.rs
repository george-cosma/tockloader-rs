// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::{
    state_store::{Action, BoardConnectionStatus, State},
    ui_management::components::{input_box, Component, ComponentRender, InputBox},
};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Direction,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

struct Properties {
    error_message: Option<String>,
}

impl From<&State> for Properties {
    fn from(state: &State) -> Self {
        let error_message =
            if let BoardConnectionStatus::Errored { err } = &state.board_connection_status {
                Some(err.clone())
            } else {
                None
            };

        Properties { error_message }
    }
}

/// Struct that handles setup of the console application
pub struct SetupPage {
    input_box: InputBox,
    action_sender: UnboundedSender<Action>,
    properties: Properties,
}

impl SetupPage {
    fn set_port(&mut self) {
        // should update the port
        if self.input_box.is_empty() {
            return;
        }

        let port = self.input_box.text();
        let _ = self.action_sender.send(Action::ConnectToBoard {
            port: port.to_string(),
        });
    }
}

impl Component for SetupPage {
    fn new(state: &State, screen_idx: Option<usize>, action_sender: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let input_box = InputBox::new(state, screen_idx, action_sender.clone());

        SetupPage {
            action_sender: action_sender.clone(),
            input_box,
            properties: Properties::from(state),
        }
        .update_with_state(state)
    }

    fn name(&self) -> &str {
        "Setup page"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        self.input_box.handle_key_event(key);

        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Enter => {
                self.set_port();
            }
            KeyCode::Char('c') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    let _ = self.action_sender.send(Action::Exit);
                }
            }
            _ => {}
        }
    }

    fn update_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            properties: Properties::from(state),
            input_box: self.input_box,
            action_sender: self.action_sender,
        }
    }

    fn handle_mouse_event(&mut self, _event: crossterm::event::MouseEvent) {}
}

impl ComponentRender<()> for SetupPage {
    fn render(&self, frame: &mut ratatui::prelude::Frame, _properties: ()) {
        let [_, vertical_centered, _] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Min(1),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size())
        else {
            panic!("afa")
        };

        let [_, both_centered, _] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Min(1),
                Constraint::Ratio(1, 3),
            ])
            .split(vertical_centered)
        else {
            panic!("adfikjge")
        };

        let [container_port_input, container_help_text, container_error_message] =
            *Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(both_centered)
        else {
            panic!("adfhfla")
        };

        // let available_ports = match tokio_serial::available_ports() {
        //     Ok(ports) => ports,
        //     Err(error) => panic!("ports not found!"),
        // };

        // let [container_port_output, container_help_text, container_error_message] =
        //     *Layout::default()
        //         .direction(Direction::Horizontal)
        //         .constraints([
        //             Constraint::Length(3),
        //             Constraint::Length(available_ports.len().try_into().unwrap()),
        //             Constraint::Min(1),
        //         ])
        //         .split(both_centered)
        // else {
        //     panic!("adfhfla")
        // };

        self.input_box.render(
            frame,
            input_box::RenderProperties {
                title: "Serial port".to_string(),
                area: container_port_input,
                border_color: Color::Yellow,
                show_cursor: true,
            },
        );

        let help_text = Paragraph::new(Text::from("Press <Enter> to connect"));
        frame.render_widget(help_text, container_help_text);

        let error = if let Some(error) = &self.properties.error_message {
            Text::from(format!("Error: {}", error))
        } else {
            Text::from("")
        };
        let error_message = Paragraph::new(error).wrap(Wrap { trim: true }).style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC),
        );

        frame.render_widget(error_message, container_error_message);
    }
}
