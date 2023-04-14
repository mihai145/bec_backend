use crate::apis::model;
use rocket::http::{Status, ContentType};
use rocket::serde::json::Json;
use serde_json::json;

// move this in a Kubernetes secret later
const MOVIE_DB_API_KEY: &str = "94f0de158bc1f10413f2ac668010303d";

#[post("/search/movieName", format="json", data="<body>")]
pub async fn search_movie_name(body: Json<model::movie::MovieNameSearchRequest>) -> (Status, (ContentType, String)) {
    let movie_name = &body.movie_name;

    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/search/movie?api_key={MOVIE_DB_API_KEY}&language=en-US&query={movie_name}&page=1&include_adult=false"))
            .await.unwrap();
    
    match api_result.json::<model::movie::MovieSearchResultAPI>().await {
        Ok(parsed) => {
            let mut movies = parsed.results;
            
            movies = movies.iter().filter(|x| x.overview.len() > 0).cloned().collect();

            for movie in &mut movies {
                augment_image_paths_movie(movie);
            }

            success_response(json!(model::movie::MovieNameSearchResponse{
                ok: true,
                results: movies
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

#[post("/search/movieId", format="json", data="<body>")]
pub async fn search_movie_id(body: Json<model::movie::MovieIdSearchRequest>) -> (Status, (ContentType, String)) {
    let movie_id = &body.movie_id;

    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/movie/{movie_id}?api_key={MOVIE_DB_API_KEY}&language=en-US"))
            .await.unwrap();
    
    match api_result.json::<model::movie::DetailedMovie>().await {
        Ok(parsed) => {
            let mut movie = parsed;
            augment_image_paths_detailed_movie(&mut movie);

            success_response(json!(model::movie::MovieIdSearchResponse{
                ok: true,
                result: movie
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

#[post("/search/actorName", format="json", data="<body>")]
pub async fn search_actor_name(body: Json<model::actor::ActorNameSearchRequest>) -> (Status, (ContentType, String)) {
    let actor_name = &body.actor_name;

    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/search/person?api_key={MOVIE_DB_API_KEY}&language=en-US&query={actor_name}&page=1&include_adult=false"))
            .await.unwrap();
    
    match api_result.json::<model::actor::ActorSearchResultApi>().await {
        Ok(parsed) => {
            let mut actors = parsed.results;

            actors = actors.iter().filter(|x| x.known_for.len() > 0 
                            && x.name.len() > 0 
                            && x.name.chars().all(|c| !c.is_ascii_digit())).cloned().collect();

            for actor in &mut actors {
                for movie in &mut actor.known_for {
                    augment_image_paths_known_for(movie);
                }
                augment_profile_path_actor(actor);
            }

            success_response(json!(model::actor::ActorNameSearchResponse{
                ok: true,
                results: actors
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

#[post("/search/actorId", format="json", data="<body>")]
pub async fn search_actor_id(body: Json<model::actor::ActorIdSearchRequest>) -> (Status, (ContentType, String)) {
    let actor_id = &body.actor_id;

    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/person/{actor_id}?api_key={MOVIE_DB_API_KEY}&language=en-US"))
            .await.unwrap();
    
    match api_result.json::<model::actor::DetailedActor>().await {
        Ok(mut parsed) => {
            augment_profile_path_detailed_actor(&mut parsed);

            success_response(json!(model::actor::ActorIdSearchResponse{
                ok: true,
                result: parsed
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

#[get("/genres")]
pub async fn get_genres() -> (Status, (ContentType, String)) {
    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/genre/movie/list?api_key={MOVIE_DB_API_KEY}&language=en-US"))
            .await.unwrap();

    match api_result.json::<model::genre::GenresResultApi>().await {
        Ok(parsed) => {
            success_response(json!(model::genre::GenreResponse{
                ok: true,
                results: parsed.genres
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

#[post("/trending", format="json", data="<body>")]
pub async fn get_trending(body: Json<model::genre::TrendingSearchRequest>) -> (Status, (ContentType, String)) {
    let genre_id = &body.genre_id;

    let api_result = reqwest
            ::get(format!("https://api.themoviedb.org/3/trending/movie/week?api_key={MOVIE_DB_API_KEY}"))
            .await.unwrap();
    
    match api_result.json::<model::genre::TrendingResultApi>().await {
        Ok(mut parsed) => {
            let mut filtered_movies = Vec::new();
            for movie in &mut parsed.results {
                if movie.genre_ids.contains(genre_id) {
                    augment_image_paths_trending_movie(movie);
                    filtered_movies.push(movie.clone());
                }
            }

            success_response(json!(model::genre::TrendingSearchResponse{
                ok: true,
                results: filtered_movies
            }).to_string())
        }
        Err(_) => parse_error()
    }
}

// Auxiliary functions to include the full URL to movie posters
// Rust does not support function overloading :(
const MOVIE_DB_IMAGE_PREFIX: &str = "https://image.tmdb.org/t/p/original";

fn augment_image_paths_movie(movie: &mut model::movie::Movie) {
    match &movie.backdrop_path {
        Some(path) => movie.backdrop_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.backdrop_path = Some(String::from(""))
    }
    match &movie.poster_path {
        Some(path) => movie.poster_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.poster_path = Some(String::from(""))
    }
}

fn augment_image_paths_detailed_movie(movie: &mut model::movie::DetailedMovie) {
    match &movie.backdrop_path {
        Some(path) => movie.backdrop_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.backdrop_path = Some(String::from(""))
    }
    match &movie.poster_path {
        Some(path) => movie.poster_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.poster_path = Some(String::from(""))
    }
}

fn augment_image_paths_known_for(movie: &mut model::actor::KnownFor) {
    match &movie.backdrop_path {
        Some(path) => movie.backdrop_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.backdrop_path = Some(String::from(""))
    }
    match &movie.poster_path {
        Some(path) => movie.poster_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.poster_path = Some(String::from(""))
    }
}

fn augment_image_paths_trending_movie(movie: &mut model::genre::TrendingMovie) {
    match &movie.backdrop_path {
        Some(path) => movie.backdrop_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.backdrop_path = Some(String::from(""))
    }
    match &movie.poster_path {
        Some(path) => movie.poster_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => movie.poster_path = Some(String::from(""))
    }
}

fn augment_profile_path_actor(actor: &mut model::actor::Actor) {
    match &actor.profile_path {
        Some(path) => actor.profile_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => actor.profile_path = Some(String::from(""))
    }
}

fn augment_profile_path_detailed_actor(actor: &mut model::actor::DetailedActor) {
    match &actor.profile_path {
        Some(path) => actor.profile_path = Some([MOVIE_DB_IMAGE_PREFIX, &path].join("")),
        None => actor.profile_path = Some(String::from(""))
    }
}

// Return a generic error when the response of themoviedb cannot be parsed
fn parse_error() -> (Status, (ContentType, String)) {
    let error_response = json!(model::error::Error{
        ok: false,
        reason: String::from("Could not parse themoviedb response")
    }).to_string();

    (Status::InternalServerError, (ContentType::JSON, error_response))
}

// Augment the response with status code and content type
fn success_response(serialized_json: String) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, serialized_json))
}