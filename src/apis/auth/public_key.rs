use async_once::AsyncOnce;
use crate::apis::auth::models;

lazy_static!{
    #[derive(Debug)]
    pub static ref AUTH0_PKEY: AsyncOnce<Option<models::Key>> = AsyncOnce::new(async {
        let response = reqwest
        ::get("https://dev-jc1flmgwmyky8n0k.us.auth0.com/.well-known/jwks.json")
        .await.unwrap();

        match response.json::<models::PublicKey>().await {
            Ok(res) => Some(res.keys[0].clone()),
            Err(_) => None
        }
    });
}
