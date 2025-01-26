
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    user_id: i32,
    title: String,
    body: String,
}