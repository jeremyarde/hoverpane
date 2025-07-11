pub mod db {
    use crate::{api::api::delete_widget, db_impl::db_impl::DbTable};

    use directories::ProjectDirs;
    use log::{debug, error, info};
    // use nanoid::NanoId;
    use rusqlite::{types::FromSql, Connection, Result as SqliteResult, ToSql};
    use rusqlite_migration::{Migrations, M};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::PathBuf;
    use widget_types::{
        AppSettings, AppUiState, ConfigInformation, Level, LicenceTier, MonitorPosition, NanoId,
        ScrapedData, WidgetBounds, WidgetConfiguration, WidgetModifier, DEFAULT_WIDGET_HEIGHT,
        DEFAULT_WIDGET_WIDTH,
    };

    fn get_db_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "hoverpane", "hoverpane")
            .expect("Failed to get project directories");
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).expect("Failed to create data directory");
        data_dir.join("widgets.db")
    }

    impl DbTable for WidgetConfiguration {
        // fn get_create_table_sql() -> &'static str {
        //     r#"CREATE TABLE IF NOT EXISTS widgets (
        //         id INTEGER PRIMARY KEY,
        //         widget_id TEXT NOT NULL UNIQUE,
        //         title TEXT NOT NULL,
        //         widget_type TEXT NOT NULL,
        //         level TEXT NOT NULL,
        //         transparent INTEGER NOT NULL,
        //         decorations INTEGER NOT NULL,
        //         is_open INTEGER NOT NULL,
        //         bounds TEXT NOT NULL
        //     )"#
        // }

        fn get_insert_sql() -> &'static str {
            "INSERT INTO widgets (widget_id, title, widget_type, level, transparent, decorations, is_open, bounds) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        }
    }

    impl DbTable for WidgetModifier {
        // fn get_create_table_sql() -> &'static str {
        //     r#"
        //                 CREATE TABLE IF NOT EXISTS modifiers (
        //                     id INTEGER PRIMARY KEY,
        //                     widget_id TEXT NOT NULL,
        //                     modifier_type TEXT NOT NULL UNIQUE
        //                 )
        //         "#
        // }

        fn get_insert_sql() -> &'static str {
            "INSERT INTO modifiers (widget_id, modifier_type) VALUES (?, ?)"
        }
    }

    impl DbTable for ScrapedData {
        // fn get_create_table_sql() -> &'static str {
        //     r#"
        //         CREATE TABLE IF NOT EXISTS scraped_data (
        //             id INTEGER PRIMARY KEY,
        //             widget_id TEXT NOT NULL,
        //             value TEXT NOT NULL,
        //             error TEXT NOT NULL,
        //             timestamp TEXT NOT NULL
        //         )
        //         "#
        // }

        fn get_insert_sql() -> &'static str {
            "INSERT INTO scraped_data (widget_id, value, error, timestamp) VALUES (?, ?, ?, ?)"
        }
    }

    pub struct Database {
        conn: Connection,
    }

    impl Database {
        pub fn reset(&mut self) {
            self.conn
                .execute("DROP TABLE IF EXISTS widgets", [])
                .unwrap();
            self.conn
                .execute("DROP TABLE IF EXISTS modifiers", [])
                .unwrap();
            self.conn
                .execute("DROP TABLE IF EXISTS scraped_data", [])
                .unwrap();

            self.conn
                .execute("DROP TABLE IF EXISTS config", [])
                .unwrap();
            self.conn.execute("PRAGMA user_version = 0", []).unwrap();
            let migrations = Migrations::new(vec![
                M::up(include_str!("../migrations/20240318000000_initial.sql")),
                // M::up(WidgetConfiguration::get_create_table_sql()),
                // M::up(WidgetModifier::get_create_table_sql()),
                // M::up(ScrapedData::get_create_table_sql()),
            ]);
            migrations.to_latest(&mut self.conn).unwrap();
        }

        pub fn from(in_memory: bool) -> SqliteResult<Self> {
            // let conn = Connection::open_in_memory()?;
            let mut conn = if in_memory {
                Connection::open_in_memory()?
            } else {
                let directory =
                    directories::ProjectDirs::from("com", "jarde", "hoverpane").unwrap();
                let data_dir = directory.data_dir();
                std::fs::create_dir_all(data_dir).unwrap();
                let db_path = data_dir.join("widgets.db");

                // Connection::open(db_path)?
                match Connection::open(&db_path) {
                    Ok(conn) => conn,
                    Err(_) => {
                        // Delete the database file if it exists
                        if db_path.exists() {
                            std::fs::remove_file(&db_path).unwrap();
                        }
                        // Try opening again after deletion
                        Connection::open(&db_path)?
                    }
                }
            };
            conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
                .unwrap();

            let migrations = Migrations::new(vec![M::up(include_str!(
                "../migrations/20240318000000_initial.sql"
            ))]);
            migrations.to_latest(&mut conn).unwrap();

            Ok(Self { conn })
        }

        pub fn set_settings(&self, settings: &AppSettings) -> SqliteResult<()> {
            let json = serde_json::to_string(settings).unwrap();
            // Upsert: delete all and insert new
            self.conn.execute("DELETE FROM config", [])?;
            self.conn
                .execute("INSERT INTO config (json) VALUES (?)", [&json])?;
            Ok(())
        }

        pub fn get_settings(&self) -> SqliteResult<AppSettings> {
            let mut stmt = self.conn.prepare("SELECT json FROM config LIMIT 1")?;
            let mut rows = stmt.query([])?;
            if let Some(row) = rows.next()? {
                let json: String = row.get(0)?;
                let settings: AppSettings = match serde_json::from_str(&json) {
                    Ok(settings) => settings,
                    Err(e) => {
                        error!("Failed to parse settings: {:?}", e);
                        let defaults = AppSettings {
                            show_tray_icon: true,
                            email: "".to_string(),
                            licence_key: "".to_string(),
                            machine_id: "".to_string(),
                            licence_tier: LicenceTier::None,
                        };
                        self.set_settings(&defaults)?;
                        defaults
                    }
                };
                Ok(settings)
            } else {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }

        pub fn get_app_ui_state(&self) -> SqliteResult<AppUiState> {
            let mut stmt = self.conn.prepare("SELECT json FROM app_ui_state LIMIT 1")?;
            let mut rows = stmt.query([])?;
            if let Some(row) = rows.next()? {
                let json: String = row.get(0)?;
                let app_ui_state: AppUiState = serde_json::from_str(&json).unwrap_or(AppUiState {
                    app_settings: AppSettings {
                        show_tray_icon: true,
                        email: "".to_string(),
                        licence_key: "".to_string(),
                        machine_id: "".to_string(),
                        licence_tier: LicenceTier::None,
                    },
                    messages: vec!["Failed to load app UI state".to_string()],
                });
                Ok(app_ui_state)
            } else {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }

        pub fn set_app_ui_state(&self, app_ui_state: &AppUiState) -> SqliteResult<()> {
            let json = serde_json::to_string(app_ui_state).unwrap();
            self.conn.execute("DELETE FROM app_ui_state", [])?;
            self.conn
                .execute("INSERT INTO app_ui_state (json) VALUES (?)", [&json])?;
            Ok(())
        }

        pub fn get_widget_configuration_by_id(
            &self,
            widget_id: &str,
        ) -> SqliteResult<WidgetConfiguration> {
            let mut stmt = self
                .conn
                .prepare("SELECT * FROM widgets WHERE widget_id = ?")?;
            let widget = stmt
                .query_map([widget_id], |row| {
                    Ok(WidgetConfiguration {
                        id: row.get(0)?,
                        widget_id: NanoId(row.get(1)?),
                        title: row.get(2)?,
                        widget_type: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
                        level: serde_json::from_str(&row.get::<_, String>(4)?).unwrap(),
                        transparent: row.get::<_, i32>(5)? != 0,
                        decorations: row.get::<_, i32>(6)? != 0,
                        is_open: row.get::<_, i32>(7)? != 0,
                        bounds: serde_json::from_str(&row.get::<_, String>(8)?).unwrap(),
                    })
                })?
                .next()
                .unwrap();

            match widget {
                Ok(widget) => Ok(widget),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        pub fn get_configuration(&self) -> SqliteResult<Vec<WidgetConfiguration>> {
            let mut stmt = self.conn.prepare("SELECT * FROM widgets")?;
            let widgets = stmt
                .query_map([], |row| {
                    Ok(WidgetConfiguration {
                        id: row.get(0)?,
                        widget_id: NanoId(row.get(1)?),
                        title: row.get(2)?,
                        widget_type: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
                        level: serde_json::from_str(&row.get::<_, String>(4)?).unwrap(),
                        transparent: row.get::<_, i32>(5)? != 0,
                        decorations: row.get::<_, i32>(6)? != 0,
                        is_open: row.get::<_, i32>(7)? != 0,
                        bounds: serde_json::from_str(&row.get::<_, String>(8)?).unwrap(),
                    })
                })?
                .collect::<SqliteResult<Vec<_>>>()?;
            Ok(widgets)
        }
        pub fn upsert_widget_configuration(
            &mut self,
            config: WidgetConfiguration,
        ) -> SqliteResult<()> {
            let tx = self.conn.transaction()?;

            // Delete existing widget if it exists
            tx.execute(
                "DELETE FROM widgets WHERE widget_id = ?",
                [&config.widget_id.0],
            )?;

            // Delete associated modifiers
            tx.execute(
                "DELETE FROM modifiers WHERE widget_id = ?",
                [&config.widget_id.0],
            )?;

            // Insert the new configuration
            let widget_type = serde_json::to_string(&config.widget_type).unwrap();
            let level = serde_json::to_string(&config.level).unwrap();
            tx.execute(
                WidgetConfiguration::get_insert_sql(),
                [
                    &config.widget_id.0,
                    &config.title,
                    &widget_type,
                    &level,
                    &(config.transparent as i32).to_string(),
                    &(config.decorations as i32).to_string(),
                    &(config.is_open as i32).to_string(),
                    &serde_json::to_string(&config.bounds).unwrap(),
                ],
            )?;

            tx.commit()?;
            Ok(())
        }

        pub fn insert_widget_configuration(
            &mut self,
            configs: Vec<WidgetConfiguration>,
        ) -> SqliteResult<()> {
            let tx = self.conn.transaction()?;
            let mut stmt = tx.prepare(WidgetConfiguration::get_insert_sql())?;

            for config in configs {
                let widget_type = serde_json::to_string(&config.widget_type).unwrap();
                let level = serde_json::to_string(&config.level).unwrap();
                let position_json = serde_json::to_string(&config.bounds).unwrap();
                match stmt.execute([
                    &config.widget_id.0,
                    &config.title,
                    &widget_type,
                    &level,
                    &(config.transparent as i32).to_string(),
                    &(config.decorations as i32).to_string(),
                    &(config.is_open as i32).to_string(),
                    &position_json,
                ]) {
                    Ok(_) => (),
                    Err(e) => {
                        if e.to_string().contains("UNIQUE constraint failed") {
                            error!("Widget with ID '{}' already exists", config.widget_id.0);
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

        pub fn insert_data(&self, insert_data: ScrapedData) -> SqliteResult<()> {
            let value = insert_data.value;
            let error = insert_data.error.unwrap_or_default();
            let timestamp = insert_data.timestamp.to_string();

            self.conn.execute(
                ScrapedData::get_insert_sql(),
                [&insert_data.widget_id, &value, &error, &timestamp],
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
                WidgetModifier::get_insert_sql(),
                [&modifier.widget_id.0, &modifier_type],
            )?;
            Ok(())
        }

        pub fn insert_widget_modifier(&self, widget_modifier: WidgetModifier) -> SqliteResult<()> {
            let modifier_type = serde_json::to_string(&widget_modifier.modifier_type).unwrap();
            self.conn.execute(
                WidgetModifier::get_insert_sql(),
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
                let mut stmt = tx.prepare(WidgetModifier::get_insert_sql())?;

                for widget_modifier in widget_modifiers {
                    let modifier_type =
                        serde_json::to_string(&widget_modifier.modifier_type).unwrap();
                    stmt.execute([&widget_modifier.widget_id.0, &modifier_type])?;
                }
            }
            tx.commit()?;
            Ok(())
        }

        pub fn get_widget_modifier(&self, widget_id: &str) -> SqliteResult<Vec<WidgetModifier>> {
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

        pub fn get_all_widget_modifiers(&self) -> SqliteResult<Vec<WidgetModifier>> {
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

        pub fn update_widget_open_state(&self, widget_id: NanoId, is_open: bool) {
            let is_open_int = is_open as i32;
            self.conn
                .execute(
                    "UPDATE widgets SET is_open = ? WHERE widget_id = ?",
                    [is_open_int.to_string(), widget_id.0],
                )
                .unwrap();
        }

        pub fn update_widget_position(
            &self,
            widget_id: &str,
            new_position: &MonitorPosition,
        ) -> SqliteResult<()> {
            let position_json = serde_json::to_string(new_position).unwrap();
            self.conn.execute(
                "UPDATE widgets SET position = ? WHERE widget_id = ?",
                [position_json, widget_id.to_string()],
            )?;
            Ok(())
        }

        pub fn update_widget_position_property(
            &self,
            widget_id: &str,
            property: &str,
            value: &str,
        ) -> SqliteResult<()> {
            self.conn.execute(
                "UPDATE widgets SET position = json_set(position, ?, ?) WHERE widget_id = ?",
                [
                    format!("$.{}", property),
                    value.to_string(),
                    widget_id.to_string(),
                ],
            )?;
            Ok(())
        }

        pub fn update_widget_bounds(
            &self,
            widget_id: String,
            bounds: widget_types::WidgetBounds,
        ) -> SqliteResult<()> {
            let position_json = serde_json::to_string(&bounds).unwrap();
            self.conn.execute(
                "UPDATE widgets SET position = ? WHERE widget_id = ?",
                [position_json, widget_id.to_string()],
            )?;
            Ok(())
        }

        pub fn set_config_information(&self, config_information: Vec<ConfigInformation>) {
            let config_information_json = serde_json::to_string(&config_information).unwrap();
            self.conn
                .execute(
                    "INSERT INTO config (json) VALUES (?)",
                    [&config_information_json],
                )
                .unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use widget_types::{Modifier, UrlConfiguration, WidgetBounds, WidgetType};

        #[test]
        fn test_modifier_roundtrip() {
            let db = Database::from(true).unwrap();
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
            let mut db = Database::from(true).unwrap();
            let widget_configuration = WidgetConfiguration {
                id: 0,
                widget_id: NanoId(String::from("1")),
                title: "Test Widget".to_string(),
                widget_type: WidgetType::Url(UrlConfiguration {
                    url: "https://example.com".to_string(),
                }),
                level: Level::Normal,
                transparent: false,
                decorations: false,
                is_open: true,
                bounds: WidgetBounds {
                    x: 100,
                    y: 100,
                    width: 100,
                    height: 100,
                },
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
