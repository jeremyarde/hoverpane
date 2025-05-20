// use log::{error, info};
use reqwest;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, path::PathBuf};

#[cfg(not(test))]
use log::{info, warn};

#[cfg(test)]
use std::{println as info, println as warn};

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

impl Updater {
    pub fn new(current_version: &str, update_check_url: &str) -> Self {
        Self {
            current_version: Version::parse(current_version).expect("Invalid version format"),
            update_check_url: update_check_url.to_string(),
        }
    }

    pub async fn check_for_updates(
        &self,
    ) -> Result<Option<UpdateInfo>, Box<dyn std::error::Error>> {
        info!("Checking for updates...");

        let response = reqwest::get(&self.update_check_url).await?;
        let update_info: UpdateInfo = response.json().await?;
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
    use super::*;

    #[cfg(not(test))]
    use log::{info, warn};

    #[cfg(test)]
    use std::{println as info, println as warn};

    #[tokio::test]
    async fn test_version_comparison() {
        let updater = Updater::new("0.4.0", "http://localhost:3001/apps/hoverpane/latest");

        let update_info = updater.check_for_updates().await.unwrap();
        println!("Update info: {:?}", update_info);
        assert!(update_info.is_some());
    }
}
