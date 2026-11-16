//! executor module

use anyhow::Result;
use log::debug;

pub struct executor;

impl executor {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating executor");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing executor");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down executor");
        Ok(())
    }
}
