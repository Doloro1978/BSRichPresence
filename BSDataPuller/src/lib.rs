pub mod schema;
pub mod schemaLivedata;
pub mod thread;
use crate::schema::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct BSData {
    pub gameData: Arc<Mutex<GameData>>,
    pub levelData: Arc<Mutex<LevelData>>,
    pub gamerunning: Arc<Mutex<u64>>,
}

#[derive(Debug)]
pub struct GameData {
    GameVersion: String,
    PluginVersion: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelState {
    Playing,
    Paused,
    Finished,
    Failed,
    Quit,
}

#[derive(Debug, Clone)]
pub struct LevelDataInner {
    pub State: LevelState,
    Hash: String,
    pub SongName: String,
    pub SongSubName: String,
    pub SongAuthor: String,
    Mapper: String,
    pub CoverImage: String,
    pub Star: f32,
    pub Diff: String,
}

#[derive(Debug)]
pub struct LevelData {
    pub LevelDataInner: Option<LevelDataInner>,
}

impl LevelData {
    pub fn overwrite_leveldata(&mut self, replacement_data: LevelDataInner) {
        self.LevelDataInner = Some(replacement_data)
    }
    pub fn update_state(&mut self, replacement_state: LevelState) {
        let build = LevelDataInner {
            State: replacement_state,
            ..self.LevelDataInner.as_ref().unwrap().clone()
        };
        self.LevelDataInner = Some(build);
    }
}

impl LevelDataInner {
    pub fn write(&mut self, awa: LevelDataInner) {
        self.SongName = awa.SongName;
        self.SongAuthor = awa.SongAuthor;
        self.CoverImage = awa.CoverImage;
    }
}

pub struct refreshBSData {
    Data: Arc<Mutex<BSData>>,
}
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio::time::sleep;
use tracing::info;
impl BSData {
    pub async fn is_game_running(&self) -> bool {
        let lastMsgTimestamp = self.gamerunning.clone().lock().await.clone();

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        if lastMsgTimestamp < (since_the_epoch.as_secs() + 100) {
            info!(lastMsgTimestamp);
            info!("{}", since_the_epoch.as_secs() + 100);
            sleep(Duration::from_secs(1)).await;
            return false;
        } else {
            return true;
        }
    }
    //pub fn
    pub fn from_raw(data: BSMetadata) -> BSData {
        //info!(U);
        let gameData = GameData {
            GameVersion: data.GameVersion,
            PluginVersion: data.PluginVersion,
        };
        let mut levelData = LevelData {
            LevelDataInner: None,
        };
        //print!(
        //    "{}, {}, {}, {}",
        //    data.InLevel, data.LevelFinished, data.LevelPaused, data.LevelQuit
        //);
        if data.InLevel {
            levelData = LevelData {
                LevelDataInner: Some(LevelDataInner {
                    SongName: data.SongName,
                    // add the rest
                    CoverImage: data.CoverImage.unwrap(),
                    SongSubName: data.SongSubName,
                    SongAuthor: data.SongAuthor,
                    Hash: data.Hash.unwrap(),
                    State: {
                        if data.InLevel {
                            LevelState::Playing
                        } else {
                            LevelState::Quit
                        }
                    },
                    Mapper: data.Mapper,
                    Star: data.RankedState.BeatleaderStars,
                    Diff: {
                        if let Some(DiffLabel) = data.CustomDifficultyLabel {
                            DiffLabel
                        } else {
                            data.Difficulty
                        }
                    },
                }),
            };
        }

        BSData {
            gameData: Arc::new(Mutex::new(gameData)),
            levelData: Arc::new(Mutex::new(levelData)),
            gamerunning: Arc::new(Mutex::new(data.UnixTimestamp)),
        }
    }
}
