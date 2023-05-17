use crate::apis::model;
use rocket::http::{Status, ContentType};
use serde_json::json;
use rocket::serde::json::Json;
use crate::apis::auth;
use crate::apis::postgres;

#[post("/search/nickname", format="json", data="<body>")]
pub async fn get_users(body: Json<model::user::UserNameSearchRequest>) -> (Status, (ContentType, String)) {

    let nickname = format!("{}{}{}", '%', &body.user_name, '%');
    let users = sqlx::query_as!(
        model::user::User,
        r#"SELECT id AS "id!", nickname AS "nickname!", email AS "email!" FROM users WHERE nickname LIKE $1"#,
        nickname
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            Vec::new()
        });

    if users.len() == 0 {
        parse_error()
    } else {
        success_response(json!(model::user::UserSearchResponse{
            ok: true,
            results: users
        }).to_string())
    }
}

#[post("/amIFollowing", format="json", data="<body>")]
pub async fn am_i_following(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let count = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!" FROM follow WHERE follower_id = $1 AND followee_id = $2"#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't fetch data! {}", e);
            model::user::DbCount{cnt: 0}
        });

    if count.cnt > 0 {
        success_response(json!(model::user::AmIFollowingResponse{
            ok: true,
            following: true
        }).to_string())
    } else {
        success_response(json!(model::user::AmIFollowingResponse{
            ok: true,
            following: false
        }).to_string())
    }
}

#[post("/follow", format="json", data="<body>")]
pub async fn follow(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret = sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into follow VALUES ($1, $2) RETURNING follower_id AS "cnt!""#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if ret.cnt != body.follower_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

#[post("/unfollow", format="json", data="<body>")]
pub async fn unfollow(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::FollowRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.follower_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from follow WHERE follower_id = $1 AND followee_id = $2 RETURNING follower_id AS "cnt!""#,
        body.follower_id, body.followee_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: -1}
        });

    if ret.cnt != body.follower_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database removal failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

#[get("/posts")]
pub async fn posts(_bearer: auth::bearer::Bearer<'_>) -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::post::Post,
        r#"SELECT post.id AS "id!", post.author_id AS "author_id!", users.nickname AS "author_nickname!", post.title AS "title!", post.content AS "content!", post.movie_id AS "movie_id", post.movie_name AS "movie_name" 
            FROM post 
            JOIN users ON post.author_id = users.id"#
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            Vec::new()
        });
    
    (Status::Accepted, (ContentType::JSON, json!(model::post::FeedResponse{
        ok: true,
        posts: res
    }).to_string()))
}

async fn get_review_count(author_id: i32, movie_id: i32) -> i32 {
    let ret = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!" from post WHERE author_id = $1 AND movie_id = $2"#,
        author_id, movie_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbCount{cnt: -1}
        });
    
    if ret.cnt == -1 {
        // db error
        return -1
    } else if ret.cnt == 1 {
        let id = sqlx::query_as!(
            model::user::DbInt,
            r#"SELECT id AS "cnt!" from post WHERE author_id = $1 AND movie_id = $2"#,
            author_id, movie_id
            ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
                // db error
                error!("Couldn't insert data! {}", e);
                model::user::DbInt{cnt: -1}
        });
        return id.cnt
    } else {
        // no review
        return 0;
    }
}

#[post("/didIReview", format="json", data="<body>")]
pub async fn did_i_review(bearer: auth::bearer::Bearer<'_>, body: Json<model::post::DidIReviewRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let cnt = get_review_count(body.author_id, body.movie_id).await;

    if cnt < 0 {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database read failed")
        }).to_string()))
    } else if cnt == 0 {
        (Status::Accepted, (ContentType::JSON, json!(model::post::DidIReviewResponse{
            ok: true,
            reviewed: false,
            post_id: -1
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::post::DidIReviewResponse{
            ok: true,
            reviewed: true,
            post_id: cnt
        }).to_string()))
    }
}

async fn make_post(author_id: i32, title: String, content: String, movie_id: Option<i32>, movie_name: Option<String>) -> model::user::DbInt {
    sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into post (author_id, title, content, movie_id, movie_name) VALUES ($1, $2, $3, $4, $5) RETURNING author_id AS "cnt!""#,
        author_id, title, content, movie_id, movie_name
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        })
}

