use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    response::Response,
    routing::{delete, get, get_service, post},
    Router,
};

// mod api;

// use db::db::Database;
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
    fs::{File, OpenOptions},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::{runtime::Runtime, sync::Mutex, time::sleep};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::ServeFile,
};
use typeshare::typeshare;
use widget_types::{
    ApiAction, AppSettings, CreateWidgetRequest, EventSender, FileConfiguration, IpcEvent, Level,
    Modifier, ScrapedData, UrlConfiguration, WidgetConfiguration, WidgetModifier, WidgetType,
};
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

use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};

use wry::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, Position},
    PageLoadEvent, Rect, WebView, WebViewBuilder, WebViewBuilderExtDarwin, WebViewExtMacOS,
};

// mod db;

use image::{self, ImageBuffer};

pub const RESIZE_DEBOUNCE_TIME: u128 = 50;
pub const DEFAULT_SCRAPE_INTERVAL: u64 = 10;
pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

use nanoid::nanoid_gen;
use widget_types::NanoId;

mod event_sender;
pub use event_sender::WinitEventSender;

const MAX_WIDGETS: usize = 2;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ViewSize {
    width: u32,
    height: u32,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// struct AppConfig {
//     show_tray_icon: bool,
// }

struct App {
    tray_icon: TrayIcon,
    app_icon: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    tray_menu_quit_id: String,
    tray_menu_show_controls_id: String,
    tray_menu_hide_titlebar_id: String,
    tray_menu_show_titlebar_id: String,
    current_size: LogicalSize<u32>,
    menu: Menu,
    current_modifiers: Modifiers,
    proxy: EventLoopProxy<UserEvent>,
    last_resize: Option<Instant>,
    all_widgets: HashMap<WindowId, WidgetView>,
    widget_id_to_window_id: HashMap<NanoId, WindowId>,
    window_id_to_webview_id: HashMap<WindowId, NanoId>,
    db: widget_db::Database,
    app_settings: AppSettings,
    app_settings_path: PathBuf,
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

// struct ElementView {
//     webview: wry::WebView,
//     nano_id: NanoId,
//     visible: bool,
// }

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
        info!("Refreshing webview for: {}", id.0);

        let window_id = self.widget_id_to_window_id[&id];
        let Some(webview) = self.all_widgets.get_mut(&window_id) else {
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
        info!("Removing webview: {:?}", id);

        if let Some(window_id) = self.widget_id_to_window_id.get(&id) {
            self.all_widgets.remove(&window_id);
            self.window_id_to_webview_id.remove(&window_id);
            self.widget_id_to_window_id.remove(&id);
        } else {
            info!("Webview not found");
        }
    }

    // fn move_webview(&mut self, id: NanoId, direction: Direction) {
    //     todo!("Actualy implement this");
    // }

    // fn minimize_webview(&mut self, id: NanoId) {
    //     todo!("Minimize not implemented");
    // }

    fn scrape_webview(&self, widget_id: NanoId, element_selector: String) {
        info!("Scraping webview: {:?}", widget_id);
        info!("Widget id to window id: {:?}", self.widget_id_to_window_id);
        info!("window to webview id: {:?}", self.window_id_to_webview_id);

        let Some(window_id) = self.widget_id_to_window_id.get(&widget_id) else {
            info!("Webview not found");
            return;
        };

        let Some(widget_view) = self.all_widgets.get(&window_id) else {
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
    type: "extractresult",
    content: {
        error: "Element not found",
        value: null,
        widget_id: "$widget_id",
        timestamp: Date.now().toString(),
  }
  })
);
}

const scrape_value = element.getAttribute("aria-label") || element.textContent.trim();

