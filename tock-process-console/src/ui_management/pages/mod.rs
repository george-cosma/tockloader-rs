// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use self::{setup_page::SetupPage, terminal_page::main_page::MainPage};
use super::components::{Component, ComponentRender};
use crate::state_store::{Action, BoardConnectionStatus, State};
use tokio::sync::mpsc::UnboundedSender;

mod setup_page;
mod terminal_page;

enum ActivePage {
    SetupPage,
    MainPage,
}

impl From<&State> for ActivePage {
    fn from(state: &State) -> Self {
        match state.board_connection_status {
            BoardConnectionStatus::Connected { .. } => ActivePage::MainPage,
            _ => ActivePage::SetupPage,
        }
    }
}

pub struct AppRouter {
    active_page: ActivePage,
    setup_page: SetupPage,
    main_page: MainPage,
}

impl AppRouter {
    fn get_active_page_component(&self) -> &dyn Component {
        match self.active_page {
            ActivePage::SetupPage => &self.setup_page,
            ActivePage::MainPage => &self.main_page,
        }
    }

    fn get_active_page_component_mut(&mut self) -> &mut dyn Component {
        match self.active_page {
            ActivePage::SetupPage => &mut self.setup_page,
            ActivePage::MainPage => &mut self.main_page,
        }
    }
}

impl Component for AppRouter {
    fn new(
        state: &State,
        _screen_idx: Option<usize>,
        action_sender: UnboundedSender<Action>,
    ) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            active_page: ActivePage::from(state),
            setup_page: SetupPage::new(state, None, action_sender.clone()),
            main_page: MainPage::new(state, None, action_sender.clone()),
        }
    }

    fn name(&self) -> &str {
        self.get_active_page_component().name()
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        self.get_active_page_component_mut().handle_key_event(key)
    }

    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) {
        self.get_active_page_component_mut()
            .handle_mouse_event(event)
    }

    fn update_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            active_page: ActivePage::from(state),
            setup_page: self.setup_page.update_with_state(state),
            main_page: self.main_page.update_with_state(state),
        }
    }
}

impl ComponentRender<()> for AppRouter {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, properties: ()) {
        match self.active_page {
            ActivePage::SetupPage => self.setup_page.render(frame, properties),
            ActivePage::MainPage => self.main_page.render(frame, properties),
        }
    }
}
