use scraper::{Html, Selector};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("Request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Selector parse error: {selector:?}, {error:?}")]
    SelectorParseError { selector: String, error: String },
}

pub struct Extractor {
    // url: String,
    // selector: String,
    client: reqwest::blocking::Client,
}

impl Extractor {
    pub fn new() -> Self {
        Self {
            // url,
            // selector,
            client: reqwest::blocking::Client::builder()
                .user_agent("gethashdown.com")
                .build()
                .unwrap(),
        }
    }
    fn extract(&self, url: String, selector: &str) -> Result<String, ExtractorError> {
        let client_response = self.client.get(url).send();

        let mut response = match client_response {
            Ok(response) => response,
            Err(e) => {
                info!("Error: {:?}", e);
                return Err(ExtractorError::RequestError(e));
            }
        };

        info!("Response status: {:?}", &response.status());
        let text = response.text().unwrap();
        info!("Response text: {:?}", &text);

        let document = Html::parse_document(&text);
        let selector = Selector::parse(selector).unwrap();

        let mut elements = document.select(&selector);

        let mut result = String::new();
        for element in elements {
            result.push_str(&element.text().collect::<Vec<_>>().join(" "));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
