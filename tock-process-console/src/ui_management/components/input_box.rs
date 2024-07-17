// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use super::{Component, ComponentRender};
use crate::state_store::{Action, State};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Rect,
    style::{Color, Style, Styled, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

/// View that is handling text user input
pub struct InputBox {
    /// Current user input
    text: String,
    /// Position of the cursor in the view
    cursor_position: usize,
}

impl InputBox {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
    }

    fn move_cursor_left(&mut self) {
        let next_position = self.cursor_position.saturating_sub(1);
        self.cursor_position = next_position.clamp(0, self.text.len());
    }

    fn move_cursor_right(&mut self) {
        let next_position = self.cursor_position.saturating_add(1);
        self.cursor_position = next_position.clamp(0, self.text.len());
    }

    fn enter_char(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position != 0 {
            let current_position = self.cursor_position;

            let mut input = self.text.clone();
            let text_len = self.text.len();
            let after_removed_char = &self.text[current_position..text_len];

            // Replace the suffix from the selected character with all the
            // characters after the selected character
            input.replace_range(current_position - 1..self.text.len(), after_removed_char);

            self.text = input;
            self.move_cursor_left();
        }
    }
}

impl Component for InputBox {
    fn new(
        _state: &State,
        _screen_idx: Option<usize>,
        _action_sender: UnboundedSender<Action>,
    ) -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
        }
    }

    fn name(&self) -> &str {
        "Input Box"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(char) => {
                self.enter_char(char);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            _ => {}
        }
    }

    fn update_with_state(self, _state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            text: self.text,
            cursor_position: self.cursor_position,
        }
    }

    fn handle_mouse_event(&mut self, _event: crossterm::event::MouseEvent) {}
}

pub struct RenderProperties {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

impl ComponentRender<RenderProperties> for InputBox {
    fn render(&self, frame: &mut ratatui::prelude::Frame, properties: RenderProperties) {
        let input = Paragraph::new(self.text.as_str())
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(properties.border_color)
                    .title(properties.title),
            );
        frame.render_widget(input, properties.area);

        if properties.show_cursor {
            frame.set_cursor(
                properties.area.x + self.cursor_position as u16 + 1,
                properties.area.y + 1,
            )
        }
    }
}
