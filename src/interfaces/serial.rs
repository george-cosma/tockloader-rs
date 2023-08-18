use crate::errors::TockloaderError;
use super::board_interface::BoardInterface;

pub struct SerialInterface {}

impl BoardInterface for SerialInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
