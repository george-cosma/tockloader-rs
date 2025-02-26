pub struct BoardSettings {
    pub arch: Option<String>,
    pub start_address: u64,
}

impl Default for BoardSettings {
    fn default() -> Self {
        Self {
            arch: None,
            start_address: 0x30000,
        }
    }
}
