//! input module

use anyhow::Result;
use log::debug;

pub struct input;

impl input {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating input");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing input");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down input");
        Ok(())
    }
}
