use serde::{Deserialize, Serialize};

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreResponse {
    pub ok: bool,
    pub results: Vec<Genre>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenresResultApi {
    pub genres: Vec<Genre>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingSearchRequest {
    pub genre_id: i64,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingSearchResponse {
    pub ok: bool,
    pub results: Vec<TrendingMovie>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingResultApi {
    pub results: Vec<TrendingMovie>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingMovie {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: Option<String>,
    pub id: i64,
    pub title: String,
    pub overview: String,
    #[serde(rename = "poster_path")]
    pub poster_path: Option<String>,
    #[serde(rename = "genre_ids")]
    pub genre_ids: Vec<i64>,
    pub popularity: f64,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
    #[serde(rename = "vote_count")]
    pub vote_count: i64,
}