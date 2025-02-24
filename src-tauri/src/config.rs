use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use serde::{Deserialize, Serialize};
use tokio::{fs, sync::RwLock};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    theme: Theme,
    place_id: Arc<String>,
    universe_id: Arc<String>,
    version_number: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Default,
    Light,
    Dark,
    Hotdog,
}

const AUTHOR_NAME: &str = "dekkonot";
const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const CONFIG_FILE_NAME: &str = "config.json";

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::config_local_dir()
        .expect("all systems should have a HOME env var set")
        .join(AUTHOR_NAME)
        .join(PROJECT_NAME)
        .join(CONFIG_FILE_NAME)
});

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    RwLock::new(Config {
        theme: Theme::Default,
        place_id: Arc::new(String::new()),
        universe_id: Arc::new(String::new()),
        version_number: Arc::new(String::new()),
    })
});

/// Creates a directory for storing the config file on the disk.
async fn make_config_dir() -> Result<()> {
    let mut path = dirs::config_local_dir().expect("all systems should have a HOME env var set");
    match fs::try_exists(&path).await {
        Ok(true) => {}
        Ok(false) => {
            panic!(
                "your system does not have a directory present at {}",
                path.display()
            );
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    path.push(AUTHOR_NAME);
    path.push(PROJECT_NAME);
    if let Err(e) = fs::create_dir_all(&path).await {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e.into());
        }
    }

    Ok(())
}

/// Loads a configuration file from the disk if one exists.
pub async fn load_config() -> Result<()> {
    if let Ok(true) = fs::try_exists(&*CONFIG_PATH).await {
        let serialized = fs::read(&*CONFIG_PATH).await?;
        *CONFIG.write().await = serde_json::from_slice(&serialized)?;
    }

    Ok(())
}

/// Serializes the currently loaded config file to the disk.
pub async fn save_config() -> Result<()> {
    make_config_dir().await?;
    let contents = serde_json::to_vec(&*CONFIG.read().await)?;
    fs::write(&*CONFIG_PATH, contents).await?;
    Ok(())
}

/// Returns the config
pub async fn get_config() -> Config {
    let lock = CONFIG.read().await;
    Config {
        theme: lock.theme,
        place_id: Arc::clone(&lock.place_id),
        universe_id: Arc::clone(&lock.universe_id),
        version_number: Arc::clone(&lock.version_number),
    }
}

pub async fn set_config(
    theme: Option<Theme>,
    place_id: Option<String>,
    universe_id: Option<String>,
    version_number: Option<String>,
) -> Result<()> {
    let mut lock = CONFIG.write().await;

    if let Some(theme) = theme {
        lock.theme = theme;
    }
    if let Some(place_id) = place_id {
        lock.place_id = Arc::new(place_id);
    }
    if let Some(universe_id) = universe_id {
        lock.universe_id = Arc::new(universe_id);
    }
    if let Some(version_number) = version_number {
        lock.version_number = Arc::new(version_number);
    }
    drop(lock);

    save_config().await
}
