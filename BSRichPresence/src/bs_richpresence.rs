use crate::bs_processing::BSProcessedData;
use crate::bs_processing::ProcessedLevelData;
use BSDataPuller::BSData;
use BSDataPuller::LevelState;
use discordipc::activity::*;
use tracing::debug;
use tracing::info;

use BSDataPuller::LevelDataInner;

pub trait RichPresence {
    async fn to_activity(&self) -> Activity;
    fn inmenu_activity() -> Activity;
    fn insong_activity(awaw: &ProcessedLevelData) -> Activity;
}

impl RichPresence for BSProcessedData {
    async fn to_activity(&self) -> Activity {
        //let level_data_a = self.levelData.lock().await;
        if let Some(level_data) = self.level_data.clone() {
            if level_data.state == LevelState::Playing {
                BSProcessedData::insong_activity(&level_data)
            } else {
                BSProcessedData::inmenu_activity()
            }
        } else {
            BSProcessedData::inmenu_activity()
        }
        //let activity = Activity::new();
        //return activity;
    }
    // all this could be an enum
    fn inmenu_activity() -> Activity {
        let mut activity = Activity::new();

        activity.assets.large_image.replace("https://image.api.playstation.com/gs2-sec/appkgo/prod/CUSA14143_00/1/i_1867cbfbe18338d0089137e8e84ec6b550a97e1f62a41df7c66e1cba550b1484/i/icon0.png".to_owned());
        activity.assets.large_text.replace("In Menus..".to_owned());

        activity
                .assets
                .small_image
                .replace("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTN7DHy1QxOQjG-80V3URdqd_CNIzCPwfd5eQ&s".to_owned());
        activity.assets.small_text.replace("Menu".to_owned());

        activity.details.replace("Sitting in menu..".to_owned());

        return activity;
    }

    fn insong_activity(awaw: &ProcessedLevelData) -> Activity {
        let mut activity = Activity::new();

        activity
            .assets
            .large_image
            .replace(awaw.cover_image.clone());
        activity.assets.large_text.replace(awaw.song_name.clone());

        activity
            .assets
            .small_image
            .replace("https://raw.githubusercontent.com/Doloro1978/BSRichPresence/refs/heads/master/Assets/RankedIcon.png".to_owned());

        let mut diff_string: String;
        debug!("{:#?}", awaw);
        if awaw.stars > 0.0 {
            let stars = awaw.stars;
            diff_string = format!("Ranked | {stars} Stars");
        } else {
            diff_string = format!("Normal");
            activity.assets.small_image.replace("https://github.com/Doloro1978/BSRichPresence/blob/master/Assets/NormalIcon.png?raw=true".to_owned());
        }
        if awaw.qualified {
            diff_string = format!("Qualified");
            // TODO Add qualified icon..
        }

        activity.assets.small_text.replace(diff_string.to_owned());

        let playing_string = format!("Playing {} ({})", awaw.song_name, awaw.song_sub_name);
        activity.details.replace(playing_string);

        return activity;
    }
}
