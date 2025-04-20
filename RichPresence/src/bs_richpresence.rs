use BSDataPuller::BSData;
use BSDataPuller::LevelState;
use discordipc::activity::*;
use tracing::info;

use BSDataPuller::LevelDataInner;

pub trait RichPresence {
    async fn to_activity(&self) -> Activity;
    fn inmenu_activity() -> Activity;
    fn insong_activity(awaw: &LevelDataInner) -> Activity;
}

impl RichPresence for BSData {
    async fn to_activity(&self) -> Activity {
        let level_data_a = self.levelData.lock().await;

        if let Some(level_data) = level_data_a.LevelDataInner.clone() {
            if level_data.State == LevelState::Playing {
                BSData::insong_activity(&level_data)
            } else {
                BSData::inmenu_activity()
            }
        } else {
            BSData::inmenu_activity()
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
        //info!("{:#?}", activity);

        return activity;
    }

    fn insong_activity(awaw: &LevelDataInner) -> Activity {
        let mut activity = Activity::new();

        activity.assets.large_image.replace(awaw.CoverImage.clone());
        if awaw.CoverImage.len() > 100 {
            activity.assets.large_image.replace(
                "https://upload.wikimedia.org/wikipedia/commons/5/5a/Black_question_mark.png"
                    .to_owned(),
            );
        };
        activity.assets.large_text.replace(awaw.SongName.clone());

        activity
            .assets
            .small_image
            .replace("https://raw.githubusercontent.com/Doloro1978/BSRichPresence/refs/heads/master/Assets/RankedIcon.png".to_owned());

        let diff_string: String;
        info!(awaw.Star);
        if awaw.Star > 0.0 {
            let stars = awaw.Star;
            diff_string = format!("Ranked | {stars} Stars");
        } else {
            diff_string = format!("Normal");
            activity.assets.small_image.replace("https://github.com/Doloro1978/BSRichPresence/blob/master/Assets/NormalIcon.png?raw=true".to_owned());
        }

        activity.assets.small_text.replace(diff_string.to_owned());

        // TODO Use format!
        let playing_string = "Playing ".to_owned() + String::from(awaw.SongName.clone()).as_str();
        activity.details.replace(playing_string);

        return activity;
    }
}
