// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::state_store::{Action, State};
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

pub trait Component {
    fn new(
        state: &State,
        screen_idx: Option<usize>,
        action_sender: UnboundedSender<Action>,
    ) -> Self
    where
        Self: Sized;

    fn update_with_state(self, state: &State) -> Self
    where
        Self: Sized;

    fn handle_key_event(&mut self, key: KeyEvent);

    fn handle_mouse_event(&mut self, event: MouseEvent);
}

pub trait ComponentRender<Properties> {
    fn render(&mut self, frame: &mut Frame, properties: Properties);
}
