use std::sync::OnceLock;

use serde::Deserialize;

/**
 * Defines the dot env content and causes an error during loading it if somethings missing
 */
#[derive(Deserialize, Debug)]
pub struct DotEnvSchema {
    /// The port of the application
    #[serde()]
    pub port: u16,
    /// The host of the application
    #[serde()]
    pub host: String,
    // The git storage path
    #[serde()]
    pub git_storage_path: String,
}

/// The config containing the env as a once cell
static CONFIG: OnceLock<DotEnvSchema> = OnceLock::new();

/**
 * Returns the env but only loads from file once
 */
pub fn environment() -> Result<&'static DotEnvSchema, anyhow::Error> {
    dotenvy::dotenv().ok();
    let env: DotEnvSchema = envy::from_env()?;

    Ok(CONFIG.get_or_init(|| env))
}
