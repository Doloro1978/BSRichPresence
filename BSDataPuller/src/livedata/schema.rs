use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BSLivedata {
    #[serde(rename = "Score")]
    pub score: u32,
    #[serde(rename = "ScoreWithMultipliers")]
    score_with_multipliers: u32,
    #[serde(rename = "MaxScore")]
    max_score: u32,
    #[serde(rename = "MaxScoreWithMultipliers")]
    max_score_with_multipliers: u32,
    #[serde(rename = "Rank")]
    rank: String,
    #[serde(rename = "FullCombo")]
    full_combo: bool,
    #[serde(rename = "NotesSpawned")]
    notes_spawned: u32,
    #[serde(rename = "Combo")]
    combo: u32,
    #[serde(rename = "Misses")]
    misses: u32,
    #[serde(rename = "Accuracy")]
    accuracy: f64,
    #[serde(rename = "BlockHitScore")]
    block_hit_score: BlockHitScore,
    #[serde(rename = "PlayerHealth")]
    player_health: f64,
    #[serde(rename = "ColorType")]
    color_type: f32,
    #[serde(rename = "CutDirection")]
    cut_direction: f32,
    #[serde(rename = "TimeElapsed")]
    pub time_elapsed: u32,
    #[serde(rename = "EventTrigger")]
    event_trigger: f32,
    #[serde(rename = "UnixTimestamp")]
    unix_timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockHitScore {
    #[serde(rename = "PreSwing")]
    pre_swing: u32,
    #[serde(rename = "PostSwing")]
    post_swing: u32,
    #[serde(rename = "CenterSwing")]
    center_swing: u32,
}

use crate::BSData;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_websocket;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use reqwest_websocket::UpgradeResponse;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Mutex;
use tracing::debug;
use tracing::error;

impl BSLivedata {
    pub async fn connection() -> UpgradeResponse {
        let client = Client::default();
        // Attempt to upgrade to WebSocket
        let response = client
            .get("ws://127.0.0.1:2946/BSDataPuller/LiveData")
            .upgrade()
            .send()
            .await
            .map_err(|e| {
                eprintln!("Failed to send WebSocket upgrade request: {}", e);
                e
            })
            .unwrap();
        return response;
    }
    // starts 2 threads to update BSData
    pub async fn start() -> Arc<Mutex<Option<BSLivedata>>> {
        //info!(UNIX_EPOCH);
        //let mut gameData = self.gameData.clone();
        let level_data = Arc::new(Mutex::new(Some(BSLivedata::default())));

        //let levelData = &self.levelData;

        let level_data_ref = level_data.clone();
        // levelData upater
        tokio::spawn(async move {
            //let awa = self.clone();
            let response = BSLivedata::connection().await;

            let mut ws = response.into_websocket().await.unwrap();
            loop {
                debug!("Hit-msg-processing-loop");
                if let Some(Ok(msg)) = ws.next().await {
                    debug!("Hit-msg-processing-livedata");
                    if let Message::Text(msg) = msg {
                        let new = serde_json::from_str::<BSLivedata>(&msg).unwrap();
                        level_data_ref.clone().lock().await.replace(new);
                        //drop(level_data_ref)
                    } else {
                        error!("Unable to handle message : \n{:#?}", msg);
                        continue;
                    };
                } else {
                    error!("Unable to await websocket.. killing thread.");
                    break;
                }
            }
            //info!("Goodbye..");
        });
        return level_data;

        // do we need to update gameData?
    }
}