window.ipc.postMessage(
JSON.stringify({
  type: "extractresult",
  content: {
    error: null,
    value: scrape_value,
    widget_id: "$widget_id",
    timestamp: Date.now().toString(),
  }
})
);
} catch (e) {
window.ipc.postMessage(
JSON.stringify({
  type: "extractresult",
  content: {
  error: JSON.stringify(e.message),
  value: null,
  widget_id: "$widget_id",
  timestamp: Date.now().toString(),
}
})
);
}

            "#,
        );

        let script_content = script_content
            .replace("$selector", &element_selector)
            .replace("$widget_id", &widget_view.nano_id.0.clone());

        let result = widget_view
            .app_webview
            .webview
            .evaluate_script(&script_content);

        info!("Scrape completed");
    }

    fn add_scrape_result(&mut self, result: ScrapedData) {
        if let Err(e) = self.db.insert_data(result) {
            error!("Failed to insert data: {:?}", e);
        }
    }

    fn create_widget(
        &mut self,
        event_loop: &ActiveEventLoop,
        widget_config: WidgetConfiguration,
        // size: LogicalSize<u32>,
    ) {
        // Check the widget limit, excluding the 'controls' widget
        let current_widget_count = self
            .all_widgets
            .values()
            .filter(|w| w.nano_id != NanoId("controls".to_string()))
            .count();

        if current_widget_count >= MAX_WIDGETS
            && widget_config.widget_id != NanoId("controls".to_string())
        {
            warn!(
                "Widget limit ({}) reached. Cannot create new widget: {}",
                MAX_WIDGETS, widget_config.title
            );
            // Optionally, send a message back to the UI or notify the user
            return;
        }

        // let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        // event_loop.set_allows_automatic_window_tabbing(enabled);
        let window_attributes = Window::default_attributes()
            .with_inner_size(self.current_size)
            .with_transparent(widget_config.transparent)
            // .with_blur(true) // barely supported, not really working
            .with_movable_by_window_background(true)
            .with_title_hidden(false)
            // .with_titlebar_buttons_hidden(false)
            // .with_titlebar_hidden(false)
            .with_title(&widget_config.title.clone())
            // .with_decorations(widget_config.decorations)
            .with_decorations(true)
            .with_has_shadow(false)
            .with_fullsize_content_view(true)
            .with_titlebar_transparent(false) // if false we can't move the window by dragging the titlebar
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

        let common_webview_attributes = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: self.current_size.into(),
            })
            .with_transparent(widget_config.transparent)
            .with_ipc_handler({
                let proxy_clone = self.proxy.clone();
                move |message| {
                    App::ipc_handler(message.body(), proxy_clone.clone());
                }
            })
            // .with_traffic_light_inset(PhysicalPosition::new(10, 10))
            .with_initialization_script(
                r#"
                window.WINDOW_ID = "$window_id  ";
                window.WIDGET_ID = "$widget_id";
                "#
                .replace("$window_id", &format!("{:?}", new_window.id()))
                .replace("$widget_id", &widget_config.widget_id.0)
                .as_str(),
            )
            .with_focused(true);

        let webview = match &widget_config.widget_type {
            WidgetType::File(file_config) => {
                let html = file_config.html.clone();
                // let html = html.replace("$widget_id", &widget_config.id.0);
                // let html = html.replace("$window_id", &format!("{:?}", new_window.id()));

                let webview = common_webview_attributes
                    .with_html(html.as_str())
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
                let webview = common_webview_attributes
                    .with_url(updated_url)
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
                .insert(widget_config.widget_id.clone(), new_window.id());
            self.window_id_to_webview_id
                .insert(new_window.id(), widget_config.widget_id.clone());
            self.all_widgets.insert(
                new_window.id(),
                WidgetView {
                    last_scrape: None,
                    last_refresh: None,
                    app_webview: AppWebView { webview: webview },
                    window: new_window,
                    nano_id: widget_config.widget_id.clone(),
                    visible: true,
                    options: WidgetOptions {
                        title: widget_config.title.clone(),
                        widget_type: widget_config.widget_type.clone(),
                    },
                },
            );
        }
    }

    fn ipc_handler(body: &str, proxy: EventLoopProxy<UserEvent>) {
        info!("IPC handler received message: {:?}", body);
        let val = serde_json::from_str::<Value>(body).unwrap();
        info!("Value: {:?}", val);

        let user_event = match serde_json::from_value::<IpcEvent>(val) {
            Ok(event) => event,
            Err(e) => {
                error!("Failed to deserialize ipc event: {:?}", e);
                return;
            }
        };
        proxy.send_event(UserEvent::IpcEvent(user_event));
    }

    fn show_controls(&mut self, event_loop: &ActiveEventLoop) {
        // check if controls widget is available, if yes focus it, otherwise create it
        let Some(window_id) = self
            .widget_id_to_window_id
            .get(&NanoId("controls".to_string()))
        else {
            info!("Controls widget not found");
            self.create_widget(event_loop, get_controls_widget_config());
            return;
        };
        let Some(window) = self.all_widgets.get(window_id) else {
            info!("Controls widget not found");
            return;
        };
        window.window.focus_window();
    }

    fn hide_titlebars(&mut self, event_loop: &ActiveEventLoop) {
        self.all_widgets.iter().for_each(|(_, widget)| {
            widget.window.set_decorations(false);
        });
    }

    fn show_titlebars(&mut self, event_loop: &ActiveEventLoop) {
        self.all_widgets.iter().for_each(|(_, widget)| {
            widget.window.set_decorations(true);
        });
    }

    fn update_app_settings(&mut self, settings: AppSettings) {
        info!("Updating app settings: {:?}", settings);
        // self.app_settings = settings;
        // save the settings to the file
        let settings_json = serde_json::to_string(&settings).unwrap();
        std::fs::write(self.app_settings_path.clone(), settings_json).unwrap();
        // update the tray icon
        // (&self.app_settings.show_tray_icon);
        if !settings.show_tray_icon {
            // self.tray_menu_quit_id = None;
            // self.tray_menu_show_controls_id = None;
            self.tray_icon.set_visible(false);
        } else {
            self.tray_icon.set_visible(true);
        }

        self.app_settings = settings;
    }
}

