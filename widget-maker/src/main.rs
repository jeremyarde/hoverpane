use axum::{
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, get_service, post},
    Json, Router,
};
use db::db::Database;
use env_logger::fmt::Timestamp;
use http::HeaderValue;
use jiff;
use log::{debug, error, info, warn};
use muda::{
    accelerator::{Accelerator, Modifiers},
    Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu,
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
use typeshare::typeshare;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder, EventLoopProxy},
    keyboard::{KeyCode, ModifiersKeyState, PhysicalKey},
    platform::macos::{
        ActiveEventLoopExtMacOS, EventLoopBuilderExtMacOS, WindowAttributesExtMacOS, WindowExtMacOS,
    },
    window::{Icon, Window, WindowId, WindowLevel},
};

use cocoa::base::nil;
use cocoa::base::{id, Nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString, NSUInteger};
use cocoa::{appkit::*, foundation::NSData};

use objc::declare::ClassDecl;
use objc::runtime::{object_getClass, Object, Sel, NO, YES};
use objc::*;

const DOCK_ICON: &[u8] = include_bytes!("/Users/jarde/Documents/misc/app-icon2.png");

use tray_icon::{TrayIconBuilder, TrayIconEvent};

use wry::{
    dpi::{LogicalPosition, LogicalSize, Position},
    PageLoadEvent, Rect, WebView, WebViewBuilder, WebViewBuilderExtDarwin,
};

mod db;

use rusqlite::{
    self,
    types::{FromSql, ToSqlOutput},
    ToSql,
};

use image;

pub const RESIZE_DEBOUNCE_TIME: u128 = 50;

pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
#[typeshare]
pub enum WidgetType {
    File(FileConfiguration),
    // Source(SourceConfiguration),
    Url(UrlConfiguration),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
#[typeshare]
pub enum Modifier {
    Scrape { selector: String },
    Refresh { interval_sec: i32 },
}

impl ToSql for Modifier {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let json = serde_json::to_string(self).unwrap();
        Ok(ToSqlOutput::from(json))
    }
}

impl FromSql for Modifier {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str().unwrap();
        Ok(serde_json::from_str(value).unwrap())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[typeshare]
pub struct WidgetModifier {
    id: i32,
    widget_id: NanoId,
    modifier_type: Modifier,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
struct UrlConfiguration {
    url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
struct FileConfiguration {
    html: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
struct WidgetConfiguration {
    id: NanoId,
    title: String,
    widget_type: WidgetType,
    level: Level,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
struct CreateWidgetRequest {
    url: String,
    title: String,
    level: Level,
    // refresh_interval: Seconds,
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
            widget_type: WidgetType::Url(UrlConfiguration {
                url: "".to_string(),
            }),
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
#[typeshare]
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
    // pub id: String,
    pub widget_id: NanoId,
    pub value: String,
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
#[typeshare]
pub struct NanoId(String);

impl std::fmt::Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

struct App {
    tray_menu_quit_id: String,
    current_size: LogicalSize<u32>,
    menu: Menu,
    current_modifiers: Modifiers,
    proxy: Arc<EventLoopProxy<UserEvent>>,
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
    last_scrape: Option<Instant>,
    last_refresh: Option<Instant>,
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
    fn refresh_webview(&mut self, id: NanoId, refresh_interval_secs: i32) {
        info!("Refreshing webview for: {}", id);

        let window_id = self.widget_id_to_window_id[&id];
        let Some(webview) = self.all_windows.get_mut(&window_id) else {
            error!("Webview not found");
            return;
        };

        if webview.last_refresh.is_some()
            && webview.last_refresh.unwrap().elapsed()
                > Duration::from_secs(refresh_interval_secs as u64)
        {
            webview.last_refresh = Some(Instant::now());
        } else {
            return;
        }

        match &webview.options.widget_type {
            WidgetType::Url(url_config) => {
                webview
                    .app_webview
                    .webview
                    .reload()
                    .expect("Something failed");
            }
            WidgetType::File(file_config) => {
                // TODO: may need to take the file and replace the window.WIDGET_ID with the actual widget id
                webview
                    .app_webview
                    .webview
                    .load_html(file_config.html.as_str())
                    .expect("Something failed");
            }
            _ => {
                error!("Cannot refresh non-url widget");
            }
        }
    }

