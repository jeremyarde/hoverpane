pub mod db {
    use log::info;
    use nanoid::NanoId;

    use crate::Record;

    use crate::MonitoredSite;

    use crate::Level;

    use crate::WidgetConfiguration;

    use crate::MonitoredElement;
    use crate::WidgetModifier;

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
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();
            connection
                .execute(
                    "CREATE TABLE elements (
                id TEXT PRIMARY KEY,
                site_id TEXT NOT NULL,
                selector TEXT NOT NULL,
                data_key TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();

            connection
                .execute(
                    "CREATE TABLE widgets (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                widget_type TEXT NOT NULL,
                level TEXT NOT NULL
            )",
                    (),
                )
                .unwrap();

            connection
                .execute(
                    "CREATE TABLE modifiers (
                id TEXT PRIMARY KEY,
                widget_id TEXT NOT NULL,
                modifier_type TEXT NOT NULL
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
                    // info!("querying row: {:?}", row);
                    Ok(WidgetConfiguration {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        widget_type: row.get(2)?,
                        level: row.get(3)?,
                    })
                })?
                .filter_map(|configuration| {
                    if let Err(e) = &configuration {
                        info!("Error mapping row: {:?}", e);
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
            let mut stmt = self.connection.prepare(
                "INSERT INTO widgets (id, title, widget_type, level) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for config in configs {
                info!("Inserting widget configuration: {:?}", config.id);
                let res = stmt.execute([
                    config.id.0.as_str(),
                    config.title.as_str(),
                    serde_json::to_string(&config.widget_type).unwrap().as_str(),
                    match config.level {
                        Level::AlwaysOnTop => "AlwaysOnTop",
                        Level::Normal => "Normal",
                        Level::AlwaysOnBottom => "AlwaysOnBottom",
                    },
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
                .filter_map(|site| site.ok())
                .collect();
            Ok(sites)
        }

        pub(crate) fn get_data(&self) -> Result<Vec<Record>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT * FROM test")?;
            let records = stmt
                .query_map([], |row| {
                    info!("querying row: {:?}", row);
                    Ok(Record {
                        id: row.get(0)?,
                        window_id: row.get(1)?,
                        data: row.get(2)?,
                    })
                })?
                .filter_map(|record| record.ok())
                .collect();
            Ok(records)
        }

        pub(crate) fn get_latest_data(&self) -> Result<Vec<Record>, rusqlite::Error> {
            info!("Getting latest data");
            let mut stmt = self.connection.prepare(
                r#"SELECT *
FROM (
    SELECT *, ROW_NUMBER() OVER (PARTITION BY window_id ORDER BY id DESC) AS rn
    FROM test
)
WHERE rn = 1"#,
            )?;
            let data = stmt
                .query_map([], |row| {
                    Ok(Record {
                        id: row.get(0)?,
                        window_id: row.get(1)?,
                        data: row.get(2)?,
                    })
                })?
                .filter_map(|record| record.ok())
                .collect();

            Ok(data)
        }

        pub(crate) fn insert_data(
            &mut self,
            table: &str,
            insert_data: Record,
        ) -> Result<(), rusqlite::Error> {
            info!("Inserting data into table: {}, {:?}", table, insert_data);
            let mut stmt = self
                .connection
                .prepare("INSERT INTO test (window_id, data) VALUES (?1, ?2)")?;
            stmt.execute([insert_data.window_id, insert_data.data])?;
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
            let mut stmt = self.connection.prepare(
                "INSERT INTO modifiers (id, widget_id, modifier_type) VALUES (?1, ?2, ?3)",
            )?;
            stmt.execute([
                modifier.id.as_str(),
                &modifier.widget_id.to_string(),
                serde_json::to_string(&modifier.modifier_type)
                    .unwrap()
                    .as_str(),
            ])?;
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
                id: "1".to_string(),
                widget_id: crate::NanoId(String::from("1")),
                modifier_type: Modifier::Refresh {},
            };
            db.insert_modifier(modifier).unwrap();
            let modifiers = db.get_modifiers().unwrap();
            assert_eq!(modifiers.len(), 1);
            assert_eq!(modifiers[0].id, "1");
            assert_eq!(modifiers[0].widget_id, crate::NanoId(String::from("1")));
            assert_eq!(modifiers[0].modifier_type, Modifier::Refresh {});
        }

        #[test]
        fn test_widget_configuration_roundtrip() {
            let mut db = Database::new();
            let widget_configuration = WidgetConfiguration {
                id: crate::NanoId(String::from("1")),
                title: "Test Widget".to_string(),
                widget_type: WidgetType::Url(UrlConfiguration {
                    url: "https://example.com".to_string(),
                }),
                level: Level::Normal,
            };
            db.insert_widget_configuration(vec![widget_configuration])
                .unwrap();
            let configurations = db.get_configuration().unwrap();
            assert_eq!(configurations.len(), 1);
            assert_eq!(configurations[0].id, crate::NanoId(String::from("1")));
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
