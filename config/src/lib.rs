mod schema;
use dirs::config_dir;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use tracing::error;
use tracing::info;
use tracing::warn;

pub async fn create_config() -> Result<File, anyhow::Error> {
    let config_path = get_config_file_path().await;
    let config_dir = {
        let mut a = config_path.clone();
        a.pop();
        a
    };
    if !config_dir.exists() {
        warn!("Creating config dir at {:#?}", &config_dir);
        fs::create_dir(config_dir)?
    }
    if !config_path.exists() {
        warn!("Creating config file at {:#?}", &config_path);
        fs::File::create_new(&config_path)?;
    }

    info!("{:#?}", config_path);
    let file = fs::File::options()
        .write(true)
        .read(true)
        .open(&config_path)?;
    return Ok(file);
}

// get full file path for the toml config
pub async fn get_config_file_path() -> PathBuf {
    let mut config_dir = config_dir().unwrap();
    config_dir.push("bs_richpresence");
    config_dir.push("config.toml");
    config_dir
}

pub async fn config_init() -> Result<schema::RichPresenceConfig, anyhow::Error> {
    let mut hihi = create_config().await?;
    let mut hihi_string = String::new();
    hihi.read_to_string(&mut hihi_string).unwrap();
    //info!("Config ");
    if hihi_string.is_empty() {
        info!("touch2");
        let default_config = schema::RichPresenceConfig::default();
        let toml_config = toml::to_string(&default_config)?;
        info!("touch3");
        hihi.write(&toml_config.into_bytes())?;
        hihi.flush()?;
        return Ok(default_config);
    }
    let toml_hihi = toml::from_str(&hihi_string);
    if let Err(toml_hihi) = toml_hihi {
        error!("{:#?}", toml_hihi);
        warn!("Returning default config...");
        return Ok(schema::RichPresenceConfig::default());
    } else {
        return Ok(toml_hihi.unwrap());
    }
}
