use BSDataPuller::BSData;
use BSDataPuller::LevelData;
use BSDataPuller::LevelState;
use BSDataPuller::schema::BSMetadata;
use discordipc::Client;
use discordipc::activity::*;
use discordipc::packet::*;

use BSDataPuller::LevelDataInner;

pub trait richpresence {
    async fn to_activity(&self) -> Activity;
    fn inmenu_Activity() -> Activity;
    fn insong_Activity(awaw: &LevelDataInner) -> Activity;
}

impl richpresence for BSData {
    async fn to_activity(&self) -> Activity {
        let levelData_a = self.levelData.lock().await;
        let mut activity = Activity::new();

        if let Some(levelData) = levelData_a.LevelDataInner.clone() {
            if levelData.State == LevelState::Playing {
                return BSData::insong_Activity(&levelData);
            } else {
                return BSData::inmenu_Activity();
            };
        } else {
            return BSData::inmenu_Activity();
        };

        return activity;
    }
    // all this could be an enum
    fn inmenu_Activity() -> Activity {
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

    fn insong_Activity(awaw: &LevelDataInner) -> Activity {
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
            .replace("https://pixsector.com/cache/8955ccde/avea0c6d1234636825bd6.png".to_owned());
        activity
            .assets
            .small_text
            .replace("Playing Song".to_owned());

        let playingString = "Playing ".to_owned() + String::from(awaw.SongName.clone()).as_str();
        activity.details.replace(playingString);

        return activity;
    }
}
