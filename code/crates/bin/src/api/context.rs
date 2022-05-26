use serde::{Deserialize, Serialize};

use crate::api::settings::Settings;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings
        }
    }
}