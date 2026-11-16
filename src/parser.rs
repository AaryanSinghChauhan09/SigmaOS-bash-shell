//! parser module

use anyhow::Result;
use log::debug;

pub struct parser;

impl parser {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating parser");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing parser");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down parser");
        Ok(())
    }
}
