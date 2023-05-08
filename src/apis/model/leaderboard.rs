use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardResultApi {
    pub ok: bool,
    pub users: Vec<LeaderboardUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardUser {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub place: i64,
    pub total_likes: i64
}