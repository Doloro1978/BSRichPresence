use BSDataPuller::BSData;
use BSDataPuller::LevelData;
use BSDataPuller::LevelState;
use BSDataPuller::schema::BSMetadata;
use discordipc::Client;
use discordipc::activity::*;
use discordipc::packet::*;

pub trait richpresence {
    async fn to_activity(&self) -> Activity;
}

impl richpresence for BSData {
    async fn to_activity(&self) -> Activity {
        let levelData_a = self.levelData.lock().await;
        let mut activity = Activity::new();
        if let Some(awaw) = levelData_a.LevelDataInner.clone() {
            if (awaw.State == LevelState::Playing) {
                //let leveldata = levelData_a.LevelDataInner.as_ref().unwrap().clone();
                if !(awaw.CoverImage.len() > 100) {
                    activity.assets.large_image.replace(awaw.CoverImage);
                } else {
                    activity.assets.large_image.replace("https://upload.wikimedia.org/wikipedia/commons/5/5a/Black_question_mark.png".to_owned());
                }
                activity.assets.large_text.replace(awaw.SongName.clone());

                activity.assets.small_image.replace(
                    "https://pixsector.com/cache/8955ccde/avea0c6d1234636825bd6.png".to_owned(),
                );
                activity
                    .assets
                    .small_text
                    .replace("Playing Song".to_owned());

                let playingString = "Playing ".to_owned() + String::from(awaw.SongName).as_str();
                activity.details.replace(playingString);
                //activity.details
            } else {
                activity.assets.large_image.replace("https://image.api.playstation.com/gs2-sec/appkgo/prod/CUSA14143_00/1/i_1867cbfbe18338d0089137e8e84ec6b550a97e1f62a41df7c66e1cba550b1484/i/icon0.png".to_owned());
                activity.assets.large_text.replace("In Menus..".to_owned());
            }
        } else {
            activity.assets.large_image.replace("https://image.api.playstation.com/gs2-sec/appkgo/prod/CUSA14143_00/1/i_1867cbfbe18338d0089137e8e84ec6b550a97e1f62a41df7c66e1cba550b1484/i/icon0.png".to_owned());
            activity.assets.large_text.replace("In Menus..".to_owned());

            activity
                .assets
                .small_image
                .replace("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTN7DHy1QxOQjG-80V3URdqd_CNIzCPwfd5eQ&s".to_owned());
            activity.assets.small_text.replace("Menu".to_owned());

            activity.details.replace("Sitting in menu..".to_owned());
        }
        return activity;
    }
}