#[post("/post", format="json", data="<body>")]
pub async fn post(bearer: auth::bearer::Bearer<'_>, body: Json<model::post::FeedPostRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret;
    
    match body.movie_id {
        Some(m_id) => {
            let cnt = get_review_count(body.author_id, m_id).await;
            if cnt > 0 {
                // already reviewed
                return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
                    ok: false,
                    reason: String::from("You have already reviewed this movie")
                }).to_string()));
            } else if cnt < 0 {
                // error while getting review count
                return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
                    ok: false,
                    reason: String::from("Database insertion failed")
                }).to_string()));
            } else {
                ret = make_post(body.author_id, body.title.clone(), body.content.clone(), body.movie_id, body.movie_name.clone()).await;
            }
        }
        None => {
            ret = make_post(body.author_id, body.title.clone(), body.content.clone(), body.movie_id, body.movie_name.clone()).await;
        }
    }

    if ret.cnt != body.author_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

async fn get_post_from_id(post_id: i32) -> model::post::Post {
    sqlx::query_as!(
        model::post::Post,
        r#"SELECT post.id AS "id!", post.author_id AS "author_id!", users.nickname AS "author_nickname!", post.title AS "title!", post.content AS "content!", post.movie_id AS "movie_id", post.movie_name AS "movie_name" 
            FROM post 
            JOIN users ON post.author_id = users.id
            WHERE post.id = $1"#,
        post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::post::Post{id: -1, author_id: -1, author_nickname: String::from(""), title: String::from(""), content: String::from(""), movie_id: Some(-1), movie_name: Some(String::from(""))}
        })
}

#[post("/getPost", format="json", data="<body>")]
pub async fn get_post(_bearer: auth::bearer::Bearer<'_>, body: Json<model::post::PostIdRequest>) -> (Status, (ContentType, String)) {
    let ret = get_post_from_id(body.post_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such post")
        }).to_string()));
    }
    
    (Status::Accepted, (ContentType::JSON, json!(model::post::PostResponse{
        ok: true,
        post: ret
    }).to_string()))
}

#[post("/editPost", format="json", data="<body>")]
pub async fn edit_post(bearer: auth::bearer::Bearer<'_>, body: Json<model::post::EditFeedPostRequest>) -> (Status, (ContentType, String)) {
    let ret = get_post_from_id(body.post_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such post")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, ret.author_id).await && !auth::bearer::is_admin(bearer).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()));
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"UPDATE post SET title = $1, content = $2 WHERE id = $3 RETURNING id AS "cnt!""#,
        body.title, body.content, body.post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't edit data! {}", e);
            model::user::DbInt{cnt: 0}
        });
    
    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not edit post")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

#[post("/deletePost", format="json", data="<body>")]
pub async fn delete_post(bearer: auth::bearer::Bearer<'_>, body: Json<model::post::PostIdRequest>) -> (Status, (ContentType, String)) {
    let ret = get_post_from_id(body.post_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such post")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, ret.author_id).await && !auth::bearer::is_admin(bearer).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from post WHERE id = $1 RETURNING id AS "cnt!""#,
        body.post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete post")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

///////////////////////////////////////

#[post("/comments", format="json", data="<body>")]
pub async fn comments(_bearer: auth::bearer::Bearer<'_>, body: Json<model::comment::CommentsFromPostRequest>) -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::comment::Comment,
        r#"SELECT comment.id AS "id!", comment.author_id AS "author_id!", users.nickname AS "author_nickname!", comment.content AS "content!", comment.post_id AS "post_id!"
            FROM comment 
            JOIN users ON comment.author_id = users.id
            WHERE comment.post_id = $1"#,
            body.post_id
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            Vec::new()
        });
    
    (Status::Accepted, (ContentType::JSON, json!(model::comment::PostCommentsResponse{
        ok: true,
        comments: res
    }).to_string()))
}

async fn make_comment(author_id: i32, content: String, post_id: i32) -> model::user::DbInt {
    let ret = get_post_from_id(post_id).await;
    if ret.id != -1 {
        let message = format!("New comment on your post titled {} about {}",ret.title,ret.movie_name.unwrap_or(String::new()));
        make_notification(ret.author_id,message).await;
    }

    sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into comment (author_id, content, post_id) VALUES ($1, $2, $3) RETURNING author_id AS "cnt!""#,
        author_id, content, post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        })
}

