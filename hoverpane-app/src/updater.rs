use reqwest;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{cmp::Ordering, path::PathBuf};
// use thiserror::Error;

#[cfg(not(test))]
use log::{error, info, warn};

#[cfg(test)]
use std::{println as info, println as warn, println as error};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub release_notes: String,
}

#[derive(Debug, Clone)]
pub struct Updater {
    current_version: Version,
    update_check_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetLatestVersionPayload {
    pub licence_key: String,
    pub machine_id: String,
    pub email: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdaterError {
    #[error("User email is empty")]
    UserEmailEmpty,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SemverError(#[from] semver::Error),
}

impl Updater {
    pub fn new(current_version: &str, update_check_url: &str) -> Self {
        Self {
            current_version: Version::parse(current_version).expect("Invalid version format"),
            update_check_url: update_check_url.to_string(),
        }
    }

    pub async fn check_for_updates(
        &self,
        licence_key: &str,
        machine_id: &str,
        email: &str,
    ) -> Result<Option<UpdateInfo>, UpdaterError> {
        if email.is_empty() {
            return Err(UpdaterError::UserEmailEmpty);
        }

        info!("Checking for updates at {}", self.update_check_url);

        let client = reqwest::Client::new();

        let response = client
            .post(&self.update_check_url)
            .json(&GetLatestVersionPayload {
                licence_key: licence_key.to_string(),
                machine_id: machine_id.to_string(),
                email: email.to_string(),
            })
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(UpdaterError::ReqwestError)?
            .error_for_status()
            .map_err(UpdaterError::ReqwestError)?;

        let update_info: UpdateInfo = response.json().await.map_err(UpdaterError::ReqwestError)?;
        info!("Update info: {:?}", update_info);

        let latest_version = Version::parse(&update_info.version)?;

        match latest_version.cmp(&self.current_version) {
            Ordering::Greater => {
                info!("New version available: {}", latest_version);
                Ok(Some(update_info))
            }
            Ordering::Equal => {
                info!("Already using the latest version");
                Ok(None)
            }
            Ordering::Less => {
                info!("Current version is newer than the latest version");
                Ok(None)
            }
        }
    }

    pub async fn download_update(
        &self,
        update_info: &UpdateInfo,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        info!("Downloading update...");

        let response = reqwest::get(&update_info.download_url).await?;
        let bytes = response.bytes().await?;

        // Create a temporary file to store the update
        let temp_dir = std::env::temp_dir();
        let update_file = temp_dir.join(format!("hoverpane-update-{}.dmg", update_info.version));

        std::fs::write(&update_file, bytes)?;
        info!("Update downloaded to: {:?}", update_file);

        Ok(update_file)
    }
}

#[cfg(test)]
mod tests {
    use crate::DesktopAppSettings;

    use super::*;

    #[cfg(not(test))]
    use log::{info, warn};
    use widget_types::AppSettings;

    #[cfg(test)]
    use std::{println as info, println as warn};

    #[tokio::test]
    async fn test_version_comparison() {
        let updater = Updater::new("0.4.0", "http://localhost:3000/apps/hoverpane/latest");

        let update_info = match updater
            .check_for_updates("1234567890", "1234567890", "test@test.com")
            .await
        {
            Ok(update_info) => update_info,
            Err(e) => {
                error!("Error: {:?}", e);
                return;
            }
        };
        println!("Update info: {:?}", update_info);
        assert!(update_info.is_some());
    }
}