fn setup_tray_menu(
    app_icon: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    proxy_clone: EventLoopProxy<UserEvent>,
) -> (String, String, String, String, TrayIcon) {
    let tray_menu = tray_icon::menu::Menu::new();
    let quit_item = tray_icon::menu::MenuItem::new("Quit Widget Maker", true, None);
    let show_controls_item = tray_icon::menu::MenuItem::new("Show controls", true, None);
    let hide_titlebar_item = tray_icon::menu::MenuItem::new("Hide titlebars", true, None);
    let show_titlebar_item = tray_icon::menu::MenuItem::new("Show titlebars", true, None);

    // Append items and the separator correctly
    tray_menu.append(&show_controls_item).unwrap();
    tray_menu.append(&hide_titlebar_item).unwrap();
    tray_menu.append(&show_titlebar_item).unwrap();
    tray_menu
        .append(&tray_icon::menu::PredefinedMenuItem::separator())
        .unwrap();
    tray_menu.append(&quit_item).unwrap();

    let tray_quit_id = quit_item.id().0.clone();
    let tray_show_controls_id = show_controls_item.id().0.clone();
    let tray_hide_titlebar_id = hide_titlebar_item.id().0.clone();
    let tray_show_titlebar_id = show_titlebar_item.id().0.clone();

    let (width, height) = app_icon.dimensions();
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Widget Maker")
        .with_icon(tray_icon::Icon::from_rgba(app_icon.into_raw(), width, height).unwrap())
        .with_menu(Box::new(tray_menu))
        .build()
        .unwrap();
    let tray_icon_proxy = proxy_clone.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        info!("Tray icon event: {:?}", event);
        tray_icon_proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    (
        tray_quit_id,
        tray_show_controls_id,
        tray_hide_titlebar_id,
        tray_show_titlebar_id,
        tray_icon,
    )
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
    IpcEvent(IpcEvent),
    ApiAction(ApiAction),
    ModifierEvent(WidgetModifier),
    MenuEvent(muda::MenuEvent),
    TrayIconEvent(tray_icon::TrayIconEvent),
    CreateWidget(CreateWidgetRequest),
    // Refresh(NanoId),
    // Scrape(NanoId, String),
    RemoveWebView(NanoId),
    // ShowNewViewForm,
    MoveWebView(NanoId, Direction),
    Minimize(NanoId),
    ToggleElementView(NanoId),
    ExtractResult(ScrapedData),
    SaveSettings(AppSettings),
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
            info!("TODO: Get widgets from db");
            let config = self.db.get_configuration().unwrap();
            widgets.extend_from_slice(&config);
        }

        info!("Found {} widgets", widgets.len());
        for widget_config in widgets {
            self.create_widget(event_loop, widget_config);
        }
        info!(
            "Widgets: {:?}",
            &self
                .all_widgets
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
                self.all_widgets.remove(&window_id);
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
                    .all_widgets
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
            WindowEvent::CursorEntered { device_id } => {
                info!("Cursor entered: {:?}", device_id);
            }
            WindowEvent::CursorLeft { device_id } => {
                info!("Cursor left: {:?}", device_id);
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
                info!("Removing webview at index {}", id.0);
                self.remove_webview(id);
            }
            UserEvent::MoveWebView(id, direction) => {
                info!("Moving webview at index {} {}", id.0, direction);
                // self.move_webview(id, direction);
            }
            // UserEvent::ExtractResult(result) => {
            //     info!("Extracted result: {:?}", result);
            //     self.add_scrape_result(result.clone());
            //     // self.update_element_view(result);
            // }
            UserEvent::Minimize(id) => {
                info!("Minimizing webview at index {}", id.0);
                // self.minimize_webview(id);
            }
            UserEvent::CreateWidget(widget_options) => {
                info!("Creating new widget: {:?}", widget_options);

                let widget_config = WidgetConfiguration::new()
                    .with_widget_type(if widget_options.url.is_some() {
                        WidgetType::Url(UrlConfiguration {
                            url: widget_options.url.unwrap(),
                        })
                    } else {
                        WidgetType::File(FileConfiguration {
                            html: widget_options.html.unwrap(),
                        })
                    })
                    .with_transparent(widget_options.transparent)
                    .with_level(widget_options.level)
                    .with_title(widget_options.title);
                let res: Result<(), ()> = {
                    // let mut db = futures::executor::block_on(self.db.lock());
                    // futures::executor::block_on(
                    //     db.insert_widget_configuration(vec![widget_config.clone()]),
                    // )
                    info!("TODO: Inserting widget configuration: {:?}", widget_config);
                    Ok(())
                };
                info!("Inserted widget configuration: {:?}", res);
                if res.is_err() {
                    error!("Failed to insert widget configuration: {:?}", res);
                }
                self.create_widget(event_loop, widget_config.clone());
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

                match id.0.as_str() {
                    val if val == self.tray_menu_quit_id => {
                        info!("Quitting application");
                        event_loop.exit();
                    }
                    val if val == self.tray_menu_show_controls_id => {
                        info!("Showing controls");
                        self.show_controls(event_loop);
                    }
                    val if val == self.tray_menu_hide_titlebar_id => {
                        info!("Hiding titlebars");
                        self.hide_titlebars(event_loop);
                    }
                    val if val == self.tray_menu_show_titlebar_id => {
                        info!("Showing titlebars");
                        self.show_titlebars(event_loop);
                    }
                    _ => {
                        info!("No tray menu show controls id found");
                    }
                }
            }
            UserEvent::ModifierEvent(modifier) => {
                info!("Modifier event: {:?}", modifier);
                match modifier.modifier_type {
                    Modifier::Refresh {
                        modifier_id,
                        interval_sec,
                    } => {
                        info!("Refreshing widget: {:?}", modifier_id);
                        self.refresh_webview(modifier_id, interval_sec);
                    }
                    Modifier::Scrape {
                        modifier_id,
                        selector,
                    } => {
                        info!("Scraping widget: {:?}", modifier.widget_id);
                        self.scrape_webview(modifier.widget_id, selector);
                    }
                }
            }
            UserEvent::ApiAction(action) => {
                info!("Api action: {:?}", action);
                match action {
                    ApiAction::DeleteWidget(widget_id) => {
                        self.remove_webview(NanoId(widget_id));
                    } // ApiAction::DeleteWidgetModifier(widget_id, modifier_id) => {
                      //     self.remove_modifier(NanoId(widget_id), NanoId(modifier_id));
                      // }
                }
            }
            UserEvent::IpcEvent(ipc_event) => {
                info!("Ipc event: {:?}", ipc_event);
                match ipc_event {
                    IpcEvent::SaveSettings(app_settings) => {
                        self.update_app_settings(app_settings);
                    }
                    IpcEvent::ExtractResult(scraped_data) => {
                        self.add_scrape_result(scraped_data);
                    }
                }
            }
            _ => {
                info!("Unknown event: {:?}", userevent);
            }
        }
    }
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
            &PredefinedMenuItem::about(Some("Widget Maker"), None),
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

