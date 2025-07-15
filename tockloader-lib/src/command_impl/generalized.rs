use async_trait::async_trait;

use crate::attributes::app_attributes::AppAttributes;
use crate::attributes::general_attributes::GeneralAttributes;
use crate::board_settings::BoardSettings;
use crate::connection::TockloaderConnection;
use crate::errors::TockloaderError;
use crate::tabs::tab::Tab;
use crate::{CommandInfo, CommandInstall, CommandList};

#[async_trait]
impl CommandList for TockloaderConnection {
    async fn list(
        &mut self,
        settings: &BoardSettings,
    ) -> Result<Vec<AppAttributes>, TockloaderError> {
        match self {
            TockloaderConnection::ProbeRS(conn) => conn.list(settings).await,
            TockloaderConnection::Serial(conn) => conn.list(settings).await,
        }
    }
}

#[async_trait]
impl CommandInfo for TockloaderConnection {
    async fn info(
        &mut self,
        settings: &BoardSettings,
    ) -> Result<GeneralAttributes, TockloaderError> {
        match self {
            TockloaderConnection::ProbeRS(conn) => conn.info(settings).await,
            TockloaderConnection::Serial(conn) => conn.info(settings).await,
        }
    }
}

#[async_trait]
impl CommandInstall for TockloaderConnection {
    async fn install_app(
        &mut self,
        settings: &BoardSettings,
        tab_file: Tab,
    ) -> Result<(), TockloaderError> {
        match self {
            TockloaderConnection::ProbeRS(conn) => conn.install_app(settings, tab_file).await,
            TockloaderConnection::Serial(conn) => conn.install_app(settings, tab_file).await,
        }
    }
}
