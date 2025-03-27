use axum::{
    extract::{ConnectInfo, State},
    routing::{get, get_service},
    Json, Router,
};
use env_logger::fmt::Timestamp;
use http::HeaderValue;
use jiff;
use log::{debug, error, info, warn};
use muda::{
    accelerator::{Accelerator, Modifiers},
    Menu, MenuItem, PredefinedMenuItem, Submenu,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    cmp::max,
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tokio::{runtime::Runtime, time::sleep};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::ServeFile,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder, EventLoopProxy},
    keyboard::{KeyCode, ModifiersKeyState, PhysicalKey},
    platform::macos::{
        ActiveEventLoopExtMacOS, EventLoopBuilderExtMacOS, WindowAttributesExtMacOS, WindowExtMacOS,
    },
    window::{Window, WindowId, WindowLevel},
};

use wry::{
    dpi::{LogicalPosition, LogicalSize, Position},
    http::Response,
    PageLoadEvent, Rect, WebView, WebViewBuilder, WebViewBuilderExtDarwin,
};

use rusqlite::{
    self,
    types::{FromSql, ToSqlOutput},
    ToSql,
};

pub const RESIZE_DEBOUNCE_TIME: u128 = 50;

pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WidgetType {
    File(FileConfiguration),
    Source(SourceConfiguration),
    Url(UrlConfiguration),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct UrlConfiguration {
    url: String,
    refresh_interval: Seconds,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct FileConfiguration {
    html: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
struct SourceConfiguration {
    url: String,
    element_selectors: Vec<String>,
    refresh_interval: Seconds,
}

impl Default for SourceConfiguration {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            element_selectors: vec![],
            refresh_interval: 0,
        }
    }
}

impl FromSql for SourceConfiguration {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str().unwrap();
        Ok(SourceConfiguration {
            url: value.to_string(),
            element_selectors: vec![],
            refresh_interval: value.to_string().parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct WidgetConfiguration {
    id: NanoId,
    title: String,
    widget_type: WidgetType,
    level: Level,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct CreateWidgetRequest {
    url: String,
    level: Level,
    refresh_interval: Seconds,
}

trait SqliteDetails {
    // fn to_sql(&self) -> rusqlite::Result<ToSqlOutput>;
    // fn from_sql(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self>;
}

impl WidgetConfiguration {
    fn new() -> Self {
        Self {
            id: NanoId(nanoid_gen(8)),
            title: "".to_string(),
            widget_type: WidgetType::Source(SourceConfiguration::default()),
            level: Level::Normal,
        }
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_widget_type(mut self, widget_type: WidgetType) -> Self {
        self.widget_type = widget_type;
        self
    }

    pub fn with_id(mut self, id: NanoId) -> Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Level {
    AlwaysOnTop,
    Normal,
    AlwaysOnBottom,
}

impl ToSql for WidgetType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let json = serde_json::to_string(self).unwrap(); // Convert to JSON
        Ok(ToSqlOutput::from(json))
    }
}

impl FromSql for WidgetType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str().unwrap();
        Ok(serde_json::from_str(value).unwrap())
    }
}

impl ToSql for Level {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let value = serde_json::to_string(self).unwrap();
        Ok(ToSqlOutput::from(value))
    }
}

impl FromSql for Level {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str().unwrap();
        match value {
            "AlwaysOnTop" => Ok(Level::AlwaysOnTop),
            "Normal" => Ok(Level::Normal),
            "AlwaysOnBottom" => Ok(Level::AlwaysOnBottom),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

use nanoid::nanoid_gen;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ViewSize {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScrapedValue {
    pub id: NanoId,
    pub value: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct NanoId(String);

impl std::fmt::Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Record {
    id: i32,
    window_id: String,
    data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MonitoredSite {
    id: i32,
    site_id: NanoId,
    url: String,
    title: String,
    refresh_interval: Seconds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MonitoredElement {
    id: i32,
    site_id: i32,
    selector: String,
    data_key: String,
}

impl FromSql for NanoId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let id = value.as_str().unwrap();
        Ok(NanoId(id.to_string()))
    }
}

struct Database {
    // data: HashMap<String, Vec<Record>>, // table -> data????
    connection: rusqlite::Connection,
}

impl Database {
    fn new() -> Self {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute(
                "CREATE TABLE test (
                id INTEGER PRIMARY KEY,
                window_id TEXT NOT NULL,
                data TEXT NOT NULL
            )",
                (),
            )
            .unwrap();
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
        Self { connection }
    }

    fn get_elements(&self) -> Result<Vec<MonitoredElement>, rusqlite::Error> {
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

    fn get_configuration(&self) -> Result<Vec<WidgetConfiguration>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM widgets")?;
        let configuration = stmt
            .query_map([], |row| {
                // info!("querying row: {:?}", row);
                Ok(WidgetConfiguration {
                    id: row.get(0)?,
                    // url: row.get(1)?,
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

    fn insert_widget_configuration(
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

    fn get_sites(&self) -> Result<Vec<MonitoredSite>, rusqlite::Error> {
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

    fn get_data(&self) -> Result<Vec<Record>, rusqlite::Error> {
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

    fn get_latest_data(&self) -> Result<Vec<Record>, rusqlite::Error> {
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

    fn insert_data(&mut self, table: &str, insert_data: Record) -> Result<(), rusqlite::Error> {
        info!("Inserting data into table: {}, {:?}", table, insert_data);
        let mut stmt = self
            .connection
            .prepare("INSERT INTO test (window_id, data) VALUES (?1, ?2)")?;
        stmt.execute([insert_data.window_id, insert_data.data])?;
        Ok(())
    }
}

struct App {
    current_size: LogicalSize<u32>,
    menu: Menu,
    current_modifiers: Modifiers,
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    last_resize: Option<Instant>,
    db: Arc<Mutex<Database>>,
    // widgets: HashMap<NanoId, WidgetView>,
    all_windows: HashMap<WindowId, WidgetView>,
    widget_id_to_window_id: HashMap<NanoId, WindowId>,
    window_id_to_webview_id: HashMap<WindowId, NanoId>,
    // clipboard: arboard::Clipboard,
}

struct WidgetView {
    // webview: wry::WebView,
    app_webview: AppWebView,
    window: Window,
    nano_id: NanoId,
    visible: bool,
    options: WidgetOptions,
}

struct ElementView {
    webview: wry::WebView,
    nano_id: NanoId,
    visible: bool,
}

// These are messages that can be received and handled from the web page
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ControlMessage {
    CreateWidget(CreateWidgetRequest),
    // CreateWidget(WidgetConfiguration),
    Refresh(NanoId),
    Remove(NanoId),
    UpdateRefreshInterval(Seconds),
    Move(NanoId, Direction),
    ExtractResult(ScrapedValue),
    Minimize(NanoId),
    ToggleElementView(NanoId),
    // SelectedText { widget_id: NanoId, text: String },
    // CopyText { widget_id: NanoId, text: String },
    // PasteText { widget_id: NanoId },
    // Extract(String, String),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AddWebView {
    url: String,
    title: String,
    refresh_interval: Seconds,
}

type Seconds = i32;

impl App {
    // TODO: possibly split out the refresh timer and the extraction logic - maybe a page auto reloads already, and we just need to grab the newest value
    fn refresh_webview(&mut self, id: NanoId) {
        info!("Refreshing webview for {}", id);

        let window_id = self.widget_id_to_window_id[&id];
        if let Some(webview) = self.all_windows.get_mut(&window_id) {
            webview
                .app_webview
                .webview
                .reload()
                .expect("Something failed");
        }
    }

    fn remove_webview(&mut self, id: NanoId) {
        todo!("Remove not implemented");
    }

    fn ipc_handler(message: &str, event_proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>) {
        info!("[ipc_handler] Received message: {:?}", message);
        let message = serde_json::from_str::<ControlMessage>(message);

        if let Err(e) = message {
            error!("Error parsing message: {:?}", e);
            return;
        }
        let message = message.unwrap();

        let proxy = event_proxy.lock().expect("Something failed");
        match message {
            ControlMessage::CreateWidget(create_widget) => {
                info!("Create widget: {:?}", create_widget);
                proxy
                    .send_event(UserEvent::CreateWidget(create_widget))
                    .expect("Something failed");
            }

            ControlMessage::Refresh(id) => {
                proxy
                    .send_event(UserEvent::Refresh(id))
                    .expect("Something failed");
            }
            ControlMessage::Remove(id) => {
                proxy
                    .send_event(UserEvent::RemoveWebView(id))
                    .expect("Something failed");
            }
            ControlMessage::UpdateRefreshInterval(_) => todo!(),
            ControlMessage::Move(id, direction) => {
                proxy
                    .send_event(UserEvent::MoveWebView(id, direction))
                    .expect("Something failed");
            }
            ControlMessage::ExtractResult(result) => {
                info!("Extracted result: {:?}", result);
                proxy
                    .send_event(UserEvent::ExtractResult(result))
                    .expect("Something failed");
            }
            ControlMessage::Minimize(id) => {
                proxy
                    .send_event(UserEvent::Minimize(id))
                    .expect("Something failed");
            }
            ControlMessage::ToggleElementView(nano_id) => {
                info!("Toggling element view for {}", nano_id);
                proxy
                    .send_event(UserEvent::ToggleElementView(nano_id))
                    .expect("Something failed");
            }
        }
    }

    fn move_webview(&mut self, id: NanoId, direction: Direction) {
        todo!("Actualy implement this");
    }

    fn minimize_webview(&mut self, id: NanoId) {
        todo!("Minimize not implemented");
    }

    fn scrape_webview(&self, id: NanoId, source_config: SourceConfiguration) {
        info!("Scraping webview: {:?}", id);
        info!("Widget id to window id: {:?}", self.widget_id_to_window_id);
        info!("window to webview id: {:?}", self.window_id_to_webview_id);

        let Some(window_id) = self.widget_id_to_window_id.get(&id) else {
            info!("Webview not found");
            return;
        };

        let Some(widget_view) = self.all_windows.get(&window_id) else {
            info!("Webview not found");
            return;
        };

        info!("TEMP: attempting to extract a value now...");
        let script_content = String::from(
            r#"
try {
const element = document.querySelector("$selector");
if (!element) {
window.ipc.postMessage(
  JSON.stringify({
    extractresult: {
      error: "Element not found",
      value: null,
      id: "$id",
    }
  })
);
}

const scrape_value = element.getAttribute("aria-label") || element.textContent.trim();

window.ipc.postMessage(
JSON.stringify({
  extractresult: {
    error: null,
    value: scrape_value,
    id: "$id",
  }
})
);
} catch (e) {
window.ipc.postMessage(
JSON.stringify({
  extractresult: {
  error: JSON.stringify(e.message),
  value: null,
  id: "$id",
}
})
);
}

            "#,
        );

        let script_content = script_content
            .replace(
                "$selector",
                &source_config
                    .element_selectors
                    .get(0)
                    .as_ref()
                    .expect("Something failed"),
            )
            .replace("$id", &id.0);

        let result = widget_view
            .app_webview
            .webview
            .evaluate_script(&script_content);

        info!("Scrape completed");
    }

    fn add_scrape_result(&mut self, result: ScrapedValue) {
        let res = self.db.try_lock().expect("Something failed").insert_data(
            "scraped_values",
            Record {
                id: 0,
                window_id: result.id.0.clone(),
                data: result.value.clone(),
            },
        );
    }

    fn resize_window(&mut self, id: WindowId, size: &LogicalSize<u32>) {
        if let Some(window) = self.all_windows.get_mut(&id) {
            window.app_webview.webview.set_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: size.clone().into(),
            });
        }
    }

    fn create_widget(
        &mut self,
        event_loop: &ActiveEventLoop,
        widget_config: WidgetConfiguration,
        // size: LogicalSize<u32>,
    ) {
        // let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        // event_loop.set_allows_automatic_window_tabbing(enabled);
        let window_attributes = Window::default_attributes()
            .with_inner_size(self.current_size)
            .with_transparent(true)
            .with_blur(true)
            .with_movable_by_window_background(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_hidden(false)
            .with_title("Watcher")
            .with_has_shadow(false)
            .with_resizable(true);

        let new_window = event_loop
            .create_window(
                window_attributes
                    .clone()
                    .with_title(&widget_config.title.clone())
                    .with_window_level(match widget_config.level {
                        Level::AlwaysOnTop => WindowLevel::AlwaysOnTop,
                        Level::Normal => WindowLevel::Normal,
                        Level::AlwaysOnBottom => WindowLevel::AlwaysOnBottom,
                    }),
            )
            .expect("Something failed");
        let scale_factor = new_window.scale_factor();
        // let form_size = new_window.inner_size().to_logical::<u32>(scale_factor);
        // self.current_size = form_size;
        let proxy_clone = Arc::clone(&self.proxy);

        let webview = match &widget_config.widget_type {
            WidgetType::File(file_config) => {
                let webview = WebViewBuilder::new()
                    .with_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: self.current_size.into(),
                        // size: LogicalSize::new(600, 240).into(),
                    })
                    // .with_initialization_script(
                    //     include_str!("../assets/init_script.js")
                    //         .replace("$widget_id", &widget.id.0)
                    //         .as_str(),
                    // )
                    .with_initialization_script(
                        r#"
                        window.WINDOW_ID = "$window_id  ";
                        window.WIDGET_ID = "$widget_id";
                        "#
                        .replace("$window_id", &format!("{:?}", new_window.id()))
                        .replace("$widget_id", &widget_config.id.0)
                        .as_str(),
                    )
                    .with_html(file_config.html.as_str())
                    .with_ipc_handler(move |message| {
                        App::ipc_handler(message.body(), proxy_clone.clone());
                    })
                    .build_as_child(&new_window)
                    .expect("Something failed");
                Some(webview)
            }
            WidgetType::Source(source_config) => {
                let updated_url = if source_config.url.starts_with("http") {
                    source_config.url.clone()
                } else {
                    format!("https://{}", source_config.url)
                };
                info!("Creating source widget with url: {}", updated_url);
                let webview = WebViewBuilder::new()
                    .with_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: self.current_size.into(),
                    })
                    .with_initialization_script(
                        // include_str!("../assets/init_script.js")
                        //     .replace("$widget_id", &widget.id.0)
                        r#"
                        window.WINDOW_ID = "$window_id  ";
                        window.WIDGET_ID = "$widget_id";
                        "#
                        .replace("$window_id", &format!("{:?}", new_window.id()))
                        .replace("$widget_id", &widget_config.id.0)
                        .as_str(),
                    )
                    .with_url(updated_url)
                    // .with_html(file_config.html_file.as_str())
                    .with_ipc_handler(move |message| {
                        App::ipc_handler(message.body(), proxy_clone.clone());
                    })
                    .build_as_child(&new_window)
                    .expect("Something failed");
                Some(webview)
            }
            WidgetType::Url(url_config) => {
                let updated_url = if url_config.url.starts_with("http") {
                    url_config.url.clone()
                } else {
                    format!("https://{}", url_config.url)
                };
                info!("Creating url widget with url: {}", updated_url);
                let webview = WebViewBuilder::new()
                    .with_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: self.current_size.into(),
                    })
                    .with_traffic_light_inset(wry::dpi::Position::Logical(LogicalPosition::new(
                        0.0, 0.0,
                    )))
                    .with_url(updated_url)
                    .with_ipc_handler(move |message| {
                        App::ipc_handler(message.body(), proxy_clone.clone());
                    })
                    .build_as_child(&new_window)
                    .expect("Something failed");
                Some(webview)
            }
            _ => {
                info!("Unknown widget type, not creating webview");
                None
            }
        };

        if let Some(webview) = webview {
            self.widget_id_to_window_id
                .insert(widget_config.id.clone(), new_window.id());
            self.window_id_to_webview_id
                .insert(new_window.id(), widget_config.id.clone());
            self.all_windows.insert(
                new_window.id(),
                WidgetView {
                    app_webview: AppWebView { webview: webview },
                    window: new_window,
                    nano_id: widget_config.id.clone(),
                    visible: true,
                    options: WidgetOptions {
                        title: widget_config.title.clone(),
                        widget_type: widget_config.widget_type.clone(),
                    },
                },
            );
        }

        self.db
            .lock()
            .unwrap()
            .insert_widget_configuration(vec![widget_config]);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct WidgetOptions {
    title: String,
    widget_type: WidgetType,
}

struct AppWebView {
    webview: WebView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    data_key: String,
    window_id: String,
    message: String, // JSON string
    timestamp: String,
}

impl AppWebView {
    fn send_message(&self, message: &Message) {
        info!("Sending message to react: {:?}", message);
        self.webview
            .evaluate_script(format!("window.onRustMessage('{}');", json!(message)).as_str())
            .expect("Something failed");
    }
}

#[derive(Debug)]
enum UserEvent {
    MenuEvent(muda::MenuEvent),
    CreateWidget(CreateWidgetRequest),
    Refresh(NanoId),
    Scrape(NanoId, SourceConfiguration),
    RemoveWebView(NanoId),
    // ShowNewViewForm,
    MoveWebView(NanoId, Direction),
    ExtractResult(ScrapedValue),
    Minimize(NanoId),
    ToggleElementView(NanoId),
    // SelectedText { widget_id: NanoId, text: String },
    // CopyText { widget_id: NanoId, text: String },
    // PasteText { widget_id: NanoId },
    // Extract(String, String),
    // ExtractResult(String),
}

impl ApplicationHandler<UserEvent> for App {
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application suspended");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        // let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let mut widgets = vec![];
        {
            let mut db = self.db.lock().expect("Something failed");
            widgets.extend_from_slice(&db.get_configuration().expect("Something failed"));
        }

        info!("Found {} widgets", widgets.len());
        for widget_config in widgets {
            self.create_widget(event_loop, widget_config);
        }
        info!(
            "Widgets: {:?}",
            &self
                .all_windows
                .iter()
                .map(|(id, widget)| (id, widget.options.title.clone()))
                .collect::<Vec<(&WindowId, String)>>()
        );

        info!("Window and webviews created successfully");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // info!("Window event received: {:?}, {:?}", event, window_id);
        match event {
            WindowEvent::CloseRequested => {
                info!("Closing window: {:?}", window_id);
                let webview_id = self.window_id_to_webview_id.remove(&window_id).unwrap();
                self.widget_id_to_window_id.remove(&webview_id);
                self.all_windows.remove(&window_id);
                // self.db
                //     .lock()
                //     .unwrap()
                //     .remove_widget_configuration(webview_id);
            }
            WindowEvent::RedrawRequested => {}
            WindowEvent::Resized(size) => {
                let now = Instant::now();
                if let Some(last_resize) = self.last_resize {
                    if now.duration_since(last_resize).as_millis() < RESIZE_DEBOUNCE_TIME {
                        // Skip this resize event if less than 50ms since last one
                        return;
                    }
                }
                let window = self
                    .all_windows
                    .get_mut(&window_id)
                    .expect("Something failed");
                window.app_webview.webview.set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: size.clone().into(),
                });
                // Debounce resize events to improve performance
                self.last_resize = Some(now);
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                info!("Keyboard input event: {:?}", event);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                info!("Modifiers changed: {:?}", modifiers);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                // info!("Cursor moved: {:?}", position);
            }
            _ => {
                info!("Unhandledevent: {:?}", event);
            }
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        let size = self.current_size.clone();
        match event {
            UserEvent::Refresh(id) => {
                info!("Refresh event received for index {}", id);
                self.refresh_webview(id);
            }
            UserEvent::RemoveWebView(id) => {
                info!("Removing webview at index {}", id);
                self.remove_webview(id);
            }
            UserEvent::MoveWebView(id, direction) => {
                info!("Moving webview at index {} {}", id, direction);
                self.move_webview(id, direction);
            }
            UserEvent::ExtractResult(result) => {
                info!("Extracted result: {:?}", result);
                self.add_scrape_result(result.clone());
                // self.update_element_view(result);
            }
            UserEvent::Minimize(id) => {
                info!("Minimizing webview at index {}", id);
                self.minimize_webview(id);
            }
            // UserEvent::ToggleElementView(nano_id) => {
            //     info!("UserEvent: Toggling element view for {}", nano_id);
            //     if let Some(mut element_view) = self.element_views.get_mut(&nano_id) {
            //         element_view.visible = !element_view.visible;
            //         if element_view.visible {
            //             element_view.webview.set_visible(true);
            //         } else {
            //             element_view.webview.set_visible(false);
            //         }
            //     }
            // }
            UserEvent::Scrape(id, source_config) => {
                info!("Scraping webview at index {}", id);
                self.scrape_webview(id, source_config);
            }
            UserEvent::CreateWidget(widget_options) => {
                info!("Creating new widget: {:?}", widget_options);
                let widget_config = WidgetConfiguration::new()
                    .with_widget_type(WidgetType::Url(UrlConfiguration {
                        url: widget_options.url,
                        refresh_interval: widget_options.refresh_interval,
                    }))
                    .with_level(widget_options.level);
                self.create_widget(event_loop, widget_config);
            }
            _ => {
                info!("Unknown event: {:?}", event);
                todo!("User event not handled: {:?}", event);
            }
        }
    }
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    #[cfg(target_os = "macos")]
    {
        info!("Initializing Macos App...");
        winit::platform::macos::EventLoopBuilderExtMacOS::with_activation_policy(
            &mut EventLoop::builder(),
            winit::platform::macos::ActivationPolicy::Regular,
        );

        winit::platform::macos::EventLoopBuilderExtMacOS::with_default_menu(
            &mut EventLoop::builder(),
            false,
        );
    }

    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .with_default_menu(false)
        .build()
        .expect("Something failed");
    let event_loop_proxy = event_loop.create_proxy();
    let menu = setup_menu();
    // menu.init_for_nsapp();

    let config: Vec<WidgetConfiguration> = vec![
        WidgetConfiguration::new()
            .with_id(NanoId("controls".to_string()))
            .with_title("Controls".to_string())
            .with_widget_type(WidgetType::File(FileConfiguration {
                html: include_str!("../../react-ui/dist/index.html").to_string(),
            }))
            .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_id(NanoId("Viewer".to_string()))
        //     .with_title("Viewer".to_string())
        //     .with_widget_type(WidgetType::File(FileConfiguration {
        //         html: include_str!("../assets/simple_viewer.html").to_string(),
        //     }))
        //     .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_id(NanoId("SPY".to_string()))
        //     .with_title("SPY".to_string())
        //     .with_widget_type(WidgetType::Source(SourceConfiguration {
        //         url: "https://finance.yahoo.com/quote/SPY/".to_string(),
        //         element_selectors: vec![r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()],
        //         refresh_interval: 240,
        //     }))
        //     .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_id(NanoId("NVDA".to_string()))
        //     .with_title("NVDA".to_string())
        //     .with_widget_type(WidgetType::Source(SourceConfiguration {
        //         url: "https://finance.yahoo.com/quote/NVDA/".to_string(),
        //         element_selectors: vec![r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()],
        //         refresh_interval: 240,
        //     }))
        //     .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_id(NanoId("test".to_string()))
        //     .with_title("test".to_string())
        //     .with_widget_type(WidgetType::Url(UrlConfiguration {
        //         url: "http://localhost:3000/test".to_string(),
        //         refresh_interval: 1000,
        //     }))
        //     .with_level(Level::Normal),
        WidgetConfiguration::new()
            .with_id(NanoId("from vite server".to_string()))
            .with_title("from vite server".to_string())
            .with_widget_type(WidgetType::Url(UrlConfiguration {
                url: "http://localhost:5173".to_string(),
                refresh_interval: 1000,
            }))
            .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_title("New Widget".to_string())
        //     .with_widget_type(WidgetType::File(FileConfiguration {
        //         html: include_str!("../assets/NewWidgetForm.html").to_string(),
        //     }))
        //     .with_level(Level::AlwaysOnTop),
    ];

    info!("Debug Config: {:?}", config.len());

    let proxy_clone = event_loop_proxy.clone();
    let proxy_clone_muda = event_loop_proxy.clone();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        info!("Menu event: {:?}", event);
        proxy_clone_muda.send_event(UserEvent::MenuEvent(event));
    }));
    let db = Arc::new(Mutex::new(Database::new()));
    {
        let mut db = db.lock().unwrap();
        match db.insert_widget_configuration(config) {
            Ok(_) => info!("Inserted widget configurations"),
            Err(e) => error!("Error inserting widget configurations: {:?}", e),
        }
    }

    let mut app = App {
        current_size: LogicalSize::new(320, 240),
        current_modifiers: Modifiers::default(),
        all_windows: HashMap::new(),
        widget_id_to_window_id: HashMap::new(),
        window_id_to_webview_id: HashMap::new(),
        db: db.clone(),
        proxy: Arc::new(Mutex::new(event_loop_proxy)),
        last_resize: None,
        // clipboard: arboard::Clipboard::new().unwrap(),
        menu,
    };

    // scraping thread
    // let db_clone = db.clone();
    // std::thread::spawn(move || {
    //     info!("Starting scraping thread");
    //     let mut min_refresh_interval = std::time::Duration::from_secs(4);
    //     // let mut last_refresh = HashMap::new();
    //     loop {
    //         std::thread::sleep(min_refresh_interval);
    //         {
    //             let mut widget_configs = vec![];
    //             {
    //                 let mut db = db_clone.lock().expect("Something failed");
    //                 let views = db.get_configuration().expect("Something failed");
    //                 widget_configs.extend(views);
    //             }
    //             info!(
    //                 "Scraping widget configs: {:?}",
    //                 widget_configs
    //                     .iter()
    //                     .map(|c| c.id.0.clone())
    //                     .collect::<Vec<_>>()
    //             );
    //             for config in widget_configs.iter_mut() {
    //                 // let mut last_refresh = last_refresh.entry(config.id.clone()).or_insert(now);
    //                 match &config.widget_type {
    //                     WidgetType::Source(source_config) => {
    //                         proxy_clone
    //                             .send_event(UserEvent::Scrape(
    //                                 config.id.clone(),
    //                                 source_config.clone(),
    //                             ))
    //                             .expect("Something failed");
    //                     }
    //                     _ => {
    //                         info!("Widget type not handled...");
    //                     }
    //                 };
    //             }
    //         }
    //     }
    // });

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        rt.block_on(async {
            let state = ApiState { db: db.clone() };

            let cors_layer = CorsLayer::new()
                .allow_methods(vec![http::Method::GET, http::Method::POST])
                .allow_headers(vec![http::HeaderName::from_static("content-type")])
                // .allow_origin(Origin::list(vec![
                //     // "http://localhost:5173".parse().unwrap(),
                //     // "http://127.0.0.1:5173".parse().unwrap(),
                // ]))
                .allow_origin(AllowOrigin::any());
            let router = Router::new()
                .route("/values", get(get_values))
                .route("/sites", get(get_sites))
                .route("/elements", get(get_elements))
                .route("/latest", get(get_latest_values))
                .route(
                    "/test",
                    get_service(ServeFile::new("../react-ui/dist/index.html")),
                )
                // .route("/values", post(update_value))
                .layer(cors_layer)
                .with_state(state);

            let addr = format!("{}:{}", "127.0.0.1", 3000);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            println!("API listening on http://{}", addr);
            axum::serve(listener, router).await.unwrap();
        });
    });

    event_loop.run_app(&mut app).expect("Something failed");
}

fn setup_menu() -> Menu {
    let menu = Menu::new();

    // Add application menu (required for macOS)
    #[cfg(target_os = "macos")]
    {
        let app_menu = Submenu::new("App", true);
        let _ = app_menu.append_items(&[
            &PredefinedMenuItem::about(Some("My App"), None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ]);
        let _ = menu.append(&app_menu);

        // Add Edit menu with standard items
        let edit_menu = Submenu::new("Edit", true);
        let _ = edit_menu.append_items(&[
            &PredefinedMenuItem::cut(None),
            &PredefinedMenuItem::copy(None),
            &PredefinedMenuItem::paste(None),
            &PredefinedMenuItem::select_all(None),
        ]);
        let _ = menu.append(&edit_menu);
    }

    // let _ = menu.append(&custom_menu);

    // Initialize the menu for the appropriate platform
    #[cfg(target_os = "macos")]
    menu.init_for_nsapp();

    #[cfg(target_os = "windows")]
    unsafe {
        menu.init_for_hwnd(window.hwnd() as isize)
    };

    #[cfg(target_os = "linux")]
    menu.init_for_gtk_window(&gtk_window, Some(&vertical_gtk_box));

    menu
}

// async fn get_widget(
//     State(state): State<ApiState>,
//     Path(id): Path<String>,
// ) -> Json<WidgetConfiguration> {
//     // let state = state.db.lock().unwrap();
//     // let widget: WidgetConfiguration = state.get_widget(id).unwrap();
//     // Json(widget)
//     ServeFile::new(path)
// }

async fn get_values(State(state): State<ApiState>) -> Json<Vec<Record>> {
    let state = state.db.lock().unwrap();
    let values: Vec<Record> = state.get_data().unwrap();
    Json(values)
}

async fn get_latest_values(State(state): State<ApiState>) -> Json<Vec<Record>> {
    let state = state.db.lock().unwrap();
    let values: Vec<Record> = state.get_latest_data().unwrap();
    Json(values)
}

async fn get_sites(State(state): State<ApiState>) -> Json<Vec<MonitoredSite>> {
    let state = state.db.lock().unwrap();
    let sites: Vec<MonitoredSite> = state.get_sites().unwrap();
    Json(sites)
}

async fn get_elements(State(state): State<ApiState>) -> Json<Vec<MonitoredElement>> {
    let state = state.db.lock().unwrap();
    let elements: Vec<MonitoredElement> = state.get_elements().unwrap();
    Json(elements)
}

#[derive(Clone)]
struct ApiState {
    db: Arc<Mutex<Database>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_message() {
        let message = ControlMessage::Refresh(NanoId("0".to_string()));
        let json = serde_json::to_string(&message).expect("Something failed");
        assert_eq!(json, r#"{"refresh":0}"#);

        // Test move command
        let message = ControlMessage::Move(NanoId("1".to_string()), Direction::Up);
        let json = serde_json::to_string(&message).expect("Something failed");
        assert_eq!(json, r#"{"move":[1,"up"]}"#);

        // Test deserialization
        let message: ControlMessage =
            serde_json::from_str(r#"{"move":[1,"up"]}"#).expect("Something failed");
        assert_eq!(
            message,
            ControlMessage::Move(NanoId("1".to_string()), Direction::Up)
        );
    }
}
