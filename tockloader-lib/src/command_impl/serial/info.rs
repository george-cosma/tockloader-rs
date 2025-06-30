use std::time::Duration;

use async_trait::async_trait;

use crate::attributes::app_attributes::AppAttributes;
use crate::attributes::general_attributes::GeneralAttributes;
use crate::attributes::system_attributes::SystemAttributes;
use crate::board_settings::BoardSettings;
use crate::bootloader_serial::{ping_bootloader_and_wait_for_response, Response};
use crate::connection::{Connection, SerialConnection};
use crate::errors::TockloaderError;
use crate::CommandInfo;

#[async_trait]
impl CommandInfo for SerialConnection {
    async fn info(
        &mut self,
        settings: &BoardSettings,
    ) -> Result<GeneralAttributes, TockloaderError> {
        if !self.is_open() {
            return Err(TockloaderError::ConnectionNotOpen);
        }
        let stream = self.stream.as_mut().expect("Board must be open");

        let response = ping_bootloader_and_wait_for_response(stream).await?;

        if response as u8 != Response::Pong as u8 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let _ = ping_bootloader_and_wait_for_response(stream).await?;
        }

        let system_attributes = SystemAttributes::read_system_attributes_serial(stream).await?;
        let app_attributes =
            AppAttributes::read_apps_data_serial(stream, settings.start_address).await?;

        Ok(GeneralAttributes::new(system_attributes, app_attributes))
    }
}
