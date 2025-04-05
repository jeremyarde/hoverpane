use sqlx::{Pool, Sqlite};
use std::error::Error;
use log::error;
use crate::{MonitoredElement, MonitoredSite, WidgetConfiguration, WidgetModifier, Level};
use crate::NanoId;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let pool = sqlx::SqlitePool::connect("sqlite:widgets.db").await?;
        Ok(Database { pool })
    }

    pub async fn get_scraped_data(&self) -> Result<Vec<ScrapedData>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", widget_id, value, error, timestamp 
            FROM scraped_data
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ScrapedData {
                id: Some(row.id),
                widget_id: row.widget_id,
                value: Some(row.value),
                error: Some(row.error),
                timestamp: row.timestamp,
            })
            .collect())
    }

    pub async fn get_widgets(&self) -> Result<Vec<WidgetConfiguration>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", widget_id, title, widget_type, level, transparent 
            FROM widgets
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let widgets = rows
            .into_iter()
            .map(|row| WidgetConfiguration {
                id: Some(row.id as u32),
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

        Ok(widgets)
    }

    pub async fn create_widget(&self, config: &WidgetConfiguration) -> Result<(), Box<dyn Error>> {
        let widget_id = config.widget_id.0.clone();
        let title = config.title.clone();
        let widget_type = serde_json::to_string(&config.widget_type)?;
        let level = match config.level {
            Level::AlwaysOnTop => "AlwaysOnTop",
            Level::Normal => "Normal",
            Level::AlwaysOnBottom => "AlwaysOnBottom",
        };
        let transparent = config.transparent as i32;

        sqlx::query!(
            r#"
            INSERT INTO widgets (widget_id, title, widget_type, level, transparent) 
            VALUES (?, ?, ?, ?, ?)
            "#,
            widget_id,
            title,
            widget_type,
            level,
            transparent
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_sites(&self) -> Result<Vec<MonitoredSite>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", url, title 
            FROM sites
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| MonitoredSite {
                id: row.id as i32,
                url: row.url,
                title: row.title,
            })
            .collect())
    }

    pub async fn get_elements(&self) -> Result<Vec<MonitoredElement>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", widget_id, selector, name 
            FROM elements
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| MonitoredElement {
                id: row.id as i32,
                widget_id: NanoId(row.widget_id),
                selector: row.selector,
                name: row.name,
            })
            .collect())
    }

    pub async fn get_latest_data(&self) -> Result<Vec<ScrapedData>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", widget_id, value, error, timestamp
            FROM scraped_data
            WHERE id IN (
                SELECT MAX(id) 
                FROM scraped_data 
                GROUP BY widget_id
            )
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ScrapedData {
                id: Some(row.id),
                widget_id: row.widget_id,
                value: Some(row.value),
                error: Some(row.error),
                timestamp: row.timestamp,
            })
            .collect())
    }

    pub async fn insert_scraped_data(&self, insert_data: &ScrapedData) -> Result<(), Box<dyn Error>> {
        let widget_id = &insert_data.widget_id;
        let value = insert_data.value.clone().unwrap_or_default();
        let error = insert_data.error.clone().unwrap_or_default();
        let timestamp = &insert_data.timestamp;

        sqlx::query!(
            r#"
            INSERT INTO scraped_data (widget_id, value, error, timestamp) 
            VALUES (?, ?, ?, ?)
            "#,
            widget_id,
            value,
            error,
            timestamp
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_widget_modifiers(&self, widget_id: &str) -> Result<Vec<WidgetModifier>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id as "id!: i64", widget_id, modifier_type 
            FROM modifiers 
            WHERE widget_id = ?
            "#,
            widget_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| WidgetModifier {
                id: Some(row.id as u32),
                widget_id: NanoId(row.widget_id),
                modifier_type: serde_json::from_str(&row.modifier_type).unwrap(),
            })
            .collect())
    }

    pub async fn add_widget_modifier(&self, widget_id: &str, modifier: &WidgetModifier) -> Result<(), Box<dyn Error>> {
        let modifier_type = serde_json::to_string(&modifier.modifier_type)?;

        sqlx::query!(
            r#"
            INSERT INTO modifiers (widget_id, modifier_type) 
            VALUES (?, ?)
            "#,
            widget_id,
            modifier_type
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_widget_modifier(&self, widget_id: &str, modifier_id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            DELETE FROM modifiers 
            WHERE widget_id = ? AND id = ?
            "#,
            widget_id,
            modifier_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl From<String> for NanoId {
    fn from(s: String) -> Self {
        NanoId(s)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ScrapedData {
    pub id: Option<i64>,
    pub widget_id: String,
    pub value: Option<String>,
    pub error: Option<String>,
    pub timestamp: String,
} 