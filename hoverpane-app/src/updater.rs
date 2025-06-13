use cargo_packager_updater::{check_update, Config, Update};
use reqwest;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{cmp::Ordering, path::PathBuf};

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
    // update_check_url: String,
    updater_config: Config,
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
    #[error(transparent)]
    CargoPackagerError(#[from] cargo_packager_updater::Error),
}

impl Updater {
    pub fn new(current_version: &str, api_url: &str) -> Self {
        Self {
            current_version: Version::parse(current_version).expect("Invalid version format"),
            // update_check_url: update_check_url.to_string(),
            updater_config: Config {
                endpoints: vec![api_url.parse().unwrap()],
                // endpoints: vec!["https://api.hoverpane.com/apps/hoverpane/updates".parse().unwrap()],
                pubkey: include_str!("../packager_public.pub").to_string(),
                ..Default::default()
            },
        }
    }

    pub fn check_for_updates(
        &self,
        // licence_key: &str,
        // machine_id: &str,
        // email: &str,
        install: bool,
    ) -> Result<(), UpdaterError> {
        // info!("Checking for updates at {}", self.update_check_url);

        info!("Checking for updates at {:?}", self.updater_config);
        match check_update(self.current_version.clone(), self.updater_config.clone()) {
            Ok(Some(update)) => {
                info!("Update info: {:?}", update);
                update.download_and_install()?;
            }
            Err(e) => {
                error!("Error checking for updates: {:?}", e);
            }
            _ => {
                info!("No update found");
            }
        }

        Ok(())
    }

    // pub async fn download_update(
    //     &self,
    //     update_info: &UpdateInfo,
    // ) -> Result<PathBuf, Box<dyn std::error::Error>> {
    //     info!("Downloading update...");

    //     let response = reqwest::get(&update_info.download_url).await?;
    //     let bytes = response.bytes().await?;

    //     // Create a temporary file to store the update
    //     let temp_dir = std::env::temp_dir();
    //     let update_file = temp_dir.join(format!("hoverpane-update-{}.dmg", update_info.version));

    //     std::fs::write(&update_file, bytes)?;
    //     info!("Update downloaded to: {:?}", update_file);

    //     Ok(update_file)
    // }
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
        let updater = Updater::new("0.4.0", "http://localhost:3000/apps/hoverpane/updates");

        match updater.check_for_updates(false) {
            Ok(()) => {
                println!("Update found");
            }
            Err(e) => {
                error!("Error: {:?}", e);
                return;
            }
        };
    }

    #[test]
    fn test_download_update() {
        let updater = Updater::new("0.10.0", "http://localhost:3000/apps/hoverpane/updates");
        match updater.check_for_updates(false) {
            Ok(()) => {
                println!("Update found");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
            _ => {
                println!("No update found");
            }
        }
    }

    #[test]
    fn test_signature_verification() {
        let updater = Updater::new("0.13.0", "http://localhost:3000/apps/hoverpane/updates");
        let update_info = UpdateInfo {
            version: "0.13.0".to_string(),
            download_url: "http://localhost:3000/apps/hoverpane/updates".to_string(),
            release_notes: "".to_string(),
        };
        let update_file = updater.check_for_updates(false).unwrap();
    }
}
