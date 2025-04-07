pub mod db {
    use directories::ProjectDirs;
    use log::{debug, error, info};
    // use nanoid::NanoId;
    use rusqlite::{Connection, Result as SqliteResult};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::PathBuf;
    use widget_types::{
        Level, NanoId, ScrapedData, ScrapedValue, WidgetConfiguration, WidgetModifier,
    };

    fn get_db_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "widget-maker", "widget-maker")
            .expect("Failed to get project directories");
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).expect("Failed to create data directory");
        data_dir.join("widgets.db")
    }

    pub struct Database {
        conn: Connection,
    }

    impl Database {
        pub fn new() -> SqliteResult<Self> {
            // let conn = Connection::open_in_memory()?;
            let conn = Connection::open(
                "/Users/jarde/Documents/code/web-extension-scraper/widget-db/widgets.db",
            )?;

            conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS widgets (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL UNIQUE,
                    title TEXT NOT NULL,
                    widget_type TEXT NOT NULL,
                    level TEXT NOT NULL,
                    transparent INTEGER NOT NULL
                )
                "#,
                [],
            )?;

            conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS modifiers (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    modifier_type TEXT NOT NULL
                )
                "#,
                [],
            )?;

            conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS scraped_data (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    value TEXT NOT NULL,
                    error TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                )
                "#,
                [],
            )?;

            Ok(Self { conn })
        }

        pub fn get_configuration(&self) -> SqliteResult<Vec<WidgetConfiguration>> {
            let mut stmt = self.conn.prepare("SELECT * FROM widgets")?;
            let rows = stmt.query_map([], |row| {
                Ok(WidgetConfiguration {
                    id: row.get(0)?,
                    widget_id: NanoId(row.get(1)?),
                    title: row.get(2)?,
                    widget_type: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
                    level: match row.get::<_, String>(4)?.as_str() {
                        "AlwaysOnTop" => Level::AlwaysOnTop,
                        "Normal" => Level::Normal,
                        "AlwaysOnBottom" => Level::AlwaysOnBottom,
                        _ => Level::Normal,
                    },
                    transparent: row.get::<_, i32>(5)? != 0,
                })
            })?;

            rows.collect()
        }

        pub fn insert_widget_configuration(
            &mut self,
            configs: Vec<WidgetConfiguration>,
        ) -> SqliteResult<()> {
            let tx = self.conn.transaction()?;
            let mut stmt = tx.prepare(
                "INSERT INTO widgets (widget_id, title, widget_type, level, transparent) VALUES (?, ?, ?, ?, ?)",
            )?;

            for config in configs {
                let widget_type = serde_json::to_string(&config.widget_type).unwrap();
                let level = serde_json::to_string(&config.level).unwrap();
                match stmt.execute([
                    &config.widget_id.0,
                    &config.title,
                    &widget_type,
                    &level,
                    &(config.transparent as i32).to_string(),
                ]) {
                    Ok(_) => (),
                    Err(e) => {
                        if e.to_string().contains("UNIQUE constraint failed") {
                            error!("Widget with ID {} already exists", config.widget_id.0);
                            drop(stmt); // Drop the statement before rolling back
                            tx.rollback()?;
                            return Err(rusqlite::Error::SqliteFailure(
                                rusqlite::ffi::Error::new(19), // SQLITE_CONSTRAINT
                                Some(format!(
                                    "Widget with ID {} already exists",
                                    config.widget_id.0
                                )),
                            ));
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            drop(stmt); // Drop the statement before committing
            tx.commit()?;
            Ok(())
        }

        pub fn get_data(&self) -> SqliteResult<Vec<ScrapedData>> {
            let mut stmt = self.conn.prepare("SELECT * FROM scraped_data")?;
            let rows = stmt.query_map([], |row| {
                Ok(ScrapedData {
                    id: row.get(0)?,
                    widget_id: row.get(1)?,
                    value: row.get(2)?,
                    error: row.get(3)?,
                    timestamp: row.get(4)?,
                })
            })?;

            rows.collect()
        }

        pub fn get_latest_data(&self) -> SqliteResult<Vec<ScrapedData>> {
            let mut stmt = self.conn.prepare(
                r#"
                SELECT *
                FROM scraped_data
                ORDER BY timestamp DESC
                LIMIT 1
                "#,
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(ScrapedData {
                    id: row.get(0)?,
                    widget_id: row.get(1)?,
                    value: row.get(2)?,
                    error: row.get(3)?,
                    timestamp: row.get(4)?,
                })
            })?;

            rows.collect()
        }

        pub fn insert_data(&self, insert_data: ScrapedValue) -> SqliteResult<()> {
            let value = insert_data.value.unwrap_or_default();
            let error = insert_data.error.unwrap_or_default();
            let timestamp = insert_data.timestamp.to_string();

            self.conn.execute(
                "INSERT INTO scraped_data (widget_id, value, error, timestamp) VALUES (?, ?, ?, ?)",
                [&insert_data.widget_id.0, &value, &error, &timestamp],
            )?;

            Ok(())
        }

        pub fn get_modifiers(&self) -> SqliteResult<Vec<WidgetModifier>> {
            let mut stmt = self.conn.prepare("SELECT * FROM modifiers")?;
            let rows = stmt.query_map([], |row| {
                Ok(WidgetModifier {
                    id: row.get(0)?,
                    widget_id: NanoId(row.get(1)?),
                    modifier_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap(),
                })
            })?;

            rows.collect()
        }

        pub fn insert_modifier(&self, modifier: WidgetModifier) -> SqliteResult<()> {
            let modifier_type = serde_json::to_string(&modifier.modifier_type).unwrap();
            self.conn.execute(
                "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)",
                [&modifier.widget_id.0, &modifier_type],
            )?;
            Ok(())
        }

        pub fn insert_widget_modifier(&self, widget_modifier: WidgetModifier) -> SqliteResult<()> {
            let modifier_type = serde_json::to_string(&widget_modifier.modifier_type).unwrap();
            self.conn.execute(
                "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)",
                [&widget_modifier.widget_id.0, &modifier_type],
            )?;
            Ok(())
        }

        pub fn insert_widget_modifiers(
            &mut self,
            widget_modifiers: Vec<WidgetModifier>,
        ) -> SqliteResult<()> {
            let tx = self.conn.transaction()?;
            {
                let mut stmt =
                    tx.prepare("INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)")?;

                for widget_modifier in widget_modifiers {
                    let modifier_type =
                        serde_json::to_string(&widget_modifier.modifier_type).unwrap();
                    stmt.execute([&widget_modifier.widget_id.0, &modifier_type])?;
                }
            }
            tx.commit()?;
            Ok(())
        }

        pub fn get_widget_modifiers(&self, widget_id: &str) -> SqliteResult<Vec<WidgetModifier>> {
            let mut stmt = self
                .conn
                .prepare("SELECT * FROM modifiers WHERE widget_id = ?")?;
            let rows = stmt.query_map([widget_id], |row| {
                Ok(WidgetModifier {
                    id: row.get(0)?,
                    widget_id: NanoId(row.get(1)?),
                    modifier_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap(),
                })
            })?;

            rows.collect()
        }

        pub fn delete_widget(&mut self, widget_id: &str) -> SqliteResult<()> {
            let tx = self.conn.transaction()?;

            // Delete associated scraped data
            tx.execute("DELETE FROM scraped_data WHERE widget_id = ?", [widget_id])?;

            // Delete associated modifiers
            tx.execute("DELETE FROM modifiers WHERE widget_id = ?", [widget_id])?;

            // Delete the widget itself
            let rows_affected =
                tx.execute("DELETE FROM widgets WHERE widget_id = ?", [widget_id])?;

            if rows_affected == 0 {
                tx.rollback()?;
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            tx.commit()?;
            Ok(())
        }

        pub fn delete_widget_modifier(&self, modifier_id: &str) -> SqliteResult<()> {
            self.conn
                .execute("DELETE FROM modifiers WHERE id = ?", [modifier_id])?;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use widget_types::{Modifier, UrlConfiguration, WidgetType};

        #[test]
        fn test_modifier_roundtrip() {
            let db = Database::new().unwrap();
            let modifier = WidgetModifier {
                id: 1,
                widget_id: NanoId(String::from("1")),
                modifier_type: Modifier::Refresh {
                    modifier_id: NanoId(String::from("1")),
                    interval_sec: 30,
                },
            };
            db.insert_modifier(modifier).unwrap();
            let modifiers = db.get_modifiers().unwrap();
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

        #[test]
        fn test_widget_configuration_roundtrip() {
            let mut db = Database::new().unwrap();
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
                .unwrap();
            let configurations = db.get_configuration().unwrap();
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
