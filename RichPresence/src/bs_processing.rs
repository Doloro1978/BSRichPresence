// Used to post process data from BSData into something useable for rich presence using a provided
// config

use BSDataPuller::BSData;
use config::schema::RichPresenceConfig;

struct ProcessedLevelData {}

struct BSProcessedData {
    level_data: ProcessedLevelData,
}

pub trait Processing {
    fn process(&self, config: &RichPresenceConfig);
}

impl Processing for BSData {
    fn process(&self, config: &RichPresenceConfig) {
        //config.ranked
    }
}
