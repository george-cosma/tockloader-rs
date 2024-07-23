// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use std::vec;

use crate::{
    state_store::{Action, BoardConnectionStatus, State},
    ui_management::components::{Component, ComponentRender},
};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Direction,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph, Wrap},
};

use tokio::sync::{mpsc::UnboundedSender, watch};
use tokio_serial::SerialPortType;

struct Properties {
    error_message: Option<String>,
}

#[derive(PartialEq)]
pub enum ShowState {
    ShowBoardsOnly,
    ShowAllSerialPorts,
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
    action_sender: UnboundedSender<Action>,
    properties: Properties,
    scrollbar_state_serial: ListState,
    scrollbar_state_boards: ListState,
    probeinfo_sender: watch::Sender<Vec<String>>,
    probeinfo_receiver: watch::Receiver<Vec<String>>,
    show_state: ShowState,
}

impl SetupPage {
    fn set_port(&mut self) {
        let probeinfo = self.probeinfo_receiver.borrow_and_update();

        let port_number = match self.show_state {
            ShowState::ShowBoardsOnly => {
                match self.scrollbar_state_boards.selected() {
                    Some(port) => port,
                    None => return, // Do nothing when there are no ports selected
                }
            }
            ShowState::ShowAllSerialPorts => {
                match self.scrollbar_state_serial.selected() {
                    Some(port) => port,
                    None => return, // Do nothing when there are no ports selected
                }
            }
        };

        let port = probeinfo[port_number].clone();
        self.action_sender
            .send(Action::ConnectToBoard { port })
            .expect("Expected action receiver to be open.");
    }
}

impl Component for SetupPage {
    fn new(
        state: &State,
        _screen_idx: Option<usize>,
        action_sender: UnboundedSender<Action>,
    ) -> Self
    where
        Self: Sized,
    {
        let mut scrollbar_state_serial = ListState::default();
        scrollbar_state_serial.select_first();

        let mut scrollbar_state_boards = ListState::default();
        scrollbar_state_boards.select_first();

        let collector: Vec<String> = vec![];
        let (probeinfo_sender, probeinfo_receiver) = watch::channel(collector);

        let show_state = ShowState::ShowBoardsOnly;

        SetupPage {
            action_sender: action_sender.clone(),
            properties: Properties::from(state),
            scrollbar_state_serial,
            scrollbar_state_boards,
            probeinfo_sender,
            probeinfo_receiver,
            show_state,
        }
        .update_with_state(state)
    }

