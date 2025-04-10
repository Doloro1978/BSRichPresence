mod bs_richpresence;
use crate::bs_richpresence::richpresence;
use BSDataPuller::BSData;
use BSDataPuller::LevelData;
use BSDataPuller::LevelState;
use BSDataPuller::schema::BSMetadata;
use discordipc::Client;
use discordipc::activity::*;
use discordipc::packet::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let oneshot_metadata = BSMetadata::get().await.unwrap();
    let bsdata = BSData::from_raw(oneshot_metadata);

    // start threads to update bsdata.
    bsdata.start().await;
    let client = Client::new_simple("1359573855412420741");
    client.connect_and_wait().unwrap();
    let activity = Activity::new().details("Some activity");
    let activity_packet = Packet::new_activity(Some(&activity), None);

    match client.send_and_wait(activity_packet).unwrap().filter() {
        Ok(_packet) => println!("Activity has been set!"),
        Err(e) => println!("Couldn't set activity: {}", e),
    }
    //let client = Arc::new(client);
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        //print!(".");
        let aw = bsdata.levelData.lock().await;
        match aw.LevelDataInner.as_ref() {
            Some(i) => {
                //print!("{:#?}", i)
                print!("");
            }
            None => {}
        }
        drop(aw);
        let activity = bsdata.to_activity().await;
        let activity_packet = Packet::new_activity(Some(&activity), None);
        client.send_and_wait(activity_packet).unwrap();
        //print!("{}", aw);
        //print!("{:#?}", awa)
        //print!()
    }
}
