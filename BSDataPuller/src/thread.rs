use crate::BSData;
use crate::BSMetadata;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use std::mem;
use tokio::spawn;
use tokio::sync;
use tracing::error;
use tracing::info;

impl BSData {
    // starts 2 threads to update BSData
    pub async fn start(&self) {
        //info!(UNIX_EPOCH);
        //let mut gameData = self.gameData.clone();
        let mut levelData = self.levelData.clone();

        //let levelData = &self.levelData;

        // levelData updater
        tokio::spawn(async move {
            //let awa = self.clone();
            let levelData_lock = levelData.clone().to_owned();
            let response = BSMetadata::connection().await;

            let mut ws = response.into_websocket().await.unwrap();
            loop {
                if let Some(Ok(msg)) = ws.next().await {
                    if let Message::Text(msg) = msg {
                        let new =
                            BSData::from_raw(serde_json::from_str::<BSMetadata>(&msg).unwrap());
                        //let mut levelDataLock = levelData_lock.lock().await;
                        //.as_ref()
                        //print!("hit1");

                        let mut data_levelData = levelData_lock.lock().await;

                        // Check if the message contains any level data
                        if new.levelData.lock().await.LevelDataInner.is_none() {
                            //print!("Msg doesnt contain lvl data\n");
                            if data_levelData.LevelDataInner.is_some() {
                                data_levelData.update_state(crate::LevelState::Finished);
                            }
                            // if msg contains !(InLevel) replace state with finished
                            continue;
                        }
                        // Lock the BSData LevelData field
                        let new_levelData = &new.levelData.lock().await;

                        //print!("hit2");
                        //print!("{:#?}\n\n", awaw.as_ref().unwrap().LevelDataInner.SongName);
                        //*data_levelData.LevelDataInner =
                        data_levelData.overwrite_leveldata(
                            new_levelData.LevelDataInner.as_ref().unwrap().to_owned(),
                        );

                        //print!("hit3");
                        //drop(levelDataLock)
                        //ws.flush();
                    } else {
                        //if let Message::Binary(msg) = msg {
                        //}
                        //print!("{:#?}", msg);
                        //if let = msg {}
                        //info!("huh");
                        //info!("guessing beat saber exited.. quitting.............");
                        //std::process::exit(0);
                        continue;
                    };
                    //print!(msg);
                }
            }
        });

        // do we need to update gameData?
    }
}
