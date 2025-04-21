// Used to post process data from BSData into something useable for rich presence using a provided
// config

use BSDataPuller::BSData;
use BSDataPuller::LevelState;
use config::schema::Leaderboards;
use config::schema::RichPresenceConfig;

#[derive(Clone, Debug)]
pub struct ProcessedLevelData {
    pub state: LevelState,
    pub hash: String,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author: String,
    pub cover_image: String,
    // Ranked Data
    pub stars: f32,
    pub qualified: bool,
    pub ranked: bool,
}

pub struct BSProcessedData {
    pub level_data: Option<ProcessedLevelData>,
}

pub trait Processing {
    async fn process(&self, config: &RichPresenceConfig) -> BSProcessedData;
}

impl Processing for BSData {
    async fn process(&self, config: &RichPresenceConfig) -> BSProcessedData {
        let lvl_data = self.levelData.clone();
        let lvl_data = lvl_data.lock().await;
        match lvl_data.LevelDataInner.clone() {
            Some(level_data) => BSProcessedData {
                level_data: Some(ProcessedLevelData {
                    state: level_data.State,
                    hash: level_data.Hash,
                    song_name: level_data.SongName,
                    song_sub_name: level_data.SongSubName,
                    song_author: level_data.SongAuthor,
                    cover_image: {
                        if level_data.CoverImage.len() > 100 {
                            // CoverImage url is invalid (it returned a base64 data)
                            "https://upload.wikimedia.org/wikipedia/commons/5/5a/Black_question_mark.png".to_owned()
                        } else {
                            level_data.CoverImage
                        }
                    },
                    stars: {
                        match config.ranked.prefered_leaderboard {
                            Leaderboards::BeatLeader => level_data.RankedData.bl_stars,
                            Leaderboards::ScoreSaber => level_data.RankedData.ss_stars,
                        }
                    },
                    ranked: {
                        match config.ranked.prefered_leaderboard {
                            Leaderboards::BeatLeader => level_data.RankedData.bl_ranked,
                            Leaderboards::ScoreSaber => level_data.RankedData.ss_ranked,
                        }
                    },
                    qualified: {
                        match config.ranked.prefered_leaderboard {
                            Leaderboards::BeatLeader => level_data.RankedData.bl_qualified,
                            Leaderboards::ScoreSaber => level_data.RankedData.ss_qualified,
                        }
                    },
                }),
            },
            None => BSProcessedData { level_data: None },
        }
    }
}
