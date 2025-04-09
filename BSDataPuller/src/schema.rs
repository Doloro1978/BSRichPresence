use futures_util::StreamExt;

use reqwest::*;
use reqwest_websocket::*;
use serde::{Deserialize, Serialize};

//use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BSMetadata {
    pub GameVersion: String,
    pub PluginVersion: String,
    pub InLevel: bool,
    pub LevelPaused: bool,
    pub LevelFinished: bool,
    pub LevelFailed: bool,
    pub LevelQuit: bool,
    pub Hash: Option<String>,
    pub LevelID: Option<String>,
    pub SongName: String,
    pub SongSubName: String,
    pub SongAuthor: String,
    pub Mapper: String,
    pub Mappers: Vec<String>,
    pub Lighters: Vec<String>,
    pub ContentRating: String,
    pub BSRKey: Option<String>,
    pub CoverImage: Option<String>,
    pub Duration: u32,
    pub MapType: String,
    pub Environment: String,
    pub Difficulty: String,
    pub CustomDifficultyLabel: Option<String>,
    pub BPM: f32,
    pub NJS: f32,
    pub Modifiers: Modifiers,
    pub ModifiersMultiplier: f32,
    pub PracticeMode: bool,
    pub PracticeModeModifiers: PracticeModeModifiers,
    pub PP: f32,
    pub Star: f32,
    pub RankedState: RankedState,
    pub Rating: Option<f32>,
    pub ColorScheme: ColorScheme,
    pub IsMultiplayer: bool,
    pub MultiplayerLobbyMaxSize: u32,
    pub MultiplayerLobbyCurrentSize: u32,
    pub PreviousRecord: u32,
    pub PreviousBSR: Option<String>,
    pub UnixTimestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modifiers {
    pub NoFailOn0Energy: bool,
    pub OneLife: bool,
    pub FourLives: bool,
    pub NoBombs: bool,
    pub NoWalls: bool,
    pub NoArrows: bool,
    pub GhostNotes: bool,
    pub DisappearingArrows: bool,
    pub SmallNotes: bool,
    pub ProMode: bool,
    pub StrictAngles: bool,
    pub ZenMode: bool,
    pub SlowerSong: bool,
    pub FasterSong: bool,
    pub SuperFastSong: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PracticeModeModifiers {
    pub SongSpeedMul: f32,
    pub StartInAdvanceAndClearNotes: bool,
    pub SongStartTime: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RankedState {
    pub Ranked: bool,
    pub Qualified: bool,
    pub BeatleaderQualified: bool,
    pub ScoresaberQualified: bool,
    pub BeatleaderRanked: bool,
    pub ScoresaberRanked: bool,
    pub BeatleaderStars: f32,
    pub ScoresaberStars: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorScheme {
    pub SaberAColor: Option<Color>,
    pub SaberBColor: Option<Color>,
    pub ObstaclesColor: Option<Color>,
    pub EnvironmentColor0: Option<Color>,
    pub EnvironmentColor1: Option<Color>,
    pub EnvironmentColor0Boost: Option<Color>,
    pub EnvironmentColor1Boost: Option<Color>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub HexCode: String,
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Alpha: f32,
}

impl BSMetadata {
    pub async fn get() -> Result<BSMetadata> {
        let client = Client::default();

        // Attempt to upgrade to WebSocket
        let response = client
            .get("ws://127.0.0.1:2946/BSDataPuller/MapData")
            .upgrade()
            .send()
            .await
            .map_err(|e| {
                eprintln!("Failed to send WebSocket upgrade request: {}", e);
                e
            })
            .unwrap();

        // Transform into WebSocket
        match response.into_websocket().await {
            Ok((mut ws)) => {
                println!("Successfully connected to WebSocket.");

                // Read the initial message from the WebSocket
                if let Some(Ok(text)) = ws.next().await {
                    if let Message::Text(awa) = text {
                        //print!("{awa}");
                        let bs_data: BSMetadata = serde_json::from_str(&awa).unwrap();
                        return Ok(bs_data);
                    }
                    Err(Err("").unwrap())
                } else {
                    eprintln!("No data received on initial connection.");
                    Err(Err("No data.").unwrap())
                }
                //Ok(())
            }
            Err(err) => {
                eprintln!("Failed to establish WebSocket connection: {}", err);
                Err(err).unwrap()
            }
        }
    }
}
