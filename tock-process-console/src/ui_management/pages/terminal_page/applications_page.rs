// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use super::{
    components::{
        apps_list::{self, AppsList},
        terminal_box::{RenderProps, TerminalBox},
    },
    section::{
        self,
        usage::{HasUsageInfo, UsageInfo, UsageInfoLine},
        SectionActivation,
    },
};
use crate::{
    state_store::{Action, AppData, State},
    ui_management::components::{Component, ComponentRender},
};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use std::{collections::HashMap, fs::OpenOptions, io::Write};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    AppsList,
    Terminal,
}

impl Section {
    pub const COUNT: usize = 2;

    fn to_usize(&self) -> usize {
        match self {
            Section::AppsList => 0,
            Section::Terminal => 1,
        }
    }
}

impl TryFrom<usize> for Section {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Section::AppsList),
            1 => Ok(Section::Terminal),
            _ => Err(()),
        }
    }
}

impl Default for Section {
    fn default() -> Self {
        Section::AppsList
    }
}

#[derive(Clone, Debug)]
struct Properties {
    active_apps: Vec<(usize, Option<String>)>,
    app_data_map: HashMap<String, AppData>,
}

impl From<&State> for Properties {
    fn from(state: &State) -> Self {
        Self {
            // TODO: the active_app should be different for each instnace of
            // ApplicationsPage, but i need to know the index of the app in
            // order to know which is the active_app for this screen
            // active_app: state.active_apps.clone(),
            active_apps: state.active_apps.clone(),
            app_data_map: state.apps_data_map.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ApplicationsPage {
    /// Action sender channel
    pub action_sender: UnboundedSender<Action>,
    /// The identificatior of the screen in focus
    pub screen_idx: usize,
    /// State of the terminal page
    properties: Properties,
    pub active_section: Option<Section>,
    /// Currently hovered section in the page
    pub currently_hovered_section: Section,
    /// Separate widget component that handles the listing of the applications on the board
    pub app_list: AppsList,
    /// Separate widget component that handles incoming messages from applications and user input
    pub terminal: TerminalBox,
}

impl ApplicationsPage {
    fn get_app_data(&self, name: &str) -> Option<&AppData> {
        self.properties.app_data_map.get(name)
    }

    fn get_component_for_section(&self, section: &Section) -> &dyn Component {
        match section {
            Section::AppsList => &self.app_list,
            Section::Terminal => &self.terminal,
        }
    }

    fn get_component_for_section_mut<'a>(&'a mut self, section: &Section) -> &'a mut dyn Component {
        match section {
            Section::Terminal => &mut self.terminal,
            Section::AppsList => &mut self.app_list,
        }
    }

    fn get_section_activation_for_section<'a>(
        &'a mut self,
        section: &Section,
    ) -> &'a mut dyn SectionActivation {
        match section {
            Section::Terminal => &mut self.terminal,
            Section::AppsList => &mut self.app_list,
        }
    }

    fn hover_next(&mut self) {
        let idx: usize = self.currently_hovered_section.to_usize();
        let next_idx = (idx + 1) % Section::COUNT;
        self.currently_hovered_section = Section::try_from(next_idx).unwrap();
    }

    fn hover_previous(&mut self) {
        let idx: usize = self.currently_hovered_section.to_usize();
        let previous_idx = if idx == 0 {
            Section::COUNT - 1
        } else {
            idx - 1
        };
        self.currently_hovered_section = Section::try_from(previous_idx).unwrap();
    }

    fn calculate_border_color(&self, section: Section) -> Color {
        match (
            self.active_section.as_ref(),
            &self.currently_hovered_section,
        ) {
            (Some(active_section), _) if active_section.eq(&section) => Color::Yellow,
            (_, hovered_section) if hovered_section.eq(&section) => Color::Blue,
            _ => Color::Reset,
        }
    }

    fn disable_section(&mut self, section: &Section) {
        self.get_section_activation_for_section(section)
            .deactivate();

        self.active_section = None;
    }

    pub fn set_screen_idx(&mut self, index: usize) {
        self.screen_idx = index;

        self.terminal.set_screen_idx(index);
        self.app_list.set_screen_idx(index);
    }
}

const DEFAULT_HOVERED_SECTION: Section = Section::AppsList;

impl Component for ApplicationsPage {
    fn new(state: &State, screen_idx: Option<usize>, action_sender: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ApplicationsPage {
            action_sender: action_sender.clone(),
            screen_idx: screen_idx.unwrap_or_default(),
            properties: Properties::from(state),
            active_section: Option::None,
            currently_hovered_section: DEFAULT_HOVERED_SECTION,
            app_list: AppsList::new(state, screen_idx, action_sender.clone()),
            terminal: TerminalBox::new(state, screen_idx, action_sender.clone()),
        }
        .update_with_state(state)
    }

