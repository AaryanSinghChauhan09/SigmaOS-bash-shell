//! alias module

use anyhow::Result;
use log::debug;

pub struct alias;

impl alias {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating alias");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing alias");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down alias");
        Ok(())
    }
}
