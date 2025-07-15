use async_trait::async_trait;

use crate::attributes::app_attributes::AppAttributes;
use crate::board_settings::BoardSettings;
use crate::connection::{Connection, ProbeRSConnection};
use crate::errors::TockloaderError;
use crate::CommandList;

#[async_trait]
impl CommandList for ProbeRSConnection {
    async fn list(
        &mut self,
        settings: &BoardSettings,
    ) -> Result<Vec<AppAttributes>, TockloaderError> {
        if !self.is_open() {
            return Err(TockloaderError::ConnectionNotOpen);
        }
        let session = self.session.as_mut().expect("Board must be open");

        let mut core = session
            .core(self.target_info.core)
            .map_err(|e| TockloaderError::CoreAccessError(self.target_info.core, e))?;

        AppAttributes::read_apps_data_probe(&mut core, settings.start_address)
    }
}