fn get_controls_widget_config() -> WidgetConfiguration {
    WidgetConfiguration::new()
        .with_widget_id(NanoId("controls".to_string()))
        .with_title("Controls".to_string())
        .with_widget_type(WidgetType::File(FileConfiguration {
            html: include_str!("../../react-ui/dist/index.html").to_string(),
        }))
        .with_level(Level::Normal)
}

fn load_app_settings(settings_filepath: PathBuf) -> AppSettings {
    if !settings_filepath.exists() {
        let app_settings = AppSettings {
            show_tray_icon: true,
        };
        std::fs::create_dir_all(settings_filepath.parent().unwrap()).unwrap();
        let config_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(settings_filepath.clone())
            .unwrap();
        serde_json::to_writer(&config_file, &app_settings).unwrap();
    }
    let app_settings =
        serde_json::from_reader(File::open(settings_filepath.clone()).unwrap()).unwrap();
    app_settings
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    // loading app settings from user directory
    let directory = directories::ProjectDirs::from("com", "jarde", "widget-maker").unwrap();
    let config_dir = directory.config_dir();
    let settings_filepath = config_dir.join("settings.json");
    let app_settings = load_app_settings(settings_filepath);
    info!("App settings: {:?}", app_settings);

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
        get_controls_widget_config(),
        // WidgetConfiguration::new()
        //     .with_widget_id(NanoId("Test SPY".to_string()))
        //     .with_title("Test SPY".to_string())
        //     .with_widget_type(WidgetType::Url(UrlConfiguration {
        //         url: "https://finance.yahoo.com/quote/SPY/".to_string(),
        //     }))
        //     .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_widget_id(NanoId("testdata".to_string()))
        //     .with_title("Test Data View Widget".to_string())
        //     .with_widget_type(WidgetType::File(FileConfiguration {
        //         html: include_str!("../assets/widget_template.html").to_string(),
        //     }))
        //     .with_level(Level::Normal),
        // WidgetConfiguration::new()
        //     .with_widget_id(NanoId("transparent".to_string()))
        //     .with_title("Transparent Widget".to_string())
        //     .with_widget_type(WidgetType::File(FileConfiguration {
        //         html: include_str!("../assets/widget_transparent.html").to_string(),
        //     }))
        //     .with_transparent(true)
        //     .with_level(Level::Normal),
    ];

    let modifiers: Vec<WidgetModifier> = vec![
        // WidgetModifier {
        //     id: 1,
        //     widget_id: NanoId("Test SPY".to_string()),
        //     modifier_type: Modifier::Scrape {
        //         modifier_id: NanoId("scrape1".to_string()),
        //         selector: r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section:nth-child(2) > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string(),
        //     },
        // },
        // WidgetModifier {
        //     id: 2,
        //     widget_id: NanoId("testdata".to_string()),
        //     modifier_type: Modifier::Refresh {
        //         modifier_id: NanoId("refresh1".to_string()),
        //         interval_sec: 5,
        //     },
        // },
    ];

    info!("Debug Config: {:?}", config.len());

    let proxy_clone = event_loop_proxy.clone();
    let proxy_clone_muda = event_loop_proxy.clone();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        info!("Menu event: {:?}", event);
        proxy_clone_muda.send_event(UserEvent::MenuEvent(event));
    }));

    // setup the icon
    let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::load_from_memory(include_bytes!("/Users/jarde/Documents/misc/app-icon2.png"))
            .expect("Failed to load icon")
            .into_rgba8();
    let (width, height) = img.dimensions();

    let (
        tray_quit_id,
        tray_show_controls_id,
        tray_hide_titlebar_id,
        tray_show_titlebar_id,
        tray_icon,
    ) = setup_tray_menu(img.clone(), event_loop_proxy.clone());
    let app_db = widget_db::Database::new().unwrap();
    tray_icon.set_visible(app_settings.show_tray_icon);

    let mut app = App {
        tray_icon,
        app_icon: img,
        db: app_db,
        app_settings,
        app_settings_path: config_dir.join("settings.json"),
        // tray_menu_quit_id2: tray_quit_id.clone(),
        // tray_menu_show_controls_id2: tray_show_controls_id.clone(),
        tray_menu_quit_id: tray_quit_id,
        tray_menu_show_controls_id: tray_show_controls_id,
        tray_menu_hide_titlebar_id: tray_hide_titlebar_id,
        tray_menu_show_titlebar_id: tray_show_titlebar_id,
        current_size: LogicalSize::new(480, 360),
        current_modifiers: Modifiers::default(),
        all_widgets: HashMap::new(),
        widget_id_to_window_id: HashMap::new(),
        window_id_to_webview_id: HashMap::new(),
        proxy: event_loop_proxy.clone(),
        last_resize: None,
        menu,
    };

    let modifier_thread_proxy = event_loop_proxy.clone();

    let event_sender = WinitEventSender::new(event_loop_proxy.clone());
    let rt = Runtime::new().unwrap();
    thread::spawn(move || {
        // Execute the future, blocking the current thread until completion
        rt.block_on(async {
            let mut api_db = widget_db::Database::new().unwrap();
            let res = api_db.insert_widget_configuration(config);
            match res {
                Ok(_) => info!("Inserted widget configurations"),
                Err(e) => error!("Error inserting widget configurations: {:?}", e),
            }
            let res = api_db.insert_widget_modifiers(modifiers);
            match res {
                Ok(_) => info!("Inserted widget modifiers"),
                Err(e) => error!("Error inserting widget modifiers: {:?}", e),
            }
            widget_db::run_api(
                Arc::new(Mutex::new(api_db)),
                event_sender.into_event_sender(),
            )
            .await;
        });
    });

    thread::spawn(move || loop {
        let modifier_db_access = widget_db::Database::new().unwrap();
        let mut last_refresh_dict = HashMap::new();
        let mut last_scrape_dict = HashMap::new();

        loop {
            let widget_modifiers = modifier_db_access.get_all_widget_modifiers();
            let Ok(widget_modifiers) = widget_modifiers else {
                error!(
                    "Error getting widget modifiers: {:?}",
                    widget_modifiers.err().unwrap()
                );
                continue;
            };
            for modifier in widget_modifiers {
                // info!("Modifier: {:?}", modifier);
                let modifier_clone = modifier.clone();
                match modifier.modifier_type {
                    Modifier::Refresh {
                        modifier_id,
                        interval_sec,
                    } => {
                        // info!("Refresh modifier: {:?}", modifier_id);
                        let last_update = last_refresh_dict
                            .entry(modifier_id.clone())
                            .or_insert(Instant::now());

                        if last_update.elapsed().as_secs() >= interval_sec as u64 {
                            info!("Refreshing widget: {:?}", modifier_id);
                            // last_update = Instant::now();
                            last_refresh_dict.insert(modifier_id.clone(), Instant::now());
                        }
                    }
                    Modifier::Scrape {
                        modifier_id,
                        selector,
                    } => {
                        info!("Scrape modifier: {:?}", modifier_id);
                        let last_update = last_scrape_dict
                            .entry(modifier_id.clone())
                            .or_insert(Instant::now());
                        if last_update.elapsed().as_secs() >= DEFAULT_SCRAPE_INTERVAL {
                            info!("Scraping widget: {:?}", modifier_id);
                            modifier_thread_proxy
                                .send_event(UserEvent::ModifierEvent(modifier_clone));
                            last_scrape_dict.insert(modifier_id.clone(), Instant::now());
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    event_loop.run_app(&mut app).expect("Something failed");
}

#[cfg(test)]
mod tests {
    use super::*;
}
