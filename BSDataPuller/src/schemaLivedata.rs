use futures_util::StreamExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BSLiveData {
    // Score properties
    pub score: i32,
    pub score_with_multipliers: i32,
    //    pub max_score: i32,
    //    pub max_score_with_multipliers: i32,
    pub rank: String,
    //    pub full_combo: bool,
    //    pub notes_spawned: i32,
    pub combo: i32,
    pub misses: i32,
    pub accuracy: f64,
    //    pub block_hit_score: SBlockHitScore,
    pub player_health: f64,
    //    pub color_type: ColorType,
    //    pub cut_direction: NoteCutDirection,

    // Misc properties
    pub time_elapsed: i32,
    //    pub event_trigger: ELiveDataEventTriggers,
}

impl BSLiveData {
    pub async fn get() -> Result<BSLiveData, ()> {
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

        // Transform into WebSocket
        match response.into_websocket().await {
            Ok((mut ws)) => {
                println!("Successfully connected to WebSocket.");

                // Read the initial message from the WebSocket
                if let Some(Ok(text)) = ws.next().await {
                    if let Message::Text(awa) = text {
                        //print!("{awa}");
                        let bs_data: BSLiveData = serde_json::from_str(&awa).unwrap();
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
