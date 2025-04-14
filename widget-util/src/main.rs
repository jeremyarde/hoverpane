use std::collections::HashMap;

use reqwest::blocking;
use serde;

mod polar;
// use polar::Root;
use serde_json::{json, Value};

fn main() {
    dotenvy::dotenv().ok();
    let token = dotenvy::var("POLAR_BENEFIT_TOKEN").unwrap();
}
