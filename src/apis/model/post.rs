use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub author_id: i32,
    pub author_nickname: String,
    pub title: String,
    pub content: String,
    pub movie_id: Option<i32>,
    pub movie_name: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedResponse {
    pub ok: bool,
    pub posts: Vec<Post>
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    pub ok: bool,
    pub post: Post
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidIReviewRequest {
    pub author_id: i32,
    pub movie_id: i32
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidIReviewResponse {
    pub ok: bool,
    pub reviewed: bool,
    pub post_id: i32
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedPostRequest {
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub movie_id: Option<i32>,
    pub movie_name: Option<String>
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditFeedPostRequest {
    pub post_id: i32,
    pub title: String,
    pub content: String
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostIdRequest {
    pub post_id: i32,
}