use std::path::PathBuf;

use clap::Parser;

use crate::config::{Application, Config};

/// Program that renders 3d objects
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Flags {
    /// Config file to use
    #[arg(short, long)]
    pub config: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    FailedToReadFile(Box<dyn std::error::Error>),
    FailedToParse(ron::de::SpannedError),
}

impl Flags {
    pub fn get_application(self) -> Result<Application, Error> {
        let raw_config = std::fs::read_to_string(self.config)
            .map_err(|v| Error::FailedToReadFile(Box::new(v)))?;

        Ok(ron::from_str::<Config>(&raw_config)
            .map_err(Error::FailedToParse)?
            .process())
    }
}
