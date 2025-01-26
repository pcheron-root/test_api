use reqwest;
// use serde::Deserialize;
use serde_derive::Deserialize;
// use sqlx::postgres::PgPoolOptions;
use tokio;

#[derive(Deserialize, Debug)] // Ajoute Deserialize pour permettre la conversion JSON -> Rust
struct Post {
    // user_id: i32,
    // title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL de l'API
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Étape 1 : Récupérer les données depuis l'API
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        println!("Erreur : {}", response.status());
        return Ok(()); // Sortie si la requête a échoué
    }

    println!("Post récupéré : {:?}", response);

    // Désérialiser la réponse JSON en une struct Post
    let post: Post = response.json().await?;
    println!("Post récupéré : {:?}", post);

    // Étape 2 : Connexion à la base de données
    // let db_url = "postgres://username:password@localhost/mydatabase"; // À adapter
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(db_url)
    //     .await?;

    // Étape 3 : Insérer les données dans la base
    // sqlx::query!(
    //     "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3)",
    //     post.user_id,
    //     post.title,
    //     post.body
    // )
    // .execute(&pool)
    // .await?;

    // println!("Post inséré avec succès dans la base de données !");
    Ok(())
}

