/*
    Appellation: power <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::ValueEnum;
use scsys::prelude::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Power {
    On,
    Off,
}

impl Power {
    pub fn handler(&self, catalyst: fn(Self) -> BoxResult) -> BoxResult {
        match self {
            Self::On => catalyst(self.clone()),
            Self::Off => catalyst(self.clone()),
        }
    }
}