    fn remove_webview(&mut self, id: NanoId) {
        todo!("Remove not implemented");
    }

    fn move_webview(&mut self, id: NanoId, direction: Direction) {
        todo!("Actualy implement this");
    }

    fn minimize_webview(&mut self, id: NanoId) {
        todo!("Minimize not implemented");
    }

    fn scrape_webview(&self, id: NanoId, element_selector: String) {
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
      widget_id: "$widget_id",
      timestamp: Date.now(),
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
    widget_id: "$widget_id",
    timestamp: Date.now(),
  }
})
);
} catch (e) {
window.ipc.postMessage(
JSON.stringify({
  extractresult: {
  error: JSON.stringify(e.message),
  value: null,
  widget_id: "$widget_id",
  timestamp: Date.now(),
}
})
);
}

            "#,
        );

        let script_content = script_content
            .replace("$selector", &element_selector)
            // .replace("$id", &id.0)
            .replace("$widget_id", &widget_view.nano_id.0);

        let result = widget_view
            .app_webview
            .webview
            .evaluate_script(&script_content);

        info!("Scrape completed");
    }

    fn add_scrape_result(&mut self, result: ScrapedValue) {
        let res = self
            .db
            .try_lock()
            .expect("Something failed")
            .insert_data(result);
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
        let new_window: Window = event_loop
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

        set_app_dock_icon(&new_window);

        let scale_factor = new_window.scale_factor();
        let proxy_clone = Arc::clone(&self.proxy);

        let webview = match &widget_config.widget_type {
            WidgetType::File(file_config) => {
                let html = file_config.html.clone();
                let html = html.replace("$widget_id", &widget_config.id.0);
                let html = html.replace("$window_id", &format!("{:?}", new_window.id()));

                let webview = WebViewBuilder::new()
                    .with_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: self.current_size.into(),
                    })
                    .with_initialization_script(
                        r#"
                        window.WINDOW_ID = "$window_id  ";
                        window.WIDGET_ID = "$widget_id";
                        "#
                        .replace("$window_id", &format!("{:?}", new_window.id()))
                        .replace("$widget_id", &widget_config.id.0)
                        .as_str(),
                    )
                    .with_html(html.as_str())
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
                        info!("IPC handler received message: {:?}", message);
                        App::ipc_handler(message.body(), proxy_clone.clone());
                    })
                    .with_initialization_script(
                        r#"
                        window.WINDOW_ID = "$window_id  ";
                        window.WIDGET_ID = "$widget_id";
                        "#
                        .replace("$window_id", &format!("{:?}", new_window.id()))
                        .replace("$widget_id", &widget_config.id.0)
                        .as_str(),
                    )
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
                    last_scrape: None,
                    last_refresh: None,
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

    fn ipc_handler(body: &str, clone: Arc<EventLoopProxy<UserEvent>>) {
        info!("IPC handler received message: {:?}", body);
        let val = serde_json::from_str::<Value>(body).unwrap();
        info!("Value: {:?}", val);
        let message: ScrapedValue = serde_json::from_value(val["extractresult"].clone()).unwrap();
        clone.send_event(UserEvent::ExtractResult(message));
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
    ModifierEvent(WidgetModifier),
    MenuEvent(muda::MenuEvent),
    TrayIconEvent(tray_icon::TrayIconEvent),
    CreateWidget(CreateWidgetRequest),
    // Refresh(NanoId),
    // Scrape(NanoId, String),
    RemoveWebView(NanoId),
    // ShowNewViewForm,
    MoveWebView(NanoId, Direction),
    ExtractResult(ScrapedValue),
    Minimize(NanoId),
    ToggleElementView(NanoId),
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
    fn user_event(&mut self, event_loop: &ActiveEventLoop, userevent: UserEvent) {
        let size = self.current_size.clone();
        match userevent {
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
            UserEvent::CreateWidget(widget_options) => {
                info!("Creating new widget: {:?}", widget_options);
                let widget_config = WidgetConfiguration::new()
                    .with_widget_type(WidgetType::Url(UrlConfiguration {
                        url: widget_options.url,
                    }))
                    .with_level(widget_options.level);
                self.create_widget(event_loop, widget_config);
            }
            UserEvent::TrayIconEvent(trayevent) => match trayevent {
                _ => {
                    info!("Unhandled Tray icon event: {:?}", trayevent);
                }
            },
            UserEvent::MenuEvent(menu_event) => {
                info!("Menu event: {:?}", menu_event);
                let id = menu_event.id();
                info!("Menu id: {:?}", id);
                if id.0 == self.tray_menu_quit_id {
                    info!("Quitting application");
                    event_loop.exit();
                } else {
                    info!("Unhandled Menu event: {:?}", menu_event);
                }
            }
            UserEvent::ModifierEvent(modifier) => {
                info!("Modifier event: {:?}", modifier);
                match modifier.modifier_type {
                    Modifier::Refresh { interval_sec } => {
                        info!("Refreshing widget: {:?}", modifier.widget_id);
                        self.refresh_webview(modifier.widget_id, interval_sec);
                    }
                    Modifier::Scrape { selector } => {
                        info!("Scraping widget: {:?}", modifier.widget_id);
                        self.scrape_webview(modifier.widget_id, selector);
                    }
                }
            }
            _ => {
                info!("Unknown event: {:?}", userevent);
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
            winit::platform::macos::ActivationPolicy::Accessory,
            // winit::platform::macos::ActivationPolicy::Regular,
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
        WidgetConfiguration::new()
            .with_id(NanoId("Test SPY".to_string()))
            .with_title("Test SPY".to_string())
            .with_widget_type(WidgetType::Url(UrlConfiguration {
                url: "https://finance.yahoo.com/quote/SPY/".to_string(),
            }))
            .with_level(Level::Normal),
        WidgetConfiguration::new()
            .with_id(NanoId("testdata".to_string()))
            .with_title("Test Data View Widget".to_string())
            .with_widget_type(WidgetType::File(FileConfiguration {
                html: include_str!("../assets/widget_template.html").to_string(),
            }))
            .with_level(Level::Normal),
    ];

    let modifiers: Vec<WidgetModifier> = vec![WidgetModifier {
        id: 1,
        widget_id: NanoId("Test SPY".to_string()),
        modifier_type: Modifier::Scrape {
            selector: r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section:nth-child(2) > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string(),
        },
    },
    WidgetModifier {
        id: 2,
        widget_id: NanoId("testdata".to_string()),
        modifier_type: Modifier::Refresh { interval_sec: 5 },
    },
    ];

    info!("Debug Config: {:?}", config.len());

    let proxy_clone = event_loop_proxy.clone();
    let proxy_clone_muda = event_loop_proxy.clone();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        info!("Menu event: {:?}", event);
        proxy_clone_muda.send_event(UserEvent::MenuEvent(event));
    }));

    // setup the icon
    let img = image::load_from_memory(include_bytes!("/Users/jarde/Documents/misc/app-icon2.png"))
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = img.dimensions();

    let tray_menu = tray_icon::menu::Menu::new();
    let quit_item = tray_icon::menu::MenuItem::new("Quit Widget Maker", true, None);
    tray_menu.append(&quit_item).unwrap();

    let tray_quit_id = quit_item.id().0.clone();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(tray_icon::Icon::from_rgba(img.into_raw(), width, height).unwrap())
        .with_menu(Box::new(tray_menu))
        .build()
        .unwrap();
    let tray_icon_proxy = proxy_clone.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        info!("Tray icon event: {:?}", event);
        tray_icon_proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    let db = Arc::new(Mutex::new(db::db::Database::new()));
    {
        let mut db = db.lock().unwrap();
        match db.insert_widget_configuration(config) {
            Ok(_) => info!("Inserted widget configurations"),
            Err(e) => error!("Error inserting widget configurations: {:?}", e),
        }
        match db.insert_widget_modifiers(modifiers) {
            Ok(_) => info!("Inserted widget modifiers"),
            Err(e) => error!("Error inserting widget modifiers: {:?}", e),
        }
    }
    let api_proxy = Arc::new(event_loop_proxy.clone());
    let modifier_thread_event_proxy = Arc::new(event_loop_proxy.clone());
    let mut app = App {
        tray_menu_quit_id: tray_quit_id,
        current_size: LogicalSize::new(480, 360),
        // current_size: LogicalSize::new(320, 240),
        current_modifiers: Modifiers::default(),
        all_windows: HashMap::new(),
        widget_id_to_window_id: HashMap::new(),
        window_id_to_webview_id: HashMap::new(),
        db: db.clone(),
        proxy: Arc::new(event_loop_proxy),
        last_resize: None,
        // clipboard: arboard::Clipboard::new().unwrap(),
        menu,
    };

    // scraping thread
    let db_clone = db.clone();
    std::thread::spawn(move || {
        info!("Starting scraping thread");
        let mut min_refresh_interval = std::time::Duration::from_secs(4);
        // let mut last_refresh = HashMap::new();
        loop {
            std::thread::sleep(min_refresh_interval);
            {
                let mut modifiers: Vec<WidgetModifier> = vec![];
                {
                    let mut db = db_clone.lock().expect("Something failed");
                    let curr_modifiers = db.get_modifiers().expect("Something failed");
                    modifiers.extend(curr_modifiers);
                }
                info!("Found # modifiers: {:?}", modifiers.len());

                // instead of going through the configs, we ought to grab different events from the db
                // events including: scraping, refreshing, minimizing, etc.
                for modifier in modifiers {
                    let res =
                        modifier_thread_event_proxy.send_event(UserEvent::ModifierEvent(modifier));
                    if res.is_err() {
                        error!("Failed to send event to event loop");
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        rt.block_on(async {
            let state = ApiState {
                db: db.clone(),
                proxy: api_proxy.clone(),
            };

            let cors_layer = CorsLayer::new()
                .allow_methods(vec![
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::DELETE,
                ])
                .allow_headers(vec![http::HeaderName::from_static("content-type")])
                .allow_origin(AllowOrigin::any());
            let router = Router::new()
                .route("/values", get(get_values))
                .route("/sites", get(get_sites))
                .route("/elements", get(get_elements))
                // .route("/latest", get(get_latest_values))
                .route("/widgets/{widget_id}/latest", get(get_latest_values))
                .route("/widgets", get(get_widgets).post(create_widget))
                .route(
                    "/widgets/{id}/modifiers",
                    post(add_widget_modifier).get(get_widget_modifiers),
                )
                .route(
                    "/widgets/{widget_id}/modifiers/{modifier_id}",
                    delete(delete_widget_modifier),
                )
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

async fn create_widget(
    State(state): State<ApiState>,
    Json(widget_options): Json<CreateWidgetRequest>,
) -> impl IntoResponse {
    info!("Creating widget: {:?}", widget_options);

    let res = state
        .proxy
        .send_event(UserEvent::CreateWidget(widget_options.clone()));

    if res.is_err() {
        error!("Failed to send event to event loop");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::CREATED.into_response()
}

pub fn set_app_dock_icon(_window: &Window) {
    unsafe {
        let data = NSData::dataWithBytes_length_(
            nil,
            DOCK_ICON.as_ptr() as *const std::os::raw::c_void,
            DOCK_ICON.len() as u64,
        );

        let ns_image = NSImage::initWithDataIgnoringOrientation_(NSImage::alloc(nil), data);
        NSApplication::setApplicationIconImage_(NSApp(), ns_image);
    }
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

async fn get_values(State(state): State<ApiState>) -> Json<Vec<ScrapedValue>> {
    let state = state.db.try_lock().unwrap();
    let values: Vec<ScrapedValue> = state.get_data().unwrap();
    Json(values)
}

async fn get_latest_values(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
) -> Json<Vec<ScrapedValue>> {
    info!("Getting latest values for widget {}", widget_id);
    let state = state.db.try_lock().unwrap();
    let values: Vec<ScrapedValue> = state.get_latest_data().unwrap();
    Json(values)
}

async fn get_sites(State(state): State<ApiState>) -> Json<Vec<MonitoredSite>> {
    let state = state.db.try_lock().unwrap();
    let sites: Vec<MonitoredSite> = state.get_sites().unwrap();
    Json(sites)
}

async fn get_elements(State(state): State<ApiState>) -> Json<Vec<MonitoredElement>> {
    let state = state.db.try_lock().unwrap();
    let elements: Vec<MonitoredElement> = state.get_elements().unwrap();
    Json(elements)
}

async fn get_widgets(State(state): State<ApiState>) -> Json<Vec<WidgetConfiguration>> {
    info!("get widgets called");
    let state = state.db.try_lock().unwrap();
    let widgets = state.get_configuration().unwrap();
    info!("# widgets: {:?}", widgets.len());
    Json(widgets)
}

async fn get_widget_modifiers(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
) -> impl IntoResponse {
    info!("Getting modifiers for widget {}", widget_id);

    let db = state.db.try_lock().unwrap();
    match db.get_widget_modifiers(widget_id.as_str()) {
        Ok(modifiers) => Json(modifiers).into_response(),
        Err(e) => {
            error!("Failed to get modifiers: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn delete_widget_modifier(
    State(state): State<ApiState>,
    Path((widget_id, modifier_id)): Path<(String, String)>,
) -> impl IntoResponse {
    info!(
        "Deleting modifier {} from widget {}",
        modifier_id, widget_id
    );

    let mut db = state.db.try_lock().unwrap();
    match db.delete_widget_modifier(&modifier_id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            error!("Failed to delete modifier: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Clone)]
struct ApiState {
    db: Arc<Mutex<db::db::Database>>,
    proxy: Arc<EventLoopProxy<UserEvent>>,
}

async fn add_widget_modifier(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
    Json(modifier): Json<WidgetModifier>,
) -> impl IntoResponse {
    info!("Adding modifier to widget {}: {:?}", widget_id, modifier);

    let widget_modifier = WidgetModifier {
        id: 0,
        widget_id: NanoId(widget_id),
        modifier_type: match modifier.modifier_type {
            Modifier::Scrape { selector } => Modifier::Scrape { selector },
            Modifier::Refresh { interval_sec } => Modifier::Refresh { interval_sec },
        },
    };

    let mut db = state.db.try_lock().unwrap();
    match db.insert_widget_modifier(widget_modifier) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            error!("Failed to add modifier: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_serde_deserialize_from_webview() {
        let body = "{\"extractresult\":{\"error\":null,\"value\":\"562.01\",\"id\":\"Test SPY\",\"widget_id\":\"Test SPY\",\"timestamp\":\"2021-01-01T00:00:00.000Z\"}}";
        // serde_json::Value::from_str(s)
        let body_text = body.to_string();
        let message: ScrapedValue = serde_json::from_str(&body_text).unwrap();
        assert_eq!(message.widget_id, NanoId("Test SPY".to_string()));
        assert_eq!(message.value, "562.01");
        assert_eq!(message.timestamp, "2021-01-01T00:00:00.000Z");
    }
}
