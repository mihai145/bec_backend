use serde::{Deserialize, Serialize};

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorNameSearchRequest {
    pub actor_name: String,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorNameSearchResponse {
    pub ok: bool,
    pub results: Vec<Actor>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorSearchResultApi {
    pub results: Vec<Actor>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,
    pub name: String,
    #[serde(rename = "original_name")]
    pub original_name: String,
    #[serde(rename = "profile_path")]
    pub profile_path: Option<String>,
    #[serde(rename = "known_for")]
    pub known_for: Vec<KnownFor>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownFor {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: Option<String>,
    pub id: i64,
    pub title: Option<String>,
    #[serde(rename = "original_language")]
    pub original_language: String,
    #[serde(rename = "original_title")]
    pub original_title: Option<String>,
    pub overview: String,
    #[serde(rename = "poster_path")]
    pub poster_path: Option<String>,
    #[serde(rename = "genre_ids")]
    pub genre_ids: Vec<i64>,
    pub popularity: f64,
    #[serde(rename = "release_date")]
    pub release_date: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "original_name")]
    pub original_name: Option<String>,
    #[serde(rename = "first_air_date")]
    pub first_air_date: Option<String>,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorIdSearchRequest {
    pub actor_id: i64,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorIdSearchResponse {
    pub ok: bool,
    pub result: DetailedActor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedActor {
    pub adult: bool,
    #[serde(rename = "also_known_as")]
    pub also_known_as: Vec<String>,
    pub biography: String,
    pub birthday: String,
    pub gender: i64,
    pub id: i64,
    #[serde(rename = "imdb_id")]
    pub imdb_id: String,
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,
    pub name: String,
    #[serde(rename = "place_of_birth")]
    pub place_of_birth: String,
    pub popularity: f64,
    #[serde(rename = "profile_path")]
    pub profile_path: Option<String>,
}
