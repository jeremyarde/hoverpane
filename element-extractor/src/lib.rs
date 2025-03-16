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
    pub fn extract(&self, url: String, selector: &str) -> Result<String, ExtractorError> {
        if selector.is_empty() {
            return Err(ExtractorError::SelectorParseError {
                selector: selector.to_string(),
                error: "Selector is empty".to_string(),
            });
        }
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
        let selector =
            Selector::parse(selector).map_err(|e| ExtractorError::SelectorParseError {
                selector: selector.to_string(),
                error: e.to_string(),
            })?;

        let mut elements = document.select(&selector);

        let mut result = String::new();
        for element in elements {
            result.push_str(&element.text().collect::<Vec<_>>().join(" "));
        }

        Ok(result)
    }

    pub fn extract_with_id(&self, url: String, id: String) -> Result<String, ExtractorError> {
        // Selector::parse(selectors)
        let url = format!("{}/{}", url, id);
        self.extract(url, "#")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let extractor = Extractor::new();
        let result = extractor.extract("https://news.ycombinator.com/item?id=43379262".to_string(), "#hnmain > tbody > tr:nth-child(3) > td > table.fatitem > tbody > tr:nth-child(2) > td.subtext > span > a:nth-child(8)");
        info!("Result: {:?}", result);

        assert!(result.is_ok());
        // assert!(!result.unwrap().is_empty());
        assert!(result.unwrap().contains("comments"));
    }
}
