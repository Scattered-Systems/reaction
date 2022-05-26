use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub mode: String,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Provider {
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub port: u16,
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View the bin locally at http://localhost:{}", self.port)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub provider: Provider,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("application.mode", "development")?
            .set_default("application.name", "Application")?
            .set_default("application.slug", "application")?
            .set_default("database.name", "application")?
            .set_default("database.uri", "redis://localhost:6379")?
            .set_default("logger.level", "info")?
            .set_default("provider.endpoint", "https://rpc.ankr.com/eth")?
            .set_default("server.port", 8080)?;

        builder = builder.add_source(glob("**/*.config.*")
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(Environment::default().separator("__"));

        if let Ok(port) = std::env::var("PORT") {
            builder = builder
                .set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}