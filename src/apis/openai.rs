use crate::apis::model;
use rocket::http::{Status, ContentType};
use rocket::serde::json::Json;
use serde_json::json;
use async_once::AsyncOnce;
use dotenv::dotenv;
use std::env;

lazy_static! {
    #[derive(Debug)]
    pub static ref OPEN_AI_KEY: AsyncOnce<String> = AsyncOnce::new(async {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let open_ai_key = std::env::var("OPEN_AI_KEY").expect("Unable to read OPEN_AI_KEY env var");

        open_ai_key
    });
}

#[post("/askGPT", format = "json", data = "<body>")]
pub async fn ask_gpt(body: Json<model::openai::RecommandationRequest>) -> (Status, (ContentType, String)) {

    // Create the MovieRecommendationRequest object
    let movie_request = model::openai::MovieRecommendationRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            model::openai::Message {
                role: "user".to_string(),
                content: format!("I like {} movies please recommend a list of movies i should watch, just the titles, csv format", body.preferences.join(" and ")),
            },
        ],
    };

    let client = reqwest::Client::new();
    let api_result = client
    .post("https://api.openai.com/v1/chat/completions")
    .json(&movie_request)
    .header("Authorization", format!("Bearer {}", &*OPEN_AI_KEY.get().await))
    .send()
    .await
    .unwrap();


    match api_result.json::<model::openai::ChatCompletionResponse>().await {
        Ok(parsed) => {
            let choices = parsed.choices;

            // Extract movie names from choices
            let mut movie_names: Vec<String> = vec![];
            for choice in choices {
                let content = choice.message.content;
                let movie_list: Vec<&str> = content.split(',').collect();
                movie_names.extend(movie_list.iter().map(|&m| m.trim().to_string()));
                
            }

            success_response(json!(model::openai::GptResponse {
                ok: true,
                results: movie_names
            })
            .to_string())
        }
        Err(e) => parse_error(e),
    }
}


// Return an error with details when the response of chat GPT cannot be parsed
fn parse_error(err: reqwest::Error) -> (Status, (ContentType, String)) {
    let error_response = json!({
        "ok": false,
        "reason": "Could not parse chat GPT response",
        "error_details": err.to_string(),
    })
    .to_string();

    (Status::InternalServerError, (ContentType::JSON, error_response))
}

// Augment the response with status code and content type
fn success_response(serialized_json: String) -> (Status, (ContentType, String)) {
    (Status::Accepted, (ContentType::JSON, serialized_json))
}
