use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawBeatSaverMap {
    pub automapper: bool,
    #[serde(rename = "blQualified")]
    pub bl_qualified: bool,
    #[serde(rename = "blRanked")]
    pub bl_ranked: bool,
    pub bookmarked: bool,
    pub collaborators: Vec<User>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "curatedAt")]
    pub curated_at: String,
    pub curator: User,
    #[serde(rename = "declaredAi")]
    pub declared_ai: String,
    #[serde(rename = "deletedAt")]
    pub deleted_at: String,
    pub description: String,
    pub id: String,
    #[serde(rename = "lastPublishedAt")]
    pub last_published_at: String,
    pub metadata: Metadata,
    pub name: String,
    pub nsfw: bool,
    pub qualified: bool,
    pub ranked: bool,
    pub stats: Stats,
    pub tags: Vec<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub uploaded: String,
    pub uploader: User,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub bpm: Option<serde_json::Value>,
    pub duration: u32,
    #[serde(rename = "levelAuthorName")]
    pub level_author_name: String,
    #[serde(rename = "songAuthorName")]
    pub song_author_name: String,
    #[serde(rename = "songName")]
    pub song_name: String,
    #[serde(rename = "songSubName")]
    pub song_sub_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub downloads: u32,
    pub downvotes: u32,
    pub plays: u32,
    pub reviews: u32,
    pub score: Option<serde_json::Value>,
    #[serde(rename = "scoreOneDP")]
    pub score_one_dp: Option<serde_json::Value>,
    pub sentiment: String,
    pub upvotes: u32,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub admin: bool,
    pub avatar: String,
    pub blurnsfw: bool,
    pub curator: bool,
    pub curatorTab: bool,
    pub description: String,
    pub email: String,
    pub followData: FollowData,
    pub hash: String,
    pub id: u32,
    pub name: String,
    pub patreon: String,
    pub playlistUrl: String,
    pub seniorCurator: bool,
    pub stats: UserStats,
    pub suspendedAt: String,
    pub testplay: bool,
    #[serde(rename = "type")]
    pub user_type: String,
    pub uniqueSet: bool,
    pub uploadLimit: u32,
    pub verifiedMapper: bool,
    pub vivifyLimit: u32,
}

#[derive(Debug, Deserialize)]
pub struct FollowData {
    pub collab: bool,
    pub curation: bool,
    pub followers: u32,
    pub following: bool,
    pub follows: u32,
    pub upload: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserStats {
    pub avgBpm: Option<serde_json::Value>,
    pub avgDuration: Option<serde_json::Value>,
    pub avgScore: Option<serde_json::Value>,
    pub diffStats: DiffStats,
    pub firstUpload: String,
    pub lastUpload: String,
    pub rankedMaps: u32,
    pub totalDownvotes: u32,
    pub totalMaps: u32,
    pub totalUpvotes: u32,
}

#[derive(Debug, Deserialize)]
pub struct DiffStats {
    pub easy: u32,
    pub expert: u32,
    pub expertPlus: u32,
    pub hard: u32,
    pub normal: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub coverURL: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub diffs: Vec<Diff>,
    pub downloadURL: String,
    pub feedback: String,
    pub hash: String,
    pub key: String,
    pub previewURL: String,
    pub sageScore: Option<serde_json::Value>,
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: String,
    pub state: String,
    pub testplayAt: String,
    pub testplays: Vec<TestPlay>,
}

#[derive(Debug, Deserialize)]
pub struct Diff {
    pub blStars: Option<serde_json::Value>,
    pub bombs: u32,
    pub characteristic: String,
    pub chroma: bool,
    pub cinema: bool,
    pub difficulty: String,
    pub environment: String,
    pub events: u32,
    pub label: String,
    pub length: u32,
    pub maxScore: u32,
    pub me: bool,
    pub ne: bool,
    pub njs: Option<serde_json::Value>,
    pub notes: u32,
    pub nps: u32,
    pub obstacles: u32,
    pub offset: Option<serde_json::Value>,
    pub paritySummary: ParitySummary,
    pub seconds: u32,
    pub stars: Option<serde_json::Value>,
    pub vivify: bool,
}

#[derive(Debug, Deserialize)]
pub struct ParitySummary {
    pub errors: u32,
    pub resets: u32,
    pub warns: u32,
}

#[derive(Debug, Deserialize)]
pub struct TestPlay {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub feedback: String,
    #[serde(rename = "feedbackAt")]
    pub feedback_at: String,
    pub user: User,
}
