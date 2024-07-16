use circular_queue::CircularQueue;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    sync::{Arc, Mutex},
};
use tui_term::vt100::Parser;

use crate::board;

/// AppData hold information about an application that is running on the board
/// and intercepted by the console application
#[derive(Debug, Clone)]
pub struct AppData {
    // Name of the application running on the board
    pub name: String,
    // Process ID of the application running on the board
    pub pid: u64,
    ///
    pub is_app: bool,
    // TODO: de vazut daca trebuie sa avem altceva in loc de string, sau daca nu avem de fapt nevoie sa fie o lista circulara
    pub logs: CircularQueue<Vec<u8>>,
    // Flag to indicate wheter there are new, unwritten logs for this app
    pub has_new_logs: bool,
}
const MAX_LOGS_TO_STORE_PER_APP: usize = 1000;

impl Default for AppData {
    fn default() -> Self {
        AppData {
            name: String::from("kernel"),
            pid: 0,
            is_app: false,
            logs: CircularQueue::with_capacity(MAX_LOGS_TO_STORE_PER_APP),
            has_new_logs: false,
        }
    }
}

impl AppData {
    fn new(name: String, pid: u64, is_app: bool) -> Self {
        AppData {
            name,
            pid,
            is_app,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoardConnectionStatus {
    Uninitialized,
    Connecting,
    Connected { port: String },
    Errored { err: String },
}

/// State struct is holding the state of the entire application
#[derive(Clone)]
pub struct State {
    pub board_connection_status: BoardConnectionStatus,
    pub active_apps: Vec<(usize, Option<String>)>,
    // TODO: should be changed
    pub apps_data_map: HashMap<String, AppData>,
    pub apps_parsers_map: HashMap<String, Arc<Mutex<Parser>>>,
}

impl Default for State {
    fn default() -> Self {
        let mut default_apps: HashMap<String, AppData> = HashMap::new();
        default_apps.insert("kernel".to_string(), AppData::default());

        let mut default_parsers: HashMap<String, Arc<Mutex<Parser>>> = HashMap::new();
        default_parsers.insert(
            "kernel".to_string(),
            Arc::new(Mutex::new(Parser::new(40, 40, 100))),
        );

        State {
            board_connection_status: BoardConnectionStatus::Uninitialized,
            active_apps: Vec::new(),
            apps_data_map: default_apps,
            apps_parsers_map: default_parsers,
        }
    }
}

impl State {
    /// Handle events from the board
    pub fn handle_board_event(&mut self, event: &board::event::Event) {
        // TODO: should cover more types of events
        match event {
            board::event::Event::NewMessage(event) => {
                if !self.apps_data_map.contains_key(&event.app) {
                    let new_app_data =
                        AppData::new(event.app.clone(), event.pid as u64, event.is_app);
                    self.apps_data_map.insert(event.app.clone(), new_app_data);
                    self.apps_parsers_map.insert(
                        event.app.clone(),
                        Arc::new(Mutex::new(Parser::new(4, 4, 100))),
                    );
                }

                let app_data = self.apps_data_map.get_mut(&event.app).unwrap();
                app_data.logs.push(event.payload.clone());

                let app_parser = self.apps_parsers_map.get(&event.app).unwrap();
                app_parser.lock().unwrap().process(&event.payload);

                let mut is_in_focus = false;
                for (_, active_app) in self.active_apps.clone() {
                    if let Some(active_app) = active_app {
                        if active_app.eq(&event.app) {
                            is_in_focus = true;
                        }
                    }
                }

                if !is_in_focus {
                    app_data.has_new_logs = true;
                }
            }
            board::event::Event::LostConnection(_err) => {
                todo!()
            }
        }
    }

    /// Marks the beggining of the connection process
    pub fn mark_connection_request_start(&mut self) {
        self.board_connection_status = BoardConnectionStatus::Connecting;
    }

    /// Processes the result of the connection request and updates the state accordingly
    pub fn process_connection_request_result(&mut self, result: anyhow::Result<String>) {
        self.board_connection_status = match result {
            Ok(port) => BoardConnectionStatus::Connected { port: port.clone() },
            Err(error) => BoardConnectionStatus::Errored {
                err: error.to_string(),
            },
        }
    }

    /// Is setting the currently active application that is visible in the UI
    /// In case of failure, the function will return
    pub fn try_set_active_room(&mut self, screen_idx: usize, app: &str) -> Option<&AppData> {
        let app_data = self.apps_data_map.get_mut(app)?;
        app_data.has_new_logs = false;

        if let Some(index) = self
            .active_apps
            .iter()
            .enumerate()
            .find(|(_index, (screen_index, _active_app))| screen_index == &screen_idx)
            .map(|(index, _)| index)
        {
            self.active_apps[index].1 = Some(String::from(app));
        } else {
            self.active_apps.push((screen_idx, Some(String::from(app))));
        }

        Some(app_data)
    }
}
