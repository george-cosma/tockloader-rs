use super::super::section::usage::{HasUsageInfo, UsageInfo, UsageInfoLine};
use crate::{
    state_store::{Action, State},
    ui_management::{
        components::{Component, ComponentRender},
        pages::terminal_page::section::SectionActivation,
    },
};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::{fs::OpenOptions, io::Write};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
struct AppState {
    pub pid: String,
    pub name: String,
    pub has_new_input: bool,
}

#[derive(Clone)]
struct Properties {
    apps: Vec<AppState>,
}

impl From<&State> for Properties {
    fn from(state: &State) -> Self {
        let mut apps = state
            .apps_data_map
            .iter()
            .map(|(name, app_data)| AppState {
                pid: app_data.pid.to_string(),
                name: name.clone(),
                has_new_input: app_data.has_new_logs,
            })
            .collect::<Vec<AppState>>();

        apps.sort_by(|a, b| a.name.cmp(&b.name));

        Self { apps }
    }
}

#[derive(Clone)]
pub struct AppsList {
    action_sender: UnboundedSender<Action>,
    properties: Properties,
    pub list_state: ListState,
    pub active_app: Option<String>,
    pub screen_idx: usize,
}

impl AppsList {
    fn next(&mut self) {
        // Compute the value of the next index based on the currently selected one
        let index = match self.list_state.selected() {
            Some(idx) => {
                if idx >= self.properties.apps.len() - 1 {
                    0
                } else {
                    idx + 1
                }
            }
            None => 0,
        };

        self.list_state.select(Some(index));
    }

    fn previous(&mut self) {
        // Compute the value of the previous index based on the currently selected one
        let index = match self.list_state.selected() {
            Some(idx) => {
                if idx == 0 {
                    self.properties.apps.len() - 1
                } else {
                    idx - 1
                }
            }
            None => 0,
        };

        self.list_state.select(Some(index));
    }

    pub(super) fn apps(&self) -> &Vec<AppState> {
        &self.properties.apps
    }

    fn get_room_index(&self, name: &str) -> Option<usize> {
        self.properties
            .apps
            .iter()
            .enumerate()
            .find_map(|(index, app_state)| {
                if app_state.name == name {
                    Some(index)
                } else {
                    None
                }
            })
    }

    pub fn set_screen_idx(&mut self, index: usize) {
        self.screen_idx = index;
    }
}

impl Component for AppsList {
    fn new(state: &State, screen_idx: Option<usize>, action_sender: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        Self {
            action_sender,
            properties: Properties::from(state),
            list_state: ListState::default(),
            active_app: None,
            screen_idx: screen_idx.unwrap_or_default(),
        }
    }

    fn name(&self) -> &str {
        "Apps List"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Up => {
                self.previous();
            }
            KeyCode::Down => {
                self.next();
            }
            KeyCode::Enter => {
                if let Some(selected_app_index) = self.list_state.selected() {
                    let applications = self.apps();

                    if let Some(app_state) = applications.get(selected_app_index) {
                        let _ = self.action_sender.send(Action::SelectApplication {
                            screen_idx: self.screen_idx,
                            app_name: app_state.name.clone(),
                        });
                    } else {
                        // TODO: should handle the error
                    }
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
            // action_sender: self.action_sender,
            // list_state: self.list_state,
            ..self
        }
    }

    fn handle_mouse_event(&mut self, _event: crossterm::event::MouseEvent) {}
}

pub struct RenderProperties {
    pub border_color: Color,
    pub area: Rect,
}

impl ComponentRender<RenderProperties> for AppsList {
    fn render(&self, frame: &mut ratatui::prelude::Frame, properties: RenderProperties) {
        let active_application = self.active_app.clone();

        let apps_list: Vec<ListItem> = self
            .apps()
            .iter()
            .map(|app_state| {
                let content = Line::from(Span::raw(app_state.name.clone()));

                let style = if self.list_state.selected().is_none()
                    && active_application.is_some()
                    && active_application.as_ref().unwrap().eq(&app_state.name)
                {
                    Style::default().add_modifier(Modifier::BOLD)
                } else if app_state.has_new_input {
                    Style::default().add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC)
                } else {
                    Style::default()
                };

                ListItem::new(content).style(style.bg(Color::Reset))
            })
            .collect();

        let apps_list_component = List::new(apps_list)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(properties.border_color))
                    .title("Running applications"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">");

        let mut applications_list_state = self.list_state.clone();
        frame.render_stateful_widget(
            apps_list_component,
            properties.area,
            &mut applications_list_state,
        );
    }
}

impl SectionActivation for AppsList {
    fn activate(&mut self) {
        let idx = self
            .active_app
            .as_ref()
            .and_then(|app_name| self.get_room_index(&app_name))
            .unwrap_or(0);

        *self.list_state.offset_mut() = 0;
        self.list_state.select(Some(idx));
    }

    fn deactivate(&mut self) {
        *self.list_state.offset_mut() = 0;
        self.list_state.select(None);
    }
}

impl HasUsageInfo for AppsList {
    fn usage_info(&self) -> UsageInfo {
        UsageInfo {
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
                    description: "to select application.".into(),
                },
            ],
        }
    }
}