#[post("/comment", format="json", data="<body>")]
pub async fn comment(bearer: auth::bearer::Bearer<'_>, body: Json<model::comment::CommentRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret;
    
    ret = make_comment(body.author_id, body.content.clone(), body.post_id).await;

    if ret.cnt != body.author_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

async fn get_comment_from_id(comment_id: i32) -> model::comment::Comment {
    sqlx::query_as!(
        model::comment::Comment,
        r#"SELECT comment.id AS "id!", comment.author_id AS "author_id!", users.nickname AS "author_nickname!", comment.content AS "content!", comment.post_id AS "post_id!"
            FROM comment 
            JOIN users ON comment.author_id = users.id
            WHERE comment.id = $1"#,
        comment_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::comment::Comment{id: -1, author_id: -1, author_nickname: String::from(""), content: String::from(""), post_id: -1}
        })
}

#[post("/getComment", format="json", data="<body>")]
pub async fn get_comment(_bearer: auth::bearer::Bearer<'_>, body: Json<model::comment::CommentIdRequest>) -> (Status, (ContentType, String)) {
    let ret = get_comment_from_id(body.comment_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such comment")
        }).to_string()));
    }
    
    (Status::Accepted, (ContentType::JSON, json!(model::comment::CommentResponse{
        ok: true,
        comment: ret
    }).to_string()))
}

#[post("/editComment", format="json", data="<body>")]
pub async fn edit_comment(bearer: auth::bearer::Bearer<'_>, body: Json<model::comment::EditCommentRequest>) -> (Status, (ContentType, String)) {
    let ret = get_comment_from_id(body.comment_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such post")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, ret.author_id).await && !auth::bearer::is_admin(bearer).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()));
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"UPDATE comment SET content = $1 WHERE id = $2 RETURNING id AS "cnt!""#,
        body.content, body.comment_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't edit data! {}", e);
            model::user::DbInt{cnt: 0}
        });
    
    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not edit post")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

#[post("/deleteComment", format="json", data="<body>")]
pub async fn delete_comment(bearer: auth::bearer::Bearer<'_>, body: Json<model::comment::CommentIdRequest>) -> (Status, (ContentType, String)) {
    let ret = get_comment_from_id(body.comment_id).await;
    if ret.id == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such post")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, ret.author_id).await && !auth::bearer::is_admin(bearer).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from comment WHERE id = $1 RETURNING id AS "cnt!""#,
        body.comment_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete post")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

#[post("/deleteUser", format="json", data="<body>")]
pub async fn delete_user(bearer: auth::bearer::Bearer<'_>, body: Json<model::user::UserIdRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::is_admin(bearer).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You are not an admin")
        }).to_string()))
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from users WHERE id = $1 RETURNING id AS "cnt!""#,
        body.user_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete user")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

fn parse_error() -> (Status, (ContentType, String)) {
    let error_response = json!(model::error::Error{
        ok: false,
        reason: String::from("Internal database error!")
    }).to_string();

    (Status::InternalServerError, (ContentType::JSON, error_response))
}

/////////////////////////////


#[post("/likesPost", format="json", data="<body>")]
pub async fn likes_post(_bearer: auth::bearer::Bearer<'_>, body: Json<model::like::PostLikeRequest>) -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!"
            FROM post_likes 
            WHERE post_likes.post_id = $1"#,
            body.post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::user::DbCount{cnt: 0}
        });
    
    (Status::Accepted, (ContentType::JSON, json!(model::like::LikeResponse{
        ok: true,
        results: res.cnt
    }).to_string()))
}

async fn make_post_like(author_id: i32, post_id: i32) -> model::user::DbInt {
    let ret = get_post_from_id(post_id).await;
    if ret.id != -1 {
        let message = format!("New like on your post titled {} about {}",ret.title,ret.movie_name.unwrap_or(String::new()));
        make_notification(ret.author_id,message).await;
    }

    sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into post_likes (user_id, post_id) VALUES ($1, $2) RETURNING user_id AS "cnt!""#,
        author_id, post_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        })
}

