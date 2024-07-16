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

    fn name(&self) -> &str;

    fn handle_key_event(&mut self, key: KeyEvent);

    fn handle_mouse_event(&mut self, event: MouseEvent);
}

pub trait ComponentRender<Properties> {
    fn render(&self, frame: &mut Frame, properties: Properties);
}
