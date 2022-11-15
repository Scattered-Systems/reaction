/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::components::{logging::Logger, networking::Server};
use scsys::prelude::config::{Config, Environment, File, FileSourceFile, FileFormat};
use scsys::prelude::{BoxResult, ConfigResult, Configurable,};
use serde::{Deserialize, Serialize};

pub fn collect_configs(pat: &str, required: bool) -> BoxResult<Vec<File<FileSourceFile, FileFormat>>> {
    let res = glob::glob(pat)?
        .map(|path| File::from(path.ok().unwrap()).required(required))
        .collect::<Vec<_>>();
    Ok(res)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AppSettings {
    pub mode: String,
    pub name: String,
}

impl AppSettings {
    pub fn name(&mut self, name: Option<&str>) -> &Self {
        self.name = match name {
            Some(v) => v.to_string(),
            None => self.name.clone(),
        };

        self
    }
    pub fn slug(&self) -> String {
        self.name.clone().to_lowercase()
    }
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub application: Option<AppSettings>,
    pub logger: Option<Logger>,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"));

        match collect_configs("**/.config/*.toml", false) {
            Err(_) => {},
            Ok(v) => {builder = builder.add_source(v)}
        };
        builder = builder.add_source(Environment::default().separator("__"));
        
        match std::env::var("RUST_LOG") {
            Err(_) => {},
            Ok(v) => {builder = builder.set_override("logger.level", Some(v))?;}
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {},
            Ok(v) => {builder = builder.set_override("server.port", v)?;}
        };

        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(v) => v,
            Err(_) => Self {
                application: Some(AppSettings::default()),
                logger: Some(Logger::default()),
                server: Server::default()
            },
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
