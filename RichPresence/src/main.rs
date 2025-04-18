mod bs_richpresence;
use crate::bs_richpresence::richpresence;
use BSDataPuller::BSData;
use BSDataPuller::LevelData;
use BSDataPuller::LevelState;
use BSDataPuller::schema::BSMetadata;
use discordipc::Client;
use discordipc::activity::*;
use discordipc::packet::*;
use std::process::exit;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let oneshot_metadata = BSMetadata::get().await.unwrap();
    let bsdata = BSData::from_raw(oneshot_metadata);

    // start threads to update bsdata.
    bsdata.start().await;

    let client = Client::new_simple("1359573855412420741");
    client.connect_and_wait().unwrap();
    let activity = Activity::new().details("Some activity");
    let activity_packet = Packet::new_activity(Some(&activity), None);
    let start = SystemTime::now();
    let startedAt = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let started_at = Arc::new(startedAt);

    match client.send_and_wait(activity_packet).unwrap().filter() {
        Ok(_packet) => info!("Activity has been set!"),
        Err(e) => info!("Couldn't set activity: {}", e),
    }

    tokio::spawn(async {
        loop {
            // if unable to ping for any reason, exit, assuming game has quit.
            // this spawns a new client / connection each time, kinda expensive
            if !BSData::ping().await {
                exit(0);
            }
            info!("pinged");
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let mut activity = bsdata.to_activity().await;
            activity.timestamps.replace(Timestamps {
                start: Some(started_at.clone().as_secs() as i64),
                ..Default::default()
            });
            let activity_packet = Packet::new_activity(Some(&activity), None);
            client.send_and_wait(activity_packet).unwrap();
            //info!("awa");
            //print!("{}", aw);
            //print!("{:#?}", awa)
            //print!()
        }
    });
    loop {}
}
