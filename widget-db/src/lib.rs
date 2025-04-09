mod api;
mod db;
mod db_impl;

use std::{path::PathBuf, sync::Arc};

use api::api::ApiClient;
use directories::ProjectDirs;
use tokio::sync::Mutex;

// pub use api::api::ApiClient;
pub use api::api::run_api;
pub use db::db::Database;
// #[tokio::main]
// async fn main() {
//     // let app = App {
//     //     db: Arc::new(Mutex::new(
//     //         db::db::Database::from(PathBuf::from("widgets.db"))
//     //             .await
//     //             .unwrap(),
//     //     )),
//     // };

//     // dotenvy::dotenv().ok();

//     // let proj_dirs = ProjectDirs::from("com", "widget-maker", "widget-maker")
//     //     .expect("Failed to get project directories");
//     // let data_dir = proj_dirs.data_dir();
//     // println!("Data directory: {}", data_dir.to_str().unwrap());

//     let data_dir = PathBuf::from("widgets.db");

//     let db = db::db::Database::from(data_dir.to_path_buf())
//         .await
//         .unwrap();
//     let db_clone = Arc::new(Mutex::new(db));
//     api::api::run_api(db_clone).await;
// }
