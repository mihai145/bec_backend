use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use async_once::AsyncOnce;

lazy_static!{
    #[derive(Debug)]
    pub static ref PG: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
        let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

        pool
    });
}
