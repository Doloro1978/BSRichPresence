use crate::bs_processing::BSProcessedData;
use crate::bs_processing::ProcessedLevelData;
use BSDataPuller::BSData;
use BSDataPuller::LevelState;
use BSDataPuller::livedata::schema::BSLivedata;
use discordipc::activity::*;
use std::cell::Ref;
use tracing::debug;
use tracing::info;

use BSDataPuller::LevelDataInner;

pub trait RichPresence {
    async fn to_activity(&self, livedata: &BSLivedata) -> Activity;
    fn inmenu_activity() -> Activity;
    fn insong_activity(awaw: &ProcessedLevelData, livedata: &BSLivedata) -> Activity;
}

impl RichPresence for BSProcessedData {
    async fn to_activity(&self, livedata: &BSLivedata) -> Activity {
        //let level_data_a = self.levelData.lock().await;
        if let Some(level_data) = self.level_data.clone() {
            if level_data.state == LevelState::Playing {
                BSProcessedData::insong_activity(&level_data, livedata)
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

    fn insong_activity(awaw: &ProcessedLevelData, livedata: &BSLivedata) -> Activity {
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

        activity.state.replace(format!(
            "{} // Score - {}",
            calculate_time(livedata.time_elapsed, awaw.time),
            livedata.score
        ));

        let playing_string = format!("Playing {} {}", awaw.song_name, awaw.song_sub_name);
        activity.details.replace(playing_string);

        return activity;
    }
}
fn calculate_time(elapsed: u32, total: u32) -> String {
    let total_min = total / 60;
    let total_sec = total % 60;
    let elapsed_min = elapsed / 60;
    let elapsed_sec = elapsed % 60;

    let total_str = format!("{:02}:{:02}", total_min, total_sec);
    let elapsed_str = format!("{:02}:{:02}", elapsed_min, elapsed_sec);

    return format!("{} / {}", elapsed_str, total_str);
}
