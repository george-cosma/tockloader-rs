pub enum KnownBoardNames {
    NucleoF4,
    MicrobitV2,
}

impl KnownBoardNames {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "nucleo-f4" => Some(Self::NucleoF4),
            "microbit-v2" => Some(Self::MicrobitV2),
            _ => None,
        }
    }
}
