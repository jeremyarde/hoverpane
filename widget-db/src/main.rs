mod api;
mod db;

use std::{path::PathBuf, sync::Arc};

use api::api::ApiClient;
use directories::ProjectDirs;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();

    // let proj_dirs = ProjectDirs::from("com", "widget-maker", "widget-maker")
    //     .expect("Failed to get project directories");
    // let data_dir = proj_dirs.data_dir();
    // println!("Data directory: {}", data_dir.to_str().unwrap());

    let data_dir = PathBuf::from("widgets.db");

    let db = db::db::Database::from(data_dir.to_path_buf())
        .await
        .unwrap();
    let db_clone = Arc::new(Mutex::new(db));
    api::api::run_api(db_clone).await;
}