    fn update_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        ApplicationsPage {
            properties: Properties::from(state),
            app_list: self.app_list.update_with_state(state),
            terminal: self.terminal.update_with_state(state),
            ..self
        }
    }

    fn name(&self) -> &str {
        "Applications Page"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        let active_section = self.active_section.clone();

        match active_section {
            // The action is destined for the ApplicationsPage overall as there is no component selected yet
            None => match key.code {
                KeyCode::Enter => {
                    let last_hovered_section = self.currently_hovered_section.clone();

                    self.active_section = Some(last_hovered_section.clone());
                    self.get_section_activation_for_section(&last_hovered_section)
                        .activate();
                }
                KeyCode::Left => self.hover_previous(),
                KeyCode::Right => self.hover_next(),
                KeyCode::Char('q') => {
                    let _ = self.action_sender.send(Action::Exit);
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let _ = self.action_sender.send(Action::Exit);
                }
                _ => {}
            },
            Some(section) => {
                self.get_component_for_section_mut(&section)
                    .handle_key_event(key);

                match section {
                    Section::AppsList if key.code == KeyCode::Enter => {
                        self.disable_section(&section);

                        self.active_section = Some(Section::Terminal);
                        self.get_section_activation_for_section(&Section::Terminal)
                            .activate();
                    }
                    _ if key.code == KeyCode::BackTab => self.disable_section(&section),
                    _ => (),
                }
            }
        }
    }

    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) {
        let active_section = self.active_section.clone();

        if let Some(active_section) = active_section {
            self.get_component_for_section_mut(&active_section)
                .handle_mouse_event(event);
        }
    }
}

const NO_APP_SELECTED_MESSAE: &str = "Select an process to see its logs!";

impl ComponentRender<apps_list::RenderProperties> for ApplicationsPage {
    fn render(&self, frame: &mut ratatui::prelude::Frame, properties: apps_list::RenderProperties) {
        let [container_active_app_header, container_content] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(1)].as_ref())
            .split(properties.area)
        else {
            panic!("The left layout should have 2 chunks")
        };

        let top_line = if let Some(app_data) = self
            .properties
            .active_apps
            .iter()
            .find(|(screen_idx, _)| screen_idx == &self.screen_idx)
            .map(|(_, active_app)| active_app)
            .and_then(|active_app| active_app.clone())
            .and_then(|active_app| self.get_app_data(active_app.as_ref()))
        {
            let mut app_info = vec![Span::from(format!("{}", app_data.name))];

            if app_data.is_app {
                app_info.append(&mut vec![
                    " --- PID ".into(),
                    Span::from(format!("{}", app_data.pid)),
                ])
            }

            Line::from(app_info)
        } else {
            Line::from(NO_APP_SELECTED_MESSAE)
        };
        let text = Text::from(top_line);

        let help_message = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Active Process Information"),
        );
        frame.render_widget(help_message, container_active_app_header);

        if self.active_section.clone().unwrap_or_default() == Section::AppsList {
            self.app_list.render(
                frame,
                apps_list::RenderProperties {
                    // border_color: self.calculate_border_color(Section::AppsList),
                    border_color: properties.border_color,
                    area: container_content,
                },
            );
        } else {
            self.terminal.render(
                frame,
                RenderProps {
                    area: container_content,
                    // border_color: self.calculate_border_color(Section::Terminal),
                    border_color: properties.border_color,
                    show_cursor: self
                        .active_section
                        .as_ref()
                        .map(|active_section| {
                            (active_section.to_usize()).eq(&Section::Terminal.to_usize())
                        })
                        .unwrap_or(false),
                },
            );
        }

        // let usage_text: Text = widget_usage_to_text(self.usage_info());
        // let usage_text = usage_text.patch_style(Style::default());
        // let usage = Paragraph::new(usage_text)
        //     .wrap(Wrap { trim: true })
        //     .block(Block::default().borders(Borders::ALL).title("Usage"));
        // frame.render_widget(usage, right);
    }
}

impl HasUsageInfo for ApplicationsPage {
    fn usage_info(&self) -> section::usage::UsageInfo {
        if let Some(section) = self.active_section.as_ref() {
            let handler: &dyn HasUsageInfo = match section {
                Section::AppsList => &self.app_list,
                Section::Terminal => &self.terminal,
            };

            handler.usage_info()
        } else {
            UsageInfo {
                description: Some("Select a widget".into()),
                lines: vec![
                    UsageInfoLine {
                        keys: vec!["q".into()],
                        description: "to exit".into(),
                    },
                    UsageInfoLine {
                        keys: vec!["←".into(), "→".into()],
                        description: "to hover widgets".into(),
                    },
                    UsageInfoLine {
                        keys: vec!["e".into()],
                        description: format!(
                            "to activate {}",
                            self.get_component_for_section(&self.currently_hovered_section)
                                .name()
                        ),
                    },
                ],
            }
        }
    }
}