#[post("/likePost", format="json", data="<body>")]
pub async fn like_post(bearer: auth::bearer::Bearer<'_>, body: Json<model::like::PostLikedRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret;
    
    ret = make_post_like(body.author_id, body.post_id).await;

    if ret.cnt != body.author_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

async fn get_post_like_from_id(post_id: i32, author_id: i32) -> i64 {
    let ret = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!"
            FROM post_likes 
            WHERE post_likes.post_id = $1 AND post_likes.user_id = $2"#,
        post_id, author_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::user::DbCount {cnt: -1}
        });
    
    return ret.cnt
}

#[post("/getLikePost", format="json", data="<body>")]
pub async fn get_like_post(_bearer: auth::bearer::Bearer<'_>, body: Json<model::like::PostLikedRequest>) -> (Status, (ContentType, String)) {
    let ret = get_post_like_from_id(body.post_id, body.author_id).await;
    if ret == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Internal Database error")
        }).to_string()));
    }
    
    (Status::Accepted, (ContentType::JSON, json!(model::like::LikeResponse{
        ok: true,
        results: ret
    }).to_string()))
}

#[post("/deleteLikePost", format="json", data="<body>")]
pub async fn delete_like_post(bearer: auth::bearer::Bearer<'_>, body: Json<model::like::PostLikedRequest>) -> (Status, (ContentType, String)) {
    let ret = get_post_like_from_id(body.post_id, body.author_id).await;
    if ret <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such like")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from post_likes WHERE post_id = $1 AND user_id = $2 RETURNING user_id AS "cnt!""#,
        body.post_id, body.author_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete like")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

///////////////////////////////


#[post("/likesComment", format="json", data="<body>")]
pub async fn likes_comment(_bearer: auth::bearer::Bearer<'_>, body: Json<model::like::CommentLikeRequest>) -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!"
            FROM comment_likes 
            WHERE comment_likes.comment_id = $1"#,
            body.comment_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::user::DbCount{cnt: 0}
        });
    
    (Status::Accepted, (ContentType::JSON, json!(model::like::LikeResponse{
        ok: true,
        results: res.cnt
    }).to_string()))
}

async fn make_comment_like(author_id: i32, comment_id: i32) -> model::user::DbInt {
    let ret = get_comment_from_id(comment_id).await;
    if ret.id != -1 {
        let message = String::from("New like on your comment");
        make_notification(ret.author_id,message).await;
    }

    sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into comment_likes (user_id, comment_id) VALUES ($1, $2) RETURNING user_id AS "cnt!""#,
        author_id, comment_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        })
}

#[post("/likeComment", format="json", data="<body>")]
pub async fn like_comment(bearer: auth::bearer::Bearer<'_>, body: Json<model::like::CommentLikedRequest>) -> (Status, (ContentType, String)) {
    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let ret;
    
    ret = make_comment_like(body.author_id, body.comment_id).await;

    if ret.cnt != body.author_id {
        (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Database insertion failed")
        }).to_string()))
    } else {
        (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
            ok: true,
            reason: String::from("Ok")
        }).to_string()))   
    }
}

async fn get_comment_like_from_id(comment_id: i32, author_id: i32) -> i64 {
    let ret = sqlx::query_as!(
        model::user::DbCount,
        r#"SELECT COUNT(*) AS "cnt!"
            FROM comment_likes 
            WHERE comment_id = $1 AND user_id = $2"#,
        comment_id, author_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            model::user::DbCount{cnt: -1}
        });
    
    return ret.cnt
}

#[post("/getLikeComment", format="json", data="<body>")]
pub async fn get_like_comment(_bearer: auth::bearer::Bearer<'_>, body: Json<model::like::CommentLikedRequest>) -> (Status, (ContentType, String)) {
    let ret = get_comment_like_from_id(body.comment_id, body.author_id).await;
    if ret == -1 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Internal Database error")
        }).to_string()));
    }
    
    (Status::Accepted, (ContentType::JSON, json!(model::like::LikeResponse{
        ok: true,
        results: ret
    }).to_string()))
}

