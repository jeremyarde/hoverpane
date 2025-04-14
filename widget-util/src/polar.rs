// use serde_derive::Deserialize;
// use serde_derive::Serialize;

use reqwest::blocking;
use serde_json::{json, Value};

pub fn get_benefits(token: &str) {
    let client = blocking::Client::new();
    let url = "https://api.polar.sh/v1/benefits";
    let response = client.get(url).bearer_auth(token).send();
    let body = response.unwrap().text().unwrap();
    let polar_response: Value = serde_json::from_str(&body).unwrap();
    println!("{}", polar_response);
}

pub fn update_polar_benefit(
    filename: &str,
    filepath: &str,
    benefit_id: &str,
    benefit_description: &str,
    token: &str,
    file_data: &[u8],
) {
    // 1. create new file
    let new_file_url = "https://api.polar.sh/v1/files/";
    let client = blocking::Client::new();

    // upload.parts.number
    // integerrequired
    // upload.parts.chunk_start
    // integerrequired
    // upload.parts.chunk_end
    // integerrequired
    // upload.parts.checksum_sha256_base64
    // string | null
    // let checksum = sha256::digest(file_data);

    let response = client
        .post(new_file_url)
        .json(&json!({
            "name": filename,
            "mime_type": "application/x-apple-diskimage",
            "size": file_data.len(),
            "upload": {
                "parts": {
                    "number": 1,
                    "chunk_start": 0,
                    "chunk_end": file_data.len(),
                    "checksum_sha256_base64": "TODO"
                }
            },
        }))
        .bearer_auth(token)
        .send();
    let body = response.unwrap().text().unwrap();
    let polar_response: Value = serde_json::from_str(&body).unwrap();
    println!("{}", polar_response);

    let url = format!("https://api.polar.sh/v1/benefits/{}", benefit_id);
    let client = blocking::Client::new();

    // Convert the binary data to a base64 string
    // let base64_data = base64::encode(file_data);

    let response = client
        .patch(url)
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .json(&json!({
            "type": "downloadables",
            "description": benefit_description,
            "properties": {
                "files": [file_data]
            }
        }))
        .send();

    match response {
        Ok(response) => {
            let body = response.text().unwrap();
            println!("Success?");
            // println!("{}", body);
        }
        Err(e) => println!("Error: {}", e),
    }
}

// use serde::{Deserialize, Serialize};

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Root {
//     pub items: Vec<Item>,
//     pub pagination: Pagination,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Item {
//     pub created_at: String,
//     pub modified_at: String,
//     pub id: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub description: String,
//     pub selectable: bool,
//     pub deletable: bool,
//     pub organization_id: String,
//     pub properties: Properties,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Properties {
//     pub archived: Archived,
//     pub files: Vec<String>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Archived {}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Pagination {
//     pub total_count: i64,
//     pub max_page: i64,
// }
