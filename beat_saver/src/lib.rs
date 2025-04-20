use reqwest::Client;
mod schema;
use crate::schema::RawBeatSaverMap;

struct BeatSaver {
    client: Client,
}

struct BeatSaverMap {
    // I would make a more "usable" version of the RawBeatSaverMap struct here.. but egh.
}

impl BeatSaver {
    pub fn create_from(client: Client) -> BeatSaver {
        BeatSaver { client }
    }
    pub async fn get_map(self, hash: String) -> Result<RawBeatSaverMap, ()> {
        let get_string = "https://api.beatsaver.com/maps/hash/".to_owned() + &hash;
        let response = self.client.get(get_string).send().await.unwrap();
        if let Ok(text_response) = response.text().await {
            let parsed_response: RawBeatSaverMap = serde_json::from_str(&text_response).unwrap();
            return Ok(parsed_response);
        } else {
            return Err(());
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
