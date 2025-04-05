pub mod db {
    use log::debug;
    use log::{error, info};
    use serde::Deserialize;
    use serde::Serialize;
    use typeshare::typeshare;

    use crate::MonitoredSite;

    use crate::Level;

    use crate::ScrapedValue;
    use crate::WidgetConfiguration;

    use crate::MonitoredElement;
    use crate::WidgetModifier;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[typeshare]
    pub struct ScrapedData {
        pub id: i32,
        pub widget_id: String,
        pub value: String,
        pub error: Option<String>,
        pub timestamp: String,
    }
    pub struct Database {
        // data: HashMap<String, Vec<Record>>, // table -> data????
        pub(crate) connection: rusqlite::Connection,
    }

    impl Database {
        pub(crate) fn new() -> Self {
            let connection = rusqlite::Connection::open_in_memory().unwrap();
            connection
                .execute(
                    "CREATE TABLE sites (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();

            connection
                .execute(
                    "CREATE TABLE widgets (
                id INTEGER PRIMARY KEY,
                widget_id TEXT KEY NOT NULL,
                title TEXT NOT NULL,
                widget_type TEXT NOT NULL,
                level TEXT NOT NULL,
                transparent BOOL NOT NULL
            )",
                    (),
                )
                .unwrap();

            connection
                .execute(
                    "CREATE TABLE modifiers (
                id INTEGER PRIMARY KEY,
                widget_id TEXT NOT NULL,
                modifier_type TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();

            connection
                .execute(
                    "CREATE TABLE scraped_data (
                    id INTEGER PRIMARY KEY,
                    widget_id TEXT NOT NULL,
                    value TEXT NOT NULL,
                    error TEXT NOT NULL,
                    timestamp TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();

            Self { connection }
        }

        pub(crate) fn get_elements(&self) -> Result<Vec<MonitoredElement>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM elements")?;
            let elements = stmt
                .query_map([], |row| {
                    Ok(MonitoredElement {
                        id: row.get(0)?,
                        site_id: row.get(1)?,
                        selector: row.get(2)?,
                        data_key: row.get(3)?,
                    })
                })?
                .filter_map(|element| element.ok())
                .collect();
            Ok(elements)
        }

        pub(crate) fn get_configuration(
            &self,
        ) -> Result<Vec<WidgetConfiguration>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM widgets")?;
            let configuration = stmt
                .query_map([], |row| {
                    Ok(WidgetConfiguration {
                        id: row.get(0)?,
                        widget_id: row.get(1)?,
                        title: row.get(2)?,
                        widget_type: row.get(3)?,
                        level: row.get(4)?,
                        transparent: row.get(5)?,
                    })
                })?
                .filter_map(|configuration| {
                    if let Err(e) = &configuration {
                        error!("Error mapping WidgetConfiguration row: {:?}", e);
                    }
                    configuration.ok()
                })
                .collect();
            Ok(configuration)
        }

        pub(crate) fn insert_widget_configuration(
            &mut self,
            configs: Vec<WidgetConfiguration>,
        ) -> Result<(), rusqlite::Error> {
            info!("Inserting ({}) widget configurations", configs.len());
            let mut stmt = self.connection.prepare(
                "INSERT INTO widgets (widget_id, title, widget_type, level, transparent) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;
            for config in configs {
                info!("Inserting widget configuration: {:?}", config.id);
                let res = stmt.execute([
                    config.widget_id.0.as_str(),
                    config.title.as_str(),
                    serde_json::to_string(&config.widget_type).unwrap().as_str(),
                    match config.level {
                        Level::AlwaysOnTop => "AlwaysOnTop",
                        Level::Normal => "Normal",
                        Level::AlwaysOnBottom => "AlwaysOnBottom",
                    },
                    (config.transparent as i32).to_string().as_str(),
                ])?;
                info!("Inserted widget configuration: {:?}", res);
            }
            Ok(())
        }

        pub(crate) fn get_sites(&self) -> Result<Vec<MonitoredSite>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM sites")?;
            let sites = stmt
                .query_map([], |row| {
                    Ok(MonitoredSite {
                        id: row.get(0)?,
                        site_id: row.get(1)?,
                        url: row.get(2)?,
                        title: row.get(3)?,
                        refresh_interval: row.get(4)?,
                    })
                })?
                .filter_map(|site| {
                    if let Err(e) = &site {
                        error!("Error mapping MonitoredSite row: {:?}", e);
                    }
                    site.ok()
                })
                .collect();
            Ok(sites)
        }

        pub(crate) fn get_data(&self) -> Result<Vec<ScrapedData>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM scraped_data")?;
            let records = stmt
                .query_map([], |row| {
                    debug!("scraped_data row: {:?}", row);
                    Ok(ScrapedData {
                        id: row.get(0)?,
                        widget_id: row.get(1)?,
                        value: row.get(2)?,
                        error: row.get(3)?,
                        timestamp: row.get(4)?,
                    })
                })?
                .filter_map(|record| {
                    if let Err(e) = &record {
                        error!("Error mapping ScrapedData row: {:?}", e);
                    }
                    record.ok()
                })
                .collect();
            Ok(records)
        }

        pub(crate) fn get_latest_data(&self) -> Result<Vec<ScrapedData>, rusqlite::Error> {
            info!("Getting latest data");
            let mut stmt = self.connection.prepare(
                r#"SELECT *
FROM (
    SELECT *, ROW_NUMBER() OVER (PARTITION BY widget_id ORDER BY id DESC) AS rn
    FROM scraped_data
)
WHERE rn = 1"#,
            )?;
            let data = stmt
                .query_map([], |row| {
                    debug!("scraped_data row: {:?}", row);

                    Ok(ScrapedData {
                        id: row.get(0)?,
                        widget_id: row.get(1)?,
                        value: row.get(2)?,
                        error: row.get(3)?,
                        timestamp: row.get(4)?,
                    })
                })?
                .filter_map(|record| {
                    if let Err(e) = &record {
                        error!("Error mapping ScrapedData row: {:?}", e);
                    }
                    record.ok()
                })
                .collect();

            Ok(data)
        }

        pub(crate) fn insert_data(
            &mut self,
            insert_data: ScrapedValue,
        ) -> Result<(), rusqlite::Error> {
            info!("Inserting data into table: {:?}", insert_data);
            let mut stmt = self.connection.prepare(
                "INSERT INTO scraped_data (widget_id, value, error, timestamp) VALUES (?1, ?2, ?3, ?4)",
            )?;
            stmt.execute([
                insert_data.widget_id.0.as_str(),
                insert_data
                    .value
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .as_str(),
                insert_data
                    .error
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .as_str(),
                insert_data.timestamp.to_string().as_str(),
            ])?;
            Ok(())
        }

        pub fn get_modifiers(&self) -> Result<Vec<WidgetModifier>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM modifiers")?;
            let modifiers = stmt
                .query_map([], |row| {
                    Ok(WidgetModifier {
                        id: row.get(0)?,
                        widget_id: row.get(1)?,
                        modifier_type: row.get(2)?,
                    })
                })?
                .filter_map(|modifier| modifier.ok())
                .collect();
            Ok(modifiers)
        }

        pub fn insert_modifier(&mut self, modifier: WidgetModifier) -> Result<(), rusqlite::Error> {
            let mut stmt = self
                .connection
                .prepare("INSERT INTO modifiers (widget_id, modifier_type) VALUES (?1, ?2)")?;
            stmt.execute([
                &modifier.widget_id.to_string(),
                serde_json::to_string(&modifier.modifier_type)
                    .unwrap()
                    .as_str(),
            ])?;
            Ok(())
        }

        pub fn insert_widget_modifier(
            &mut self,
            widget_modifier: WidgetModifier,
        ) -> Result<(), rusqlite::Error> {
            let mut stmt = self
                .connection
                .prepare("INSERT INTO modifiers (widget_id, modifier_type) VALUES (?1, ?2)")?;
            stmt.execute([
                &widget_modifier.widget_id.to_string(),
                serde_json::to_string(&widget_modifier.modifier_type)
                    .unwrap()
                    .as_str(),
            ])?;
            Ok(())
        }

        pub fn insert_widget_modifiers(
            &mut self,
            widget_modifiers: Vec<WidgetModifier>,
        ) -> Result<(), rusqlite::Error> {
            let mut stmt = self
                .connection
                .prepare("INSERT INTO modifiers (widget_id, modifier_type) VALUES (?1, ?2)")?;
            for widget_modifier in widget_modifiers {
                // info!("Inserting widget modifier: {:?}", widget_modifier);
                stmt.execute([
                    &widget_modifier.widget_id.to_string(),
                    serde_json::to_string(&widget_modifier.modifier_type)
                        .unwrap()
                        .as_str(),
                ])?;
            }
            Ok(())
        }

        pub fn get_widget_modifiers(
            &self,
            widget_id: &str,
        ) -> Result<Vec<WidgetModifier>, rusqlite::Error> {
            let mut stmt = self
                .connection
                .prepare("SELECT * FROM modifiers WHERE widget_id = ?1")?;
            let modifiers = stmt
                .query_map([widget_id], |row| {
                    Ok(WidgetModifier {
                        id: row.get(0)?,
                        widget_id: row.get(1)?,
                        modifier_type: row.get(2)?,
                    })
                })?
                .filter_map(|modifier| modifier.ok())
                .collect();
            Ok(modifiers)
        }

        pub fn delete_widget_modifier(&mut self, modifier_id: &str) -> Result<(), rusqlite::Error> {
            let mut stmt = self
                .connection
                .prepare("DELETE FROM modifiers WHERE id = ?")?;
            stmt.execute([modifier_id])?;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{Modifier, UrlConfiguration, WidgetType};

        use super::*;

        #[test]
        fn test_modifier_roundtrip() {
            let mut db = Database::new();
            let modifier = WidgetModifier {
                id: 1,
                widget_id: crate::NanoId(String::from("1")),
                modifier_type: Modifier::Refresh { interval_sec: 30 },
            };
            db.insert_modifier(modifier).unwrap();
            let modifiers = db.get_modifiers().unwrap();
            assert_eq!(modifiers.len(), 1);
            assert_eq!(modifiers[0].id, 1);
            assert_eq!(modifiers[0].widget_id, crate::NanoId(String::from("1")));
            assert_eq!(
                modifiers[0].modifier_type,
                Modifier::Refresh { interval_sec: 30 }
            );
        }

        #[test]
        fn test_widget_configuration_roundtrip() {
            let mut db = Database::new();
            let widget_configuration = WidgetConfiguration {
                id: 0,
                widget_id: crate::NanoId(String::from("1")),
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
            assert_eq!(
                configurations[0].widget_id,
                crate::NanoId(String::from("1"))
            );
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
