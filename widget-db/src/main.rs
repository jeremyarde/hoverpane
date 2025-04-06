mod api;
mod db;

use api::api::ApiClient;

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let db = db::db::Database::new().await.unwrap();
}
