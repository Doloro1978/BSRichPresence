use serde::{Deserialize, Serialize};

// Config options

// Select leaderboard to pull star ratings from
// If we are allowed to contact beatsaver for cover images
// ect ect.

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RichPresenceConfig {
    pub maps: Map,
    pub ranked: Ranked,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Map {
    pub fetch_from_beatsaver: bool,
    pub beatsaver_button: bool,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Leaderboards {
    #[default]
    BeatLeader,
    ScoreSaber,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ranked {
    pub prefered_leaderboard: Leaderboards, // Select the leaderboard to pull star ratings from.
}
