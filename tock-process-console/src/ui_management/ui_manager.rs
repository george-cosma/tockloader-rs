// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::{
    state_store::{Action, State},
    termination::Interrupted,
    ui_management::{
        components::{Component, ComponentRender},
        pages::AppRouter,
    },
};
use anyhow::Context;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    fs::OpenOptions,
    io::{self, Stdout, Write},
};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tokio_stream::StreamExt;

pub struct UiManager {
    action_writer: UnboundedSender<Action>,
}

impl UiManager {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_writer, action_reader) = mpsc::unbounded_channel();

        (Self { action_writer }, action_reader)
    }

    pub async fn main_loop(
        self,
        mut state_reader: UnboundedReceiver<State>,
        mut interrupt_reader: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        let mut app_router = {
            let state = state_reader.recv().await.unwrap();

            AppRouter::new(&state, None, self.action_writer.clone())
        };

        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();

        let result: anyhow::Result<Interrupted> = loop {
            tokio::select! {
                // Handle state updates
                Some(state) = state_reader.recv() => {
                    app_router = app_router.update_with_state(&state);
                },
                // Handle interruption signal
                Ok(interrupted) = interrupt_reader.recv() => {
                    break Ok(interrupted);
                },
                result = crossterm_events.next() => {
                    match result {
                        Some(Ok(Event::Key(key))) => {
                            app_router.handle_key_event(key);
                        },
                        Some(Ok(Event::Mouse(event))) => {
                            app_router.handle_mouse_event(event);
                        }
                        None => break Ok(Interrupted::UserRequest),
                        _ => (),
                    }
                }
            }

            if let Err(err) = terminal
                .draw(|frame| app_router.render(frame, ()))
                .context("Could not render the terminal")
            {
                break Err(err);
            }
        };

        let _ = close_terminal(&mut terminal);

        result
    }
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn close_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(terminal.show_cursor()?)
}
