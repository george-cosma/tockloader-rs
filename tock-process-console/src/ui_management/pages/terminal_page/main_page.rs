// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state_store::{Action, AppData, State},
    ui_management::components::{Component, ComponentRender},
};

use super::{
    applications_page::ApplicationsPage,
    components::apps_list,
    section::usage::{widget_usage_to_text, UsageInfo, UsageInfoLine},
};

struct Properties {
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    active_apps: Vec<(usize, Option<String>)>,
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    app_data_map: HashMap<String, AppData>,
}

impl From<&State> for Properties {
    fn from(state: &State) -> Self {
        Self {
            active_apps: state.active_apps.clone(),
            app_data_map: state.apps_data_map.clone(),
        }
    }
}

pub struct MainPage {
    pub action_sender: UnboundedSender<Action>,
    // TODO(NegrilaRares): investigate if we need port
    #[allow(dead_code)]
    properties: Properties,
    pub hovered_screen: usize,
    pub active_screen: Option<usize>,
    pub screens: Vec<ApplicationsPage>,
}

impl MainPage {
    fn hover_next_screen(&mut self) {
        let current_idx = self.hovered_screen;
        let next_idx = (current_idx + 1) % self.screens.len();

        self.hovered_screen = next_idx;
    }

    fn hover_previous_screen(&mut self) {
        let current_idx = self.hovered_screen;
        let previous_idx = if current_idx == 0 {
            self.screens.len() - 1
        } else {
            current_idx - 1
        };

        self.hovered_screen = previous_idx;
    }
    fn disable_active_screen(&mut self) {
        self.active_screen = None;
    }
}
impl Component for MainPage {
    fn new(
        state: &State,
        _screen_idx: Option<usize>,
        action_sender: UnboundedSender<Action>,
    ) -> Self
    where
        Self: Sized,
    {
        MainPage {
            action_sender: action_sender.clone(),
            properties: Properties::from(state),
            hovered_screen: 0,
            active_screen: Option::None,
            screens: vec![ApplicationsPage::new(state, Some(0), action_sender.clone())],
        }
        .update_with_state(state)
    }

    fn update_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        let mut updated_screens = Vec::new();
        for screen in self.screens.clone() {
            updated_screens.push(screen.update_with_state(state))
        }

        let action_sender = self.screens.last().unwrap().action_sender.clone();
        for (new_screen_idx, _) in state.active_apps.clone() {
            if self
                .screens
                .iter()
                .find(|app| app.screen_idx == new_screen_idx)
                .is_none()
            {
                updated_screens.push(ApplicationsPage::new(
                    state,
                    Some(new_screen_idx),
                    action_sender.clone(),
                ));
            }
        }

        MainPage {
            properties: Properties::from(state),
            screens: updated_screens,
            ..self
        }
    }

    fn name(&self) -> &str {
        "Main Page"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        let active_screen = self.active_screen.clone();

        match active_screen {
            None => match key.code {
                KeyCode::Tab => {
                    let _ = self.action_sender.send(Action::AddScreen {
                        screen_idx: self.screens.len(),
                    });
                }
                KeyCode::Left => self.hover_previous_screen(),
                KeyCode::Right => self.hover_next_screen(),
                KeyCode::Enter => {
                    let last_hovered_screen = self.hovered_screen;
                    self.active_screen = Some(last_hovered_screen);
                }
                KeyCode::Char('q') => {
                    let _ = self.action_sender.send(Action::Exit);
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let _ = self.action_sender.send(Action::Exit);
                }
                _ => {}
            },
            Some(screen) => {
                self.screens[screen].handle_key_event(key);

                if key.code == KeyCode::Esc {
                    self.disable_active_screen();
                }
            }
        }
    }

    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) {
        if let Some(active_screen) = self.active_screen {
            self.screens[active_screen].handle_mouse_event(event);
        }
    }
}

impl ComponentRender<()> for MainPage {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, _properties: ()) {
        let [left, right] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(frame.size())
        else {
            panic!("The main layout should have 3 chunks")
        };

        let mut constraints: Vec<Constraint> = Vec::new();
        for _ in 0..self.screens.len() {
            constraints.push(Constraint::Percentage(100 / (self.screens.len()) as u16));
        }

        let terminals = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_slice())
            .split(left);

        for idx in 0..terminals.len() {
            let is_hovered_screen = if idx == self.hovered_screen {
                true
            } else {
                false
            };
            let is_active_screen = if idx == self.active_screen.unwrap_or(usize::MAX) {
                true
            } else {
                false
            };

            self.screens[idx].render(
                frame,
                apps_list::RenderProperties {
                    border_color: if is_active_screen {
                        ratatui::style::Color::LightMagenta
                    } else if is_hovered_screen {
                        ratatui::style::Color::DarkGray
                    } else {
                        ratatui::style::Color::White
                    },
                    area: terminals[idx],
                },
            )
        }

        // let usage_text: Text = widget_usage_to_text(self.usage_info());
        let usage_info = UsageInfo {
            description: Some("Select the running process to interact with".into()),
            lines: vec![
                UsageInfoLine {
                    keys: vec!["Esc".into()],
                    description: "to cancel.".into(),
                },
                UsageInfoLine {
                    keys: vec!["↑".into(), "↓".into()],
                    description: "to navigate".into(),
                },
                UsageInfoLine {
                    keys: vec!["Enter".into()],
                    description: "to select a process.".into(),
                },
            ],
        };
        let usage_text = widget_usage_to_text(usage_info);
        let usage_text = usage_text.patch_style(Style::default());
        let usage = Paragraph::new(usage_text)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Usage"));
        frame.render_widget(usage, right);
    }
}
