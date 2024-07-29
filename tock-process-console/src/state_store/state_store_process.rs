// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use super::{Action, State};
use crate::{
    board::{connection::ConnectionHandler, event::Event},
    termination::{Interrupted, Terminator},
};
use bytes::Bytes;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

type ConnectionHandle = (UnboundedReceiver<Event>, UnboundedSender<Bytes>);

pub struct StateStore {
    state_sender: UnboundedSender<State>,
}

impl StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_sender, state_receiver) = mpsc::unbounded_channel::<State>();

        (StateStore { state_sender }, state_receiver)
    }

    // Will only return in case of interruption
    pub async fn main_loop(
        self,
        mut terminator: Terminator,
        mut action_receiver: UnboundedReceiver<Action>,
        mut intrerrupt_receiver: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        let mut connection_handle: Option<ConnectionHandle> = None;
        let mut state = State::default();

        self.state_sender.send(state.clone())?;

        let result: Interrupted = loop {
            // Check if we have a board connection
            if let Some((ref mut event_receiver, ref command_writer)) = connection_handle {
                // Treat after connection related actions
                tokio::select! {
                    maybe_event = event_receiver.recv() => {
                        match maybe_event {
                            Some(event) => {
                            state.handle_board_event(&event);
                            },
                            None => {
                                connection_handle = Option::None;
                                state = State::default();
                            }
                        }
                    },
                    Some(action) = action_receiver.recv() => match action {
                        Action::SendMessage { content } => {
                            if !state.active_apps.is_empty() {
                                command_writer.send(
                                    content
                                ).expect("Expected command reader to be open.");
                            }
                        },
                        Action::AddScreen { screen_idx } => {
                            state.active_apps.push((screen_idx, None))
                        }
                        Action::SelectApplication { screen_idx, app_name } => {
                            state.try_set_active_room(screen_idx, app_name.as_str());
                        },
                        Action::Exit => {
                            let _ = terminator.terminate(Interrupted::UserRequest);

                            break Interrupted::UserRequest;
                        }
                        _ => {}
                    }
                }
            } else {
                // We are not connected to the board
                // Treat before connection related actions
                tokio::select! {
                    Some(action) = action_receiver.recv() => {

                        match action {
                            // We received an action to connect to the board at a given port address
                            Action::ConnectToBoard { port } => {
                                state.mark_connection_request_start();

                                // Emit event to re-render any part depending on the connection status
                                self.state_sender.send(state.clone())?;

                                match connect_to_board(&port).await {
                                    Ok(connection_result) => {
                                        connection_handle = Some(connection_result);
                                        state.process_connection_request_result(Ok(port));
                                    },
                                    Err(err) => {
                                        state.process_connection_request_result(Err(err));
                                    }
                                }
                            },
                            Action::Exit => {
                                let _ = terminator.terminate(Interrupted::UserRequest);

                                break Interrupted::UserRequest;
                            }
                            _ => {},
                        }
                    },
                    // Catch and handle interrupt signal to gracefully shutdown
                    Ok(interrupted) = intrerrupt_receiver.recv() => {
                        break interrupted;
                    }
                }
            }

            // Just modified the state object as per action received
            // so we send the new state to the UI
            self.state_sender.send(state.clone())?;
        };

        Ok(result)
    }
}

async fn connect_to_board(tty: &str) -> anyhow::Result<ConnectionHandle> {
    match ConnectionHandler::connection_init(tty).await {
        Ok((event_reader, command_writer)) => Ok((event_reader, command_writer)),
        Err(err) => Err(err),
    }
}
