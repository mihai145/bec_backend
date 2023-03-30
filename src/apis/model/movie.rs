use serde::{Deserialize, Serialize};

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieNameSearchRequest {
    pub movie_name: String,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieNameSearchResponse {
    pub ok: bool,
    pub results: Vec<Movie>,
}

// Received from themoviedb API
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieSearchResultAPI {
    pub results: Vec<Movie>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: Option<String>,
    #[serde(rename = "genre_ids")]
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub overview: String,
    pub popularity: f64,
    #[serde(rename = "poster_path")]
    pub poster_path: Option<String>,
    #[serde(rename = "release_date")]
    pub release_date: String,
    pub title: String,
}

// Received from client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieIdSearchRequest {
    pub movie_id: i64,
}

// Returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieIdSearchResponse {
    pub ok: bool,
    pub result: DetailedMovie,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedMovie {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: Option<String>,
    pub genres: Vec<Genre>,
    pub id: i64,
    #[serde(rename = "imdb_id")]
    pub imdb_id: String,
    pub overview: String,
    pub popularity: f64,
    #[serde(rename = "poster_path")]
    pub poster_path: Option<String>,
    #[serde(rename = "release_date")]
    pub release_date: String,
    pub revenue: i64,
    pub runtime: i64,
    pub status: String,
    pub tagline: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: i64,
    pub name: String,
}
