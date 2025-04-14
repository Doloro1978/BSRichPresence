mod schema;
use tracing::info;

pub async fn create_config() -> Result<(), ()> {
    let default_config = schema::RichPresenceConfig::default();

    let toml_config = toml::to_string(&default_config).unwrap();
    info!("{:#?}", toml_config);

    Ok(())
}
