//! prompt module

use anyhow::Result;
use log::debug;

pub struct prompt;

impl prompt {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating prompt");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing prompt");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down prompt");
        Ok(())
    }
}
