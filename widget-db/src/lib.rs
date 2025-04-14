mod api;
mod db;
mod db_impl;

use std::{path::PathBuf, sync::Arc};

use directories::ProjectDirs;
use tokio::sync::Mutex;

pub use api::api::run_api;
pub use db::db::Database;
