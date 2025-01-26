use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tokio;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub session_id: String,
    pub survey_id: String,
    pub start_date: DateTime<Utc>,
    pub first_response_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub t: String,
    pub m: String,
    pub edf: String,
    pub created_at: Option<DateTime<Utc>>, // Optionnel car il a une valeur par défaut
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // GET request
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        println!("Error : {}", response.status());
        return Ok(());
    }

    // Deserialize data
    let answer: Session = response.json().await?; // ! we can get many data to insert

    // Connect DB
    let db_url = "postgres://username:password@localhost:5432/mydatabase"; // À adapter
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Insert in DB
    sqlx::query!(
        "INSERT INTO answers (session_id, survey_id, start_date, first_response_date, end_date, t, m, edf)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (session_id) DO UPDATE
         SET survey_id = $2, start_date = $3, first_response_date = $4, end_date = $5, t = $6, m = $7, edf = $8",
        answer.session_id,
        answer.survey_id,
        answer.start_date,
        answer.first_response_date,
        answer.end_date,
        answer.t,
        answer.m,
        answer.edf
    )
    .execute(&pool)
    .await?;

    println!("Post request succeeded!");
    Ok(())
}