#[post("/deleteLikeComment", format="json", data="<body>")]
pub async fn delete_like_comment(bearer: auth::bearer::Bearer<'_>, body: Json<model::like::CommentLikedRequest>) -> (Status, (ContentType, String)) {
    let ret = get_comment_like_from_id(body.comment_id, body.author_id).await;
    if ret <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("No such like")
        }).to_string()));
    }

    if !auth::bearer::match_sub(bearer, body.author_id).await {
        return (Status::Unauthorized, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("You do not have permission to act on behalf of other users")
        }).to_string()))
    }

    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from comment_likes WHERE comment_id = $1 AND user_id = $2 RETURNING user_id AS "cnt!""#,
        body.comment_id, body.author_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: 0}
        });

    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete like")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

#[get("/leaderboard")]
pub async fn get_leaderboard() -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::leaderboard::LeaderboardUser,
        r#"WITH 
        commentLikesSum AS 
        (	SELECT c.author_id, COUNT(cl.user_id)
             FROM comment_likes cl
             JOIN comment c ON c.id = cl.comment_id 
            GROUP BY c.author_id
        ),
        postLikesSum AS
        ( 	SELECT p.author_id, COUNT(pl.user_id)
             FROM post_likes pl
             JOIN post p ON p.id = pl.post_id 
            GROUP BY p.author_id
        )
        SELECT *,
				ROW_NUMBER() OVER(ORDER BY "total_likes!" DESC) AS "place!"
		FROM(
			SELECT
				coalesce(cls.author_id,pls.author_id) as "id!", 
                u.nickname as "nickname!", 
                u.email as "email!",
                (CASE WHEN cls.count is NULL THEN 0 ELSE cls.count END) + (CASE WHEN pls.count is NULL THEN 0 ELSE pls.count END) AS "total_likes!"
			FROM commentLikesSum cls
			FULL OUTER JOIN postLikesSum pls ON cls.author_id = pls.author_id
			INNER JOIN users u ON cls.author_id = u.id OR pls.author_id = u.id
			ORDER BY "total_likes!" DESC
			LIMIT 5
			) tabel;"#,
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            Vec::new()
        });
        
        (Status::Accepted, (ContentType::JSON, json!(model::leaderboard::LeaderboardResultApi{
            ok: true,
            users: res
        }).to_string()))
}

// Augment the response with status code and content type
fn success_response(serialized_json: String) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, serialized_json))
}

async fn make_notification(user_id: i32, message: String) -> model::user::DbInt
{
    sqlx::query_as!(
        model::user::DbInt,
        r#"INSERT into notification (user_id, message) VALUES ($1, $2) RETURNING notification_id AS "cnt!""#,
        user_id, message
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't insert data! {}", e);
            model::user::DbInt{cnt: 0}
        })
}
#[post("/deleteNotification", format="json", data="<body>")]
pub async fn delete_notification(_bearer: auth::bearer::Bearer<'_>,body: Json<model::user::NotificationDelete>) -> (Status, (ContentType, String)) {
    let res = sqlx::query_as!(
        model::user::DbInt,
        r#"DELETE from notification WHERE notification_id = $1 RETURNING user_id AS "cnt!""#,
        body.notification_id
        ).fetch_one(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't delete data! {}", e);
            model::user::DbInt{cnt: -1}
        });
    
    if res.cnt <= 0 {
        return (Status::InternalServerError, (ContentType::JSON, json!(model::error::Error{
            ok: false,
            reason: String::from("Could not delete like")
        }).to_string()));
    }

    (Status::Accepted, (ContentType::JSON, json!(model::error::Error{
        ok: true,
        reason: String::from("OK")
    }).to_string()))
}

#[post("/getNotification", format="json", data="<body>")]
pub async fn get_notification(_bearer: auth::bearer::Bearer<'_>,body: Json<model::user::NotificationRequest>) -> (Status, (ContentType, String)) {
    
    let res = sqlx::query_as!(
        model::user::Notification,
        r#"SELECT notification_id AS "notification_id!",message AS "message!"
            FROM notification
            WHERE user_id = $1"#,
            body.user_id
        ).fetch_all(&*(postgres::pool::PG.get().await)).await.unwrap_or_else(|e| {
            error!("Couldn't read data! {}", e);
            Vec::new()
        });
    
    (Status::Accepted, (ContentType::JSON, json!(model::user::NotificationResponse{
        ok: true,
        results: res
    }).to_string()))
}
