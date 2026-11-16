//! renderer module

use anyhow::Result;
use log::debug;

pub struct renderer;

impl renderer {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating renderer");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing renderer");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down renderer");
        Ok(())
    }
}
