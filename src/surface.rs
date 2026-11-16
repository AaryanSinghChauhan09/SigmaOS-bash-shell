//! surface module

use anyhow::Result;
use log::debug;

pub struct surface;

impl surface {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating surface");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing surface");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down surface");
        Ok(())
    }
}
