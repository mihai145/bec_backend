use serde::{Deserialize, Serialize};

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeResponse {
    pub ok: bool,
    pub results: i64,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentLikeRequest {
    pub comment_id: i32,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLikeRequest {
    pub post_id: i32,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentLikedRequest {
    pub comment_id: i32,
    pub author_id: i32
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLikedRequest {
    pub post_id: i32,
    pub author_id: i32
}