//! env module

use anyhow::Result;
use log::debug;

pub struct env;

impl env {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating env");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing env");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down env");
        Ok(())
    }
}
