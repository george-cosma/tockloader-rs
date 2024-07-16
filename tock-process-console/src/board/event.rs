#[derive(Debug, Clone)]
pub struct NewMessageEvent {
    pub app: String,
    pub pid: u8,
    pub is_app: bool,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Event {
    NewMessage(NewMessageEvent),
    LostConnection(String),
}
