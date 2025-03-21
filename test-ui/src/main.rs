use axum::{extract::State, routing::get, Json, Router};
use element_extractor::Extractor;
use env_logger::fmt::Timestamp;
use http::HeaderValue;
use jiff;
use log::{debug, error, info, warn};
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
use tower_http::cors::CorsLayer;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder, EventLoopProxy},
    platform::macos::WindowAttributesExtMacOS,
    window::{Window, WindowId, WindowLevel},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    http::Response,
    Rect, WebView, WebViewBuilder,
};

use rusqlite::{self, types::FromSql};

pub const WEBVIEW_HEIGHT: u32 = 200;
pub const WEBVIEW_WIDTH: u32 = 50;
pub const CONTROL_PANEL_HEIGHT: u32 = 40;
pub const CONTROL_PANEL_WIDTH: u32 = 50;
pub const WINDOW_WIDTH: u32 = 240;
pub const WINDOW_HEIGHT: u32 = 100;
pub const RESIZE_DEBOUNCE_TIME: u128 = 300;

pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WidgetType {
    Display,
    Source(SourceConfiguration),
    Tracker,
    Controls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
impl FromSql for WidgetType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str().unwrap();
        match value {
            "Display" => Ok(WidgetType::Display),
            "Source" => {
                let source_configuration = SourceConfiguration::default();
                Ok(WidgetType::Source(source_configuration))
            }
            "Tracker" => Ok(WidgetType::Tracker),
            "Controls" => Ok(WidgetType::Controls),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoredView {
    id: NanoId,
    url: String,
    title: String,
    // index: usize,
    refresh_count: usize,
    last_refresh: jiff::Timestamp,
    refresh_interval: std::time::Duration,
    last_scrape: jiff::Timestamp,
    scrape_interval: std::time::Duration,
    element_selector: Option<String>,
    scraped_history: Vec<ScrapedValue>,
    // original_size: ViewSize,
    hidden: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct WidgetConfiguration {
    id: NanoId,
    // url: String,
    title: String,
    refresh_interval: Seconds,
    widget_type: WidgetType,
}

use nanoid::nanoid_gen;

impl MonitoredView {
    pub fn from(
        title: String,
        url: String,
        refresh_interval: std::time::Duration,
        scrape_interval: std::time::Duration,
        element_selector: Option<String>,
    ) -> Self {
        Self {
            id: NanoId(nanoid_gen(8)),
            url,
            title,
            refresh_count: 0,
            last_refresh: jiff::Timestamp::now(),
            refresh_interval,
            last_scrape: jiff::Timestamp::now(),
            scrape_interval,
            element_selector,
            scraped_history: vec![],
            hidden: false,
        }
    }
}

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
                refresh_interval INTEGER NOT NULL,
                widget_type TEXT NOT NULL
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
                Ok(WidgetConfiguration {
                    id: row.get(0)?,
                    // url: row.get(1)?,
                    title: row.get(1)?,
                    refresh_interval: row.get(2)?,
                    widget_type: row.get(3)?,
                })
            })?
            .filter_map(|configuration| configuration.ok())
            .collect();
        Ok(configuration)
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
    // window: Option<Window>,
    // new_view_form_window: Option<Window>,
    // react_ui_window: Option<Window>,
    // react_webview: Option<AppWebView>,
    // monitored_views: Arc<Mutex<HashMap<NanoId, MonitoredView>>>,
    // webviews: HashMap<NanoId, wry::WebView>,
    // element_views: HashMap<NanoId, ElementView>,
    // controls: HashMap<NanoId, wry::WebView>,
    // new_view_form: Option<wry::WebView>,
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    last_resize: Option<Instant>,
    db: Arc<Mutex<Database>>,
    // widgets: HashMap<NanoId, WidgetView>,
    all_windows: HashMap<WindowId, WidgetView>,
    webview_to_window: HashMap<NanoId, WindowId>,
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ControlMessage {
    CreateWidget(bool),
    Refresh(NanoId),
    Add(AddWebView),
    Remove(NanoId),
    UpdateRefreshInterval(Seconds),
    Move(NanoId, Direction),
    ExtractResult(ScrapedValue),
    Minimize(NanoId),
    ToggleElementView(NanoId),
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
    // fn calculate_window_height(&self) -> u32 {
    //     let view_count = self.monitored_views.lock().expect("Something failed").len();
    //     if view_count == 0 {
    //         return WINDOW_WIDTH; // Default height for empty window
    //     }
    //     WINDOW_WIDTH // Let the window be resizable instead of calculating fixed height
    // }

    // TODO: possibly split out the refresh timer and the extraction logic - maybe a page auto reloads already, and we just need to grab the newest value
    fn refresh_webview(&mut self, id: NanoId) {
        info!("Refreshing webview for {}", id);
        // let view_details: MonitoredView;
        // {
        //     let mut views = self.monitored_views.lock().expect("Something failed");
        //     // info!("TEMP: index: {}, Views: {:#?}", id, views);
        //     view_details = views.get(&id).expect("Something failed").clone();
        // }

        let window_id = self.webview_to_window[&id];
        if let Some(webview) = self.all_windows.get_mut(&window_id) {
            webview
                .app_webview
                .webview
                .reload()
                .expect("Something failed");
        }
    }

    // fn add_webview(&mut self, view: AddWebView, event_loop: &ActiveEventLoop) {
    //     // let window_id = self.webview_to_window[&id];
    //     // if let Some(window) = self.all_windows.get_mut(&window_id) {

    //     let mut view = MonitoredView {
    //         id: NanoId(view.title.clone()),
    //         url: if view.url.starts_with("https://") {
    //             view.url
    //         } else {
    //             format!("https://{}", view.url)
    //         },
    //         title: view.title,
    //         // index: self.monitored_views.len(),
    //         refresh_count: 0,
    //         last_refresh: jiff::Timestamp::now(),
    //         refresh_interval: std::time::Duration::from_secs(view.refresh_interval as u64),
    //         last_scrape: jiff::Timestamp::now(),
    //         scrape_interval: std::time::Duration::from_secs(1),
    //         element_selector: None,
    //         hidden: false,
    //         scraped_history: vec![],
    //     };

    //     // Add to monitored views first so positions are calculated correctly
    //     let mut num_views = 0;
    //     {
    //         let mut views = self.monitored_views.lock().expect("Something failed");
    //         views.insert(view.id.clone(), view.clone());
    //         num_views = views.len();
    //     }

    //     let widget_options = WidgetOptions {
    //         title: view.title,
    //         url: view.url,
    //         width: WINDOW_WIDTH,
    //         height: WINDOW_HEIGHT,
    //     };

    //     let window_id = self.create_new_widget(event_loop, widget_options);

    //     // Create and add the webview and controls
    //     let webview = self.create_webview(&size, window, &mut view, num_views, num_views);

    //     let element_view = self.create_element_view(&size, window, &mut view, num_views, num_views);
    //     self.element_views.insert(
    //         view.id.clone(),
    //         ElementView {
    //             webview: element_view,
    //             nano_id: view.id.clone(),
    //             visible: false,
    //         },
    //     );
    //     let controls = {
    //         let webview_len = self.webviews.len();
    //         self.create_controls(
    //             &size,
    //             window,
    //             webview_len,
    //             self.proxy.clone(),
    //             NanoId(view.title.clone()),
    //         )
    //     };

    //     self.webviews.insert(view.id.clone(), webview);
    //     self.controls.insert(view.id.clone(), controls);

    //     // Fix positions of all webviews to ensure proper layout
    //     self.fix_webview_positions();
    //     // }
    // }

    fn create_webview(
        &self,
        size: &LogicalSize<u32>,
        window: &Window,
        view: &mut MonitoredView,
        // index: usize,
        // num_views: usize,
    ) -> WebView {
        let proxy = self.proxy.clone();

        let starting_height = window.inner_size().height;
        let starting_width = size.width;
        let width = if view.hidden { 0 } else { starting_width };
        let height = if view.hidden { 0 } else { starting_height };

        // view.original_size = ViewSize {
        //     width: starting_width,
        //     height: starting_height,
        // };

        let mut webviewbuilder = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, starting_height).into(),
                size: LogicalSize::new(width, height).into(),
            })
            .with_visible(true)
            .with_focused(false)
            .with_clipboard(true)
            .with_background_throttling(wry::BackgroundThrottlingPolicy::Throttle)
            .with_ipc_handler(move |message| {
                info!("Received message from webview: {:?}", message);
                match serde_json::from_str::<ScrapedValue>(&message.body()) {
                    Ok(scrape_result) => {
                        info!("Scrape result: {:?}", scrape_result);
                        proxy
                            .lock()
                            .expect("Something failed")
                            .send_event(UserEvent::ExtractResult(scrape_result))
                            .expect("Something failed");
                    }
                    Err(e) => {
                        warn!("Failed to parse message: {:?}", e);
                    }
                }
            });

        webviewbuilder = webviewbuilder.with_url(&view.url);
        let webview = webviewbuilder
            .build_as_child(window)
            .expect("Something failed");

        webview
    }

    fn remove_webview(&mut self, id: NanoId) {
        todo!("Remove not implemented");
        // self.webviews.remove(&id);
        // self.controls.remove(&id);

        // self.monitored_views
        //     .lock()
        //     .expect("Something failed")
        //     .remove(&id);

        // Update window height
        // if let Some(window) = self.window.as_ref() {
        //     let new_height = self.calculate_window_height();
        //     window.request_inner_size(LogicalSize::new(WINDOW_WIDTH, new_height));
        // }

        // self.fix_webview_positions();
    }

    // fn fix_webview_positions(&mut self) {
    //     let window = self.window.as_ref().expect("Something failed");
    //     let size = window.inner_size().to_logical::<u32>(window.scale_factor());
    //     self.resize_webviews(&size);
    // }

    fn create_controls(
        &self,
        size: &LogicalSize<u32>,
        window: &Window,
        i: usize,
        proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
        webview_id: NanoId,
    ) -> WebView {
        let script_contents = include_str!("../assets/controls.html").replace("$id", &webview_id.0);
        debug!("Controls script contents: {}", script_contents);

        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, WEBVIEW_HEIGHT * i as u32).into(),
                size: LogicalSize::new(size.width, CONTROL_PANEL_HEIGHT).into(),
            })
            .with_html(script_contents)
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().expect("Something failed");
                let message: ControlMessage =
                    serde_json::from_str(&message.body()).expect("Something failed");
                match message {
                    ControlMessage::CreateWidget(create_widget) => {
                        info!("Create widget: {}", create_widget);
                        if create_widget {
                            info!("Sending CreateNewWidget event");
                            proxy
                                .send_event(UserEvent::CreateNewWidget)
                                .expect("Something failed");
                        }
                    }
                    ControlMessage::Refresh(id) => {
                        proxy
                            .send_event(UserEvent::Refresh(id))
                            .expect("Something failed");
                    }
                    ControlMessage::Add(view) => {
                        proxy
                            .send_event(UserEvent::AddWebView(view))
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
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            .build_as_child(window)
            .expect("Something failed");

        control_panel
    }

    fn create_new_view_form(
        size: &LogicalSize<u32>,
        window: &Window,
        i: usize,
        proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    ) -> WebView {
        // put new view form at top of the window
        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, size.height).into(),
            })
            .with_html(include_str!("../assets/new_view.html"))
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().expect("Something failed");
                let message: ControlMessage =
                    serde_json::from_str(&message.body()).expect("Something failed");
                match message {
                    // ControlMessage::Refresh(index) => {
                    //     proxy.send_event(UserEvent::Refresh(index)).expect("Something failed");
                    // }
                    ControlMessage::Add(view) => {
                        proxy
                            .send_event(UserEvent::AddWebView(view))
                            .expect("Something failed");
                    }
                    ControlMessage::CreateWidget(create_widget) => {
                        if create_widget {
                            info!("Sending CreateNewWidget event");
                            proxy
                                .send_event(UserEvent::CreateNewWidget)
                                .expect("Something failed");
                        }
                    }
                    _ => {
                        error!("Unknown message from controls: {:?}", message);
                    }
                }
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            .build_as_child(window)
            .expect("Something failed");

        control_panel
    }

    // fn resize_webviews(&mut self, size: &LogicalSize<u32>) {
    //     let window = self.window.as_ref().expect("Something failed");
    //     let num_views = self.webviews.len();
    //     if num_views == 0 {
    //         return;
    //     }

    //     let webview_height = size.height / num_views as u32;

    //     // Pre-calculate common values
    //     let width = size.width;

    //     // Update all webviews in a single pass
    //     for (i, (id, webview)) in self.webviews.iter_mut().enumerate() {
    //         let y_position = webview_height * i as u32;

    //         // Only resize if the webview is visible
    //         if let Ok(bounds) = webview.bounds() {
    //             let current_size = bounds.size.to_logical::<u32>(window.scale_factor());
    //             if current_size.width > 0 {
    //                 // Only resize visible webviews
    //                 webview.set_bounds(Rect {
    //                     position: LogicalPosition::new(0, y_position).into(),
    //                     size: LogicalSize::new(width, webview_height).into(),
    //                 });
    //             }
    //         }

    //         // Update control position
    //         if let Some(control) = self.controls.get_mut(&id) {
    //             control.set_bounds(Rect {
    //                 position: LogicalPosition::new(0, y_position).into(),
    //                 size: LogicalSize::new(width, CONTROL_PANEL_HEIGHT).into(),
    //             });
    //         }
    //     }

    //     if let Some(new_view_form) = self.new_view_form.as_ref() {
    //         if let Some(new_form_window) = self.new_view_form_window.as_ref() {
    //             let new_form_size = new_form_window
    //                 .inner_size()
    //                 .to_logical::<u32>(window.scale_factor());
    //             new_view_form.set_bounds(Rect {
    //                 position: LogicalPosition::new(0, 0).into(),
    //                 size: LogicalSize::new(new_form_size.width, new_form_size.height).into(),
    //             });
    //         }
    //     }
    // }

    fn move_webview(&mut self, id: NanoId, direction: Direction) {
        // let new_index = match direction {
        //     Direction::Up => {
        //         if index + 1 >= self.webviews.len() {
        //             return; // Can't move up if already at top
        //         }
        //         index + 1
        //     }
        //     Direction::Down => {
        //         if index == 0 {
        //             return; // Can't move down if already at bottom
        //         }
        //         index - 1
        //     }
        // };

        todo!("Actualy implement this");
        // // Swap webviews
        // self.webviews.swap(index, new_index);

        // // Swap controls
        // self.controls.swap(index, new_index);

        // // Swap monitored views and update indices
        // {
        //     let mut views = self.monitored_views.lock().expect("Something failed");
        //     views.swap(index, new_index);
        //     views[index].index = index;
        //     views[new_index].index = new_index;
        // } // Lock is dropped here

        // Update positions of all webviews and controls
        // self.fix_webview_positions();
    }

    fn minimize_webview(&mut self, id: NanoId) {
        todo!("Minimize not implemented");
        // info!("Minimizing webview at index {}", id);
        // let window = self.window.as_ref().expect("Something failed");
        // if let Some(webview) = self.webviews.get_mut(&id) {
        // if let Ok(bounds) = webview.bounds() {
        //     // let original_size = self.monitored_views.lock().expect("Something failed")[&id]
        //     //     .original_size
        //     //     .clone();
        //     let current_size = bounds.size.to_logical::<u32>(window.scale_factor());
        //     info!("Current size: {:?}", current_size);
        //     if current_size.width > 0 {
        //         webview.set_bounds(Rect {
        //             position: LogicalPosition::new(0, 0).into(),
        //             size: LogicalSize::new(0, 0).into(),
        //         });
        //     } else {
        //         webview.set_bounds(Rect {
        //             position: LogicalPosition::new(0, 0).into(),
        //             size: LogicalSize::new(window.inner_size().width, window.inner_size().height)
        //                 .into(),
        //         });
        //     }
        // }
        // {
        //     let mut views = self.monitored_views.lock().expect("Something failed");
        //     views.entry(id).and_modify(|view| view.hidden = true);
        // }
    }

    fn create_element_view(
        &self,
        size: &LogicalSize<u32>,
        window: &Window,
        view: &mut MonitoredView,
        index: usize,
        num_views: usize,
    ) -> WebView {
        let starting_height = window.inner_size().height / num_views as u32;

        let width = if view.hidden { 0 } else { 200 };
        let height = if view.hidden { 0 } else { 100 };

        let element_view = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, starting_height * index as u32 + 50).into(),
                size: LogicalSize::new(width, height).into(),
            })
            .with_html(include_str!("../assets/element_view.html"))
            // .with_url("https://www.google.com")
            .with_transparent(false)
            // .with_background_color((0, 200, 90, 100))
            .with_visible(true) // Make sure it's visible
            .build_as_child(window)
            .expect("Something failed");
        element_view
    }

    fn scrape_webview(&self, id: NanoId, source_config: SourceConfiguration) {
        info!("Scraping webview: {:?}", id);
        let window_id = self.webview_to_window[&id];
        if self.all_windows.get(&window_id).is_none() {
            info!("Webview not found");
            return;
        }

        let widget_view = self.all_windows.get(&window_id).expect("Something failed");
        // TODO: check if widget type is something you can scrape against?

        // let view_details: MonitoredView;
        // {
        //     let views = self.monitored_views.lock().expect("Something failed");
        //     view_details = views.get(&id).expect("Something failed").clone();
        // }
        let window_id = self.webview_to_window[&id];
        let window = self.all_windows.get(&window_id).expect("Something failed");
        // match window.options.widget_type {
        //     WidgetType::Display => {
        //         info!("Display widget, not scraping");
        //     }
        //     _ => {
        //         info!("Scraping widget");
        //     }
        // }
        // if view_details.element_selector.is_none() {
        //     info!("No element to scrape for {}", id);
        //     return;
        // }

        // let webview = widget_view.app_webview.webview;

        info!("TEMP: attempting to extract a value now...");
        let script_content = String::from(
            r#"
try {
const element = document.querySelector("$selector");
if (!element) {
window.ipc.postMessage(
  JSON.stringify({
    error: "Element not found",
    value: null,
    id: "$id",
  })
);
}

window.ipc.postMessage(
JSON.stringify({
  error: null,
  value: element.textContent,
  id: "$id",
})
);
} catch (e) {
window.ipc.postMessage(
JSON.stringify({
  error: JSON.stringify(e.message),
  value: null,
  id: "$id",
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
        // let result = webview.evaluate_script(&script_content);

        // let script_content = include_str!("../assets/find_element.js")
        //     .replace(
        //         "$selector",
        //         &view_details.element_selector.as_ref().expect("Something failed"),
        //     )
        //     .replace("$id", &view_details.id.0);
        let result = widget_view
            .app_webview
            .webview
            .evaluate_script(&script_content);

        // if let Some(react_webview) = self.react_webview.as_ref() {
        //     info!("Sending message to react...");
        //     react_webview.send_message("Hello from Rust");
        // }

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

    fn create_widget(&mut self, event_loop: &ActiveEventLoop, widget_options: WidgetOptions) {
        let cloned_widget_options = widget_options.clone();
        let new_window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_active(true)
                    .with_decorations(true)
                    .with_title(widget_options.title.clone())
                    .with_has_shadow(false)
                    .with_title_hidden(false)
                    .with_titlebar_hidden(false)
                    .with_resizable(true),
            )
            .unwrap();
        // let new_webview = WebViewBuilder::new()
        //     // .with_bounds(Rect {
        //     //     position: LogicalPosition::new(0, 0).into(),
        //     //     size: size.clone().into(),
        //     // })
        //     .with_url("https://www.google.com")
        //     .build_as_child(&new_window)
        //     .unwrap();

        let mut view = MonitoredView {
            id: NanoId(nanoid_gen(8)),
            url: widget_options.url.clone(),
            title: widget_options.title,
            refresh_count: 0,
            last_refresh: jiff::Timestamp::now(),
            refresh_interval: std::time::Duration::from_secs(10),
            last_scrape: jiff::Timestamp::now(),
            scrape_interval: std::time::Duration::from_secs(10),
            element_selector: None,
            scraped_history: vec![],
            hidden: false,
        };
        let size = new_window
            .inner_size()
            .to_logical::<u32>(new_window.scale_factor());

        let webview = self.create_webview(&size, &new_window, &mut view);

        let widget_id = NanoId(nanoid_gen(8));
        let window_id = new_window.id();
        let widget_view = WidgetView {
            app_webview: AppWebView { webview },
            window: new_window,
            nano_id: widget_id.clone(),
            visible: true,
            options: cloned_widget_options,
        };
        self.all_windows.insert(window_id, widget_view);
    }
}

// fn widget_attributes() -> WindowAttributes {
//     Window::default_attributes()
//         .with_inner_size(LogicalSize::new(WINDOW_WIDTH, window_height))
//         .with_title("new view")
// }
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct WidgetOptions {
    title: String,
    url: String,
    width: u32,
    height: u32,
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
    CreateNewWidget,
    Refresh(NanoId),
    Scrape(NanoId, SourceConfiguration),
    AddWebView(AddWebView),
    RemoveWebView(NanoId),
    // ShowNewViewForm,
    MoveWebView(NanoId, Direction),
    ExtractResult(ScrapedValue),
    Minimize(NanoId),
    ToggleElementView(NanoId),
    // Extract(String, String),
    // ExtractResult(String),
}

impl ApplicationHandler<UserEvent> for App {
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application suspended");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");

        let mut db = self.db.lock().expect("Something failed");
        let sites = db.get_sites().expect("Something failed");

        let configuration = db.get_configuration().expect("Something failed");

        // todo!("Build the windows based on the configuration in the database");
        // let window_height = self.calculate_window_height();
        let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let window_attributes = Window::default_attributes()
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(size)
            .with_transparent(true)
            .with_blur(true)
            .with_movable_by_window_background(true)
            .with_fullsize_content_view(false)
            .with_title_hidden(false)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_hidden(false)
            .with_title("Watcher")
            .with_has_shadow(false)
            .with_resizable(true);

        // controls window
        let new_view_form_window = event_loop
            .create_window(
                window_attributes
                    .clone()
                    .with_inner_size(size)
                    .with_title("new view"),
            )
            .expect("Something failed");

        let scale_factor = new_view_form_window.scale_factor();
        // let scale_factor = 0.5;
        let form_size = new_view_form_window
            .inner_size()
            .to_logical::<u32>(scale_factor);
        let new_view_form =
            App::create_new_view_form(&form_size, &new_view_form_window, 0, self.proxy.clone());
        // self.new_view_form = Some(new_view_form);

        let new_view_form_id = NanoId(nanoid_gen(8));
        self.webview_to_window
            .insert(new_view_form_id.clone(), new_view_form_window.id());
        self.all_windows.insert(
            new_view_form_window.id(),
            WidgetView {
                app_webview: AppWebView {
                    webview: new_view_form,
                },
                window: new_view_form_window,
                nano_id: new_view_form_id,
                visible: true,
                options: WidgetOptions::default(),
            },
        );

        let reactui_window = event_loop
            .create_window(window_attributes.with_title("reactui"))
            .expect("Something failed");
        let react_webview = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: size.into(),
            })
            .with_url("http://localhost:5173")
            // .with_html("<div>Hello from Rust</div>")
            .with_ipc_handler(|message| {
                info!("Received message from react: {:?}", message);
            })
            .build_as_child(&reactui_window)
            .expect("Something failed");
        // self.react_webview = Some(AppWebView {
        //     webview: react_webview,
        // });
        // self.react_ui_window = Some(reactui_window);
        // self.react_webview = Some(react_webview);

        let reactui_window_id = NanoId(nanoid_gen(8));
        self.webview_to_window
            .insert(reactui_window_id.clone(), reactui_window.id());
        self.all_windows.insert(
            reactui_window.id(),
            WidgetView {
                app_webview: AppWebView {
                    webview: react_webview,
                },
                window: reactui_window,
                nano_id: reactui_window_id,
                visible: true,
                options: WidgetOptions::default(),
            },
        );

        info!("Window and webviews created successfully");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        info!("Window event   received: {:?}", event);
        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // info!("Redraw requested");
                // window.request_redraw();
                // self.window.as_ref().expect("Something failed").request_redraw();
            }
            WindowEvent::Resized(size) => {
                let now = Instant::now();
                if let Some(last_resize) = self.last_resize {
                    if now.duration_since(last_resize).as_millis() < RESIZE_DEBOUNCE_TIME {
                        // Skip this resize event if less than 50ms since last one
                        return;
                    }
                }
                self.all_windows.iter_mut().for_each(|(_, widget)| {
                    widget.app_webview.webview.set_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: size.clone().into(),
                    });
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
            _ => {}
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Refresh(id) => {
                info!("Refresh event received for index {}", id);
                self.refresh_webview(id);
            }
            UserEvent::AddWebView(view) => {
                info!("Adding new webview: {:?}", view);
                // self.add_webview(view);
                self.create_widget(
                    event_loop,
                    WidgetOptions {
                        title: view.title,
                        url: view.url,
                        width: WINDOW_WIDTH,
                        height: WINDOW_WIDTH,
                    },
                );
            }
            UserEvent::RemoveWebView(id) => {
                info!("Removing webview at index {}", id);
                self.remove_webview(id);
            }
            // UserEvent::ShowNewViewForm => {
            //     info!("Showing new view form");
            //     self.new_view_form
            //         .as_ref()
            //         .expect("New view form not found")
            //         .set_visible(true);
            // }
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
            UserEvent::CreateNewWidget => {
                info!("Creating new widget");
                self.create_widget(event_loop, WidgetOptions::default());
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

    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .expect("Something failed");
    let event_loop_proxy = event_loop.create_proxy();
    // let monitored_views = Arc::new(Mutex::new(vec![
    //     MonitoredView {
    //         url: "https://finance.yahoo.com/quote/SPY/".to_string(),
    //         title: "SPY".to_string(),
    //         // index: 2,
    //         refresh_count: 0,
    //         last_refresh: jiff::Timestamp::now(),
    //         refresh_interval: std::time::Duration::from_secs(240),
    //         element_selector: Some(r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()),
    //         scraped_history: vec![],
    //         // original_size: ViewSize {
    //         //     width: 0,
    //         //     height: 0,
    //         // },
    //         hidden: true,
    //         last_scrape: jiff::Timestamp::now(),
    //         scrape_interval: std::time::Duration::from_secs(1),
    //         id: NanoId("SPY".to_string()),
    //     },
    //     MonitoredView {
    //         url: "https://finance.yahoo.com/quote/NVDA/".to_string(),
    //         title: "NVDA".to_string(),
    //         // index: 1,
    //         refresh_count: 0,
    //         last_refresh: jiff::Timestamp::now(),
    //         refresh_interval: std::time::Duration::from_secs(240),
    //         element_selector: Some(r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()),
    //         scraped_history: vec![],
    //         // original_size: ViewSize {
    //         //     width: 0,
    //         //     height: 0,4
    //         // },
    //         hidden: true,
    //         last_scrape: jiff::Timestamp::now(),
    //         scrape_interval: std::time::Duration::from_secs(1),
    //         id: NanoId("NVDA".to_string()),
    //     },
    //     MonitoredView::from(
    //         "Twitch Viewers".to_string(),
    //         "https://www.twitch.tv/atrioc".to_string(),
    //         std::time::Duration::from_secs(240),
    //         std::time::Duration::from_secs(10),
    //         Some(String::from(r#"#live-channel-stream-information > div > div > div.Layout-sc-1xcs6mc-0.kYbRHX > div.Layout-sc-1xcs6mc-0.evfzyg > div.Layout-sc-1xcs6mc-0.iStNQt > div.Layout-sc-1xcs6mc-0.hJHxso > div > div > div.Layout-sc-1xcs6mc-0.bKPhAm > div:nth-child(1) > div > p.CoreText-sc-1txzju1-0.fiDbWi > span"#)),
    //     )
    //     ]
    //     .iter()
    //     .map(|view| (view.id.clone(), view.clone()))
    //     .collect::<HashMap<NanoId, MonitoredView>>()
    // ));

    let config = vec![WidgetConfiguration {
        id: NanoId(nanoid_gen(8)),
        title: "SPY".to_string(),
        refresh_interval: 240,
        widget_type: WidgetType::Source(SourceConfiguration {
            url: "https://finance.yahoo.com/quote/SPY/".to_string(),
            element_selectors: vec![r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()],
            refresh_interval: 240,
        }),

    },
    WidgetConfiguration {
        id: NanoId(nanoid_gen(8)),
        title: "NVDA".to_string(),
        refresh_interval: 240,
        widget_type: WidgetType::Source(SourceConfiguration {
            url: "https://finance.yahoo.com/quote/NVDA/".to_string(),
            element_selectors: vec![r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()],
            refresh_interval: 240,
        }),
    }];
    let proxy_clone = event_loop_proxy.clone();
    let db = Arc::new(Mutex::new(Database::new()));
    let mut app = App {
        all_windows: HashMap::new(),
        webview_to_window: HashMap::new(),
        db: db.clone(),
        // window: None,
        // new_view_form_window: None,
        // react_ui_window: None,
        // react_webview: None,
        // monitored_views: monitored_views,
        // webviews: HashMap::new(),
        // controls: HashMap::new(),
        // new_view_form: None,
        // element_views: HashMap::new(),
        proxy: Arc::new(Mutex::new(event_loop_proxy)),
        last_resize: None,
        // widgets: HashMap::new(),
    };

    let db_clone = db.clone();
    std::thread::spawn(move || {
        let mut min_refresh_interval = std::time::Duration::from_secs(4);
        // let mut last_refresh = HashMap::new();
        loop {
            std::thread::sleep(min_refresh_interval);
            {
                let mut widget_configs = vec![];
                {
                    let mut db = db_clone.lock().expect("Something failed");
                    let views = db.get_configuration().expect("Something failed");
                    widget_configs.extend(views);
                }
                for config in widget_configs.iter_mut() {
                    // let mut last_refresh = last_refresh.entry(config.id.clone()).or_insert(now);
                    match &config.widget_type {
                        WidgetType::Source(source_config) => {
                            proxy_clone
                                .send_event(UserEvent::Scrape(
                                    config.id.clone(),
                                    source_config.clone(),
                                ))
                                .expect("Something failed");
                        }
                        _ => {
                            info!("Widget type not handled: {:?}", config.widget_type);
                        }
                    };
                    // proxy_clone
                    //     .send_event(UserEvent::Scrape(config.id.clone()))
                    //     .expect("Something failed");
                    // proxy_clone
                    //     .send_event(UserEvent::Refresh(config.id.clone()))
                    //     .expect("Something failed");
                }
            }
        }
    });

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        rt.block_on(async {
            let state = ApiState { db: db.clone() };

            let cors_layer = CorsLayer::new()
                .allow_methods(vec![http::Method::GET, http::Method::POST])
                .allow_headers(vec![http::HeaderName::from_static("content-type")])
                .allow_origin(http::HeaderValue::from_static("http://localhost:5173"));
            let router = Router::new()
                .route("/values", get(get_values))
                .route("/sites", get(get_sites))
                .route("/elements", get(get_elements))
                .route("/latest", get(get_latest_values))
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

    #[test]
    fn test_add_webview() {
        let add_post_data = "{\"add\":{\"url\":\"\",\"refresh_interval\":\"\",\"title\":\"\"}}";

        let message: ControlMessage =
            serde_json::from_str(add_post_data).expect("Something failed");

        assert_eq!(
            message,
            ControlMessage::Add(AddWebView {
                url: "".to_string(),
                refresh_interval: 60,
                title: "".to_string(),
            })
        );
    }
}