    fn name(&self) -> &str {
        "Setup page"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
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
            KeyCode::Char('a') => {
                if self.show_state == ShowState::ShowBoardsOnly {
                    self.show_state = ShowState::ShowAllSerialPorts;
                    self.scrollbar_state_serial.select_first();
                } else if self.show_state == ShowState::ShowAllSerialPorts {
                    self.show_state = ShowState::ShowBoardsOnly;
                    self.scrollbar_state_boards.select_first();
                }
            }
            KeyCode::Up => {
                if self.show_state == ShowState::ShowAllSerialPorts {
                    self.scrollbar_state_serial.select_previous()
                } else if self.show_state == ShowState::ShowBoardsOnly {
                    self.scrollbar_state_boards.select_previous()
                }
            }
            KeyCode::Down => {
                if self.show_state == ShowState::ShowAllSerialPorts {
                    self.scrollbar_state_serial.select_next()
                } else if self.show_state == ShowState::ShowBoardsOnly {
                    self.scrollbar_state_boards.select_next()
                }
            }
            KeyCode::PageUp => {
                if self.show_state == ShowState::ShowAllSerialPorts {
                    self.scrollbar_state_serial.select_previous()
                } else if self.show_state == ShowState::ShowBoardsOnly {
                    self.scrollbar_state_boards.select_previous()
                }
            }
            KeyCode::PageDown => {
                if self.show_state == ShowState::ShowAllSerialPorts {
                    self.scrollbar_state_serial.select_next()
                } else if self.show_state == ShowState::ShowBoardsOnly {
                    self.scrollbar_state_boards.select_next()
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
            action_sender: self.action_sender,
            scrollbar_state_serial: self.scrollbar_state_serial,
            scrollbar_state_boards: self.scrollbar_state_boards,
            probeinfo_sender: self.probeinfo_sender,
            probeinfo_receiver: self.probeinfo_receiver,
            show_state: self.show_state,
        }
    }

    fn handle_mouse_event(&mut self, _event: crossterm::event::MouseEvent) {}
}

impl ComponentRender<()> for SetupPage {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, _properties: ()) {
        let temp_serial_position_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Min(2),
                Constraint::Percentage(35),
            ]);

        let [_, serial_position_v, _] = temp_serial_position_v.areas(frame.size());

        let temp_serial_position_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(18),
                Constraint::Min(2),
                Constraint::Percentage(18),
            ]);
        let [_, serial_position_h, _] = temp_serial_position_h.areas(serial_position_v);

        let available_ports = match tokio_serial::available_ports() {
            Ok(ports) => ports,
            Err(error) => panic!("Error while searching for ports: {}", error),
        };

        let mut vec_serial: Vec<Text> = vec![];
        let mut vec_boards: Vec<Text> = vec![];
        let mut board_ports: Vec<String> = vec![];
        let mut serial_ports: Vec<String> = vec![];
        for (port_index, port) in available_ports.iter().enumerate() {
            let product = match &port.port_type {
                SerialPortType::UsbPort(usb) => usb.product.clone(),
                SerialPortType::PciPort => Some("PciPort".to_string()),
                SerialPortType::BluetoothPort => Some("BluetoothPort".to_string()),
                SerialPortType::Unknown => Some("Unknown".to_string()),
            };
            let temp_serial = format! {"Port[{port_index}](Name:{:#?}, Type:{}), \n", port.port_name, product.unwrap_or("Unknown".to_string())};
            if let SerialPortType::UsbPort(_) = port.port_type {
                // Add to boards only if its a USB type.
                vec_boards.push(temp_serial.clone().into());
                board_ports.push(port.port_name.clone());
            }
            // Fall-through, add to serial regardless of type.
            vec_serial.push(temp_serial.into());
            serial_ports.push(port.port_name.clone());
        }

        if self.show_state == ShowState::ShowAllSerialPorts {
            let list = List::new(vec_serial)
                .style(Style::default().fg(Color::Cyan))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .fg(Color::Yellow)
                        .title(format!(" Serial ports - {} ", available_ports.len()))
                        .title_style(Style::default().fg(Color::Blue)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(" > ")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(list, serial_position_h, &mut self.scrollbar_state_serial);
        }

        if self.show_state == ShowState::ShowBoardsOnly {
            let boards_found = vec_boards.len();

            let list = List::new(vec_boards)
                .style(Style::default().fg(Color::Cyan))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .fg(Color::Yellow)
                        .title(format!(" Number of boards found: {}  ", boards_found))
                        .title_style(Style::default().fg(Color::Blue)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(" > ")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(list, serial_position_h, &mut self.scrollbar_state_boards);
        }

        if self.show_state == ShowState::ShowBoardsOnly {
            match self.probeinfo_sender.send(board_ports) {
                Ok(data) => data,
                Err(error) => println!("{}", error),
            };
        } else if self.show_state == ShowState::ShowAllSerialPorts {
            match self.probeinfo_sender.send(serial_ports) {
                Ok(data) => data,
                Err(error) => println!("{}", error),
            };
        }

        let temp_help_text_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(65),
                Constraint::Min(2),
                Constraint::Percentage(10),
            ]);

        let [_, help_text_v, _] = temp_help_text_v.areas(frame.size());

        let temp_help_text_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(18),
                Constraint::Min(2),
                Constraint::Percentage(35),
            ]);

        let [_, help_text_h, _] = temp_help_text_h.areas(help_text_v);

        let temp_panic_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Min(2),
                Constraint::Percentage(10),
            ]);

        let [_, panic_v, _] = temp_panic_v.areas(frame.size());

        let temp_panic_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(18),
                Constraint::Min(2),
                Constraint::Percentage(35),
            ]);

        let [_, panic_h, _] = temp_panic_h.areas(panic_v);

        let temp_show_text_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(69),
                Constraint::Min(2),
                Constraint::Percentage(10),
            ]);

        let [_, show_text_v, _] = temp_show_text_v.areas(frame.size());

        let temp_show_text_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(18),
                Constraint::Min(2),
                Constraint::Percentage(35),
            ]);

        let [_, show_text_h, _] = temp_show_text_h.areas(show_text_v);

        let show_text = Paragraph::new(Text::from("Press A to switch display mode."));
        frame.render_widget(show_text, show_text_h);

        let temp_enter_text_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(72),
                Constraint::Min(2),
                Constraint::Percentage(10),
            ]);

        let [_, enter_text_v, _] = temp_enter_text_v.areas(frame.size());

        let temp_enter_text_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(18),
                Constraint::Min(2),
                Constraint::Percentage(35),
            ]);

        let [_, enter_text_h, _] = temp_enter_text_h.areas(enter_text_v);

        let help_text = Paragraph::new(Text::from("Press Enter to select highlighted port."));
        frame.render_widget(help_text, enter_text_h);

        let help_text = Paragraph::new(Text::from("Use ▲ ▼ PageUp PageDown to scroll.  "));
        frame.render_widget(help_text, help_text_h);

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

        frame.render_widget(error_message, panic_h);
    }
}
