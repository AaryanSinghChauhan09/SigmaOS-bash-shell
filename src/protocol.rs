//! protocol module

use anyhow::Result;
use log::debug;

pub struct protocol;

impl protocol {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating protocol");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing protocol");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down protocol");
        Ok(())
    }
}
