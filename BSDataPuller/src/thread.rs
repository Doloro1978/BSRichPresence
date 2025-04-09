use crate::BSData;
use crate::BSMetadata;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use std::mem;
use tokio::spawn;
use tokio::sync;

impl BSData {
    // starts 2 threads to update BSData
    pub async fn start(&self) {
        //let mut gameData = self.gameData.clone();
        let mut levelData = self.levelData.clone();

        //let levelData = &self.levelData;

        // levelData updater
        tokio::spawn(async move {
            //let awa = self.clone();
            let levelData_lock = levelData.clone().to_owned();
            let client = Client::default();
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

            let mut ws = response.into_websocket().await.unwrap();
            loop {
                if let Some(Ok(msg)) = ws.next().await {
                    if let Message::Text(msg) = msg {
                        // TODO:PLEASE PLEASE PLEASE ADD ERROR HANDLING
                        let new =
                            BSData::from_raw(serde_json::from_str::<BSMetadata>(&msg).unwrap());
                        //let mut levelDataLock = levelData_lock.lock().await;
                        //.as_ref()
                        print!("hit1");

                        let mut data_levelData = levelData_lock.lock().await;

                        // Check if the message contains any level data
                        if new.levelData.lock().await.LevelDataInner.is_none() {
                            print!("Msg doesnt contain lvl data\n");
                            if data_levelData.LevelDataInner.is_some() {
                                data_levelData.update_state(crate::LevelState::Finished);
                            }
                            // if msg contains !(InLevel) replace state with finished
                            continue;
                        }
                        // Lock the BSData LevelData field
                        let new_levelData = &new.levelData.lock().await;

                        print!("hit2");
                        //print!("{:#?}\n\n", awaw.as_ref().unwrap().LevelDataInner.SongName);
                        //*data_levelData.LevelDataInner =
                        data_levelData.overwrite_leveldata(
                            new_levelData.LevelDataInner.as_ref().unwrap().to_owned(),
                        );

                        print!("hit3");
                        //drop(levelDataLock)
                        //ws.flush();
                    } else {
                        print!("huh");
                        continue;
                    };
                    //print!(msg);
                }
            }
        });

        // do we need to update gameData?
    }
}
