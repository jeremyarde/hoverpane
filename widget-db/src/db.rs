pub mod db {
    use directories::ProjectDirs;
    use log::debug;
    use log::{error, info};
    use nanoid::NanoId;
    use serde::Deserialize;
    use serde::Serialize;
    use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
    use std::fs;
    use std::path::PathBuf;
    use widget_types::{Level, ScrapedData, ScrapedValue, WidgetConfiguration, WidgetModifier};

    fn get_db_path() -> PathBuf {
        // Get the standard project directories for your app
        let proj_dirs = ProjectDirs::from("com", "widget-maker", "widget-maker")
            .expect("Failed to get project directories");

        // Use the data directory for the database
        let data_dir = proj_dirs.data_dir();

        // Create the directory if it doesn't exist
        fs::create_dir_all(data_dir).expect("Failed to create data directory");

        // Return the full path to the database file
        data_dir.join("widgets.db")
    }

    pub struct Database {
        pub(crate) pool: Pool<Sqlite>,
    }

    impl Database {
        pub async fn from(db_path: &str) -> Result<Self, sqlx::Error> {
            // Ensure parent directory exists
            // if let Some(parent) = db_path.parent() {
            //     fs::create_dir_all(parent)?;
            // }

            // let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

            // Create database if it doesn't exist
            // if !db_path.exists() {
            //     sqlx::sqlite::SqlitePoolOptions::new()
            //         .max_connections(1)
            //         .connect(&db_url)
            //         .await?;
            // }
            info!("DB path: {:?}", db_path);

            // Create connection pool
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&db_path)
                .await?;

            // Run migrations if they exist
            sqlx::migrate!("./migrations").run(&pool).await?;

            // Create tables if they don't exist
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS sites (
                    id INTEGER PRIMARY KEY,
                    url TEXT NOT NULL,
                    title TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS widgets (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    title TEXT NOT NULL,
                    widget_type TEXT NOT NULL,
                    level TEXT NOT NULL,
                    transparent INTEGER NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS modifiers (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    modifier_type TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS scraped_data (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    value TEXT NOT NULL,
                    error TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                )
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self { pool })
        }

        pub async fn get_configuration(&self) -> Result<Vec<WidgetConfiguration>, sqlx::Error> {
            let rows = sqlx::query!("SELECT * FROM widgets")
                .fetch_all(&self.pool)
                .await?;

            let configurations = rows
                .into_iter()
                .map(|row| WidgetConfiguration {
                    id: row.id,
                    widget_id: NanoId(row.widget_id),
                    title: row.title,
                    widget_type: serde_json::from_str(&row.widget_type).unwrap(),
                    level: match row.level.as_str() {
                        "AlwaysOnTop" => Level::AlwaysOnTop,
                        "Normal" => Level::Normal,
                        "AlwaysOnBottom" => Level::AlwaysOnBottom,
                        _ => Level::Normal,
                    },
                    transparent: row.transparent != 0,
                })
                .collect();

            Ok(configurations)
        }

        pub async fn insert_widget_configuration(
            &self,
            configs: Vec<WidgetConfiguration>,
        ) -> Result<(), sqlx::Error> {
            for config in configs {
                let widget_type = serde_json::to_string(&config.widget_type).unwrap();
                let level = serde_json::to_string(&config.level).unwrap();
                let transparent = serde_json::to_string(&config.transparent).unwrap();
                sqlx::query!(
                    "INSERT INTO widgets (widget_id, title, widget_type, level, transparent) VALUES (?, ?, ?, ?, ?)",
                    config.widget_id.0,
                    config.title,
                    widget_type,
                    level,
                    config.transparent
                )
                .execute(&self.pool)
                .await?;
            }
            Ok(())
        }

        // pub async fn get_sites(&self) -> Result<Vec<MonitoredSite>, sqlx::Error> {
        //     let rows = sqlx::query!("SELECT * FROM sites")
        //         .fetch_all(&self.pool)
        //         .await?;

        //     let sites = rows
        //         .into_iter()
        //         .map(|row| MonitoredSite {
        //             id: row.id,
        //             site_id: row.site_id,
        //             url: row.url,
        //             title: row.title,
        //             refresh_interval: row.refresh_interval,
        //         })
        //         .collect();

        //     Ok(sites)
        // }

        pub async fn get_data(&self) -> Result<Vec<ScrapedData>, sqlx::Error> {
            sqlx::query_as!(ScrapedData, "SELECT * FROM scraped_data")
                .fetch_all(&self.pool)
                .await
        }

        pub async fn get_latest_data(&self) -> Result<Vec<ScrapedData>, sqlx::Error> {
            sqlx::query_as!(
                ScrapedData,
                r#"
                SELECT *
                FROM scraped_data
                ORDER BY timestamp DESC
                LIMIT 1
                "#
            )
            .fetch_all(&self.pool)
            .await
        }

        pub async fn insert_data(&self, insert_data: ScrapedValue) -> Result<(), sqlx::Error> {
            let value = insert_data.value.unwrap_or_default();
            let error = insert_data.error.unwrap_or_default();
            let timestamp = insert_data.timestamp.to_string();
            sqlx::query!(
                "INSERT INTO scraped_data (widget_id, value, error, timestamp) VALUES (?, ?, ?, ?)",
                insert_data.widget_id.0,
                value,
                error,
                timestamp
            )
            .execute(&self.pool)
            .await?;
            Ok(())
        }

        pub async fn get_modifiers(&self) -> Result<Vec<WidgetModifier>, sqlx::Error> {
            let rows = sqlx::query!("SELECT * FROM modifiers")
                .fetch_all(&self.pool)
                .await?;

            let modifiers = rows
                .into_iter()
                .map(|row| WidgetModifier {
                    id: row.id,
                    widget_id: NanoId(row.widget_id),
                    modifier_type: serde_json::from_str(&row.modifier_type).unwrap(),
                })
                .collect();

            Ok(modifiers)
        }

        pub async fn insert_modifier(&self, modifier: WidgetModifier) -> Result<(), sqlx::Error> {
            let modifier_type = serde_json::to_string(&modifier.modifier_type).unwrap();
            sqlx::query!(
                "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)",
                modifier.widget_id.0,
                modifier_type
            )
            .execute(&self.pool)
            .await?;
            Ok(())
        }

        pub async fn insert_widget_modifier(
            &self,
            widget_modifier: WidgetModifier,
        ) -> Result<(), sqlx::Error> {
            let modifier_type = serde_json::to_string(&widget_modifier.modifier_type).unwrap();
            sqlx::query!(
                "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)",
                widget_modifier.widget_id.0,
                modifier_type
            )
            .execute(&self.pool)
            .await?;
            Ok(())
        }

        pub async fn insert_widget_modifiers(
            &self,
            widget_modifiers: Vec<WidgetModifier>,
        ) -> Result<(), sqlx::Error> {
            for widget_modifier in widget_modifiers {
                let modifier_type = serde_json::to_string(&widget_modifier.modifier_type).unwrap();
                sqlx::query!(
                    "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)",
                    widget_modifier.widget_id.0,
                    modifier_type
                )
                .execute(&self.pool)
                .await?;
            }
            Ok(())
        }

        pub async fn get_widget_modifiers(
            &self,
            widget_id: &str,
        ) -> Result<Vec<WidgetModifier>, sqlx::Error> {
            let rows = sqlx::query!("SELECT * FROM modifiers WHERE widget_id = ?", widget_id)
                .fetch_all(&self.pool)
                .await?;

            let modifiers = rows
                .into_iter()
                .map(|row| WidgetModifier {
                    id: row.id,
                    widget_id: NanoId(row.widget_id),
                    modifier_type: serde_json::from_str(&row.modifier_type).unwrap(),
                })
                .collect();

            Ok(modifiers)
        }

        pub async fn delete_widget_modifier(&self, modifier_id: &str) -> Result<(), sqlx::Error> {
            sqlx::query!("DELETE FROM modifiers WHERE id = ?", modifier_id)
                .execute(&self.pool)
                .await?;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use widget_types::{Modifier, UrlConfiguration, WidgetType};

        use super::*;

        #[tokio::test]
        async fn test_modifier_roundtrip() {
            let mut db = Database::from(PathBuf::from("widgets.db")).await.unwrap();
            let modifier = WidgetModifier {
                id: 1,
                widget_id: NanoId(String::from("1")),
                modifier_type: Modifier::Refresh {
                    modifier_id: NanoId(String::from("1")),
                    interval_sec: 30,
                },
            };
            db.insert_modifier(modifier).await.unwrap();
            let modifiers = db.get_modifiers().await.unwrap();
            assert_eq!(modifiers.len(), 1);
            assert_eq!(modifiers[0].id, 1);
            assert_eq!(modifiers[0].widget_id, NanoId(String::from("1")));
            assert_eq!(
                modifiers[0].modifier_type,
                Modifier::Refresh {
                    modifier_id: NanoId(String::from("1")),
                    interval_sec: 30
                }
            );
        }

        #[tokio::test]
        async fn test_widget_configuration_roundtrip() {
            let mut db = Database::from(PathBuf::from("widgets.db")).await.unwrap();
            let widget_configuration = WidgetConfiguration {
                id: 0,
                widget_id: NanoId(String::from("1")),
                title: "Test Widget".to_string(),
                widget_type: WidgetType::Url(UrlConfiguration {
                    url: "https://example.com".to_string(),
                }),
                level: Level::Normal,
                transparent: false,
            };
            db.insert_widget_configuration(vec![widget_configuration])
                .await
                .unwrap();
            let configurations = db.get_configuration().await.unwrap();
            assert_eq!(configurations.len(), 1);
            assert_eq!(configurations[0].widget_id, NanoId(String::from("1")));
            assert_eq!(configurations[0].title, "Test Widget");
            assert_eq!(
                configurations[0].widget_type,
                WidgetType::Url(UrlConfiguration {
                    url: "https://example.com".to_string(),
                })
            );
            assert_eq!(configurations[0].level, Level::Normal);
        }
    }
}
