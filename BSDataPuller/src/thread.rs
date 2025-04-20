use crate::BSData;
use crate::BSMetadata;
use futures_util::StreamExt;
use reqwest_websocket;
use reqwest_websocket::Message;
use tracing::debug;
use tracing::error;

impl BSData {
    // starts 2 threads to update BSData
    pub async fn start(&self) {
        //info!(UNIX_EPOCH);
        //let mut gameData = self.gameData.clone();
        let level_data = self.levelData.clone();

        //let levelData = &self.levelData;

        // levelData updater
        tokio::spawn(async move {
            //let awa = self.clone();
            let level_data_lock = level_data.clone().to_owned();
            let response = BSMetadata::connection().await;

            let mut ws = response.into_websocket().await.unwrap();
            loop {
                debug!("Hit-msg-processing-loop");
                if let Some(Ok(msg)) = ws.next().await {
                    debug!("Hit-msg-processing");
                    if let Message::Text(msg) = msg {
                        let new =
                            BSData::from_raw(serde_json::from_str::<BSMetadata>(&msg).unwrap());
                        //let mut levelDataLock = levelData_lock.lock().await;
                        //.as_ref()
                        //print!("hit1");

                        let mut data_level_data = level_data_lock.lock().await;

                        // Check if the message contains any level data
                        if new.levelData.lock().await.LevelDataInner.is_none() {
                            //print!("Msg doesnt contain lvl data\n");
                            if data_level_data.LevelDataInner.is_some() {
                                data_level_data.update_state(crate::LevelState::Finished);
                            }
                            // if msg contains !(InLevel) replace state with finished
                            continue;
                        }
                        // Lock the BSData LevelData field
                        let new_level_data = &new.levelData.lock().await;

                        //print!("hit2");
                        //print!("{:#?}\n\n", awaw.as_ref().unwrap().LevelDataInner.SongName);
                        //*data_levelData.LevelDataInner =
                        data_level_data.overwrite_leveldata(
                            new_level_data.LevelDataInner.as_ref().unwrap().to_owned(),
                        );

                        //print!("hit3");
                        //drop(levelDataLock)
                        //ws.flush();
                    } else {
                        error!("Unable to handle message : \n{:#?}", msg);
                        //if let Message::Close
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
                } else {
                    error!("Unable to await websocket.. killing thread.");
                    break;
                }
            }
            //info!("Goodbye..");
        });

        // do we need to update gameData?
    }
}
