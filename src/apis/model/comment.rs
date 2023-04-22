use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub author_id: i32,
    pub author_nickname: String,
    pub content: String,
    pub post_id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCommentsResponse {
    pub ok: bool,
    pub comments: Vec<Comment>
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsFromPostRequest {
    pub post_id: i32,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub ok: bool,
    pub comment: Comment
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRequest {
    pub author_id: i32,
    pub content: String,
    pub post_id: i32,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditCommentRequest {
    pub comment_id: i32,
    pub content: String
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentIdRequest {
    pub comment_id: i32,
}