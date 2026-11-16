//! seat module

use anyhow::Result;
use log::debug;

pub struct seat;

impl seat {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating seat");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing seat");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down seat");
        Ok(())
    }
}
