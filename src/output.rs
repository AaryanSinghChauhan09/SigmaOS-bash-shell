//! output module

use anyhow::Result;
use log::debug;

pub struct output;

impl output {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating output");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing output");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down output");
        Ok(())
    }
}
