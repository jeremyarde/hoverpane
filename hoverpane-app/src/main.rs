#![allow(warnings)]

use env_logger::Builder;

use log::{error, info, warn, LevelFilter};
use muda::{accelerator::Modifiers, Menu, MenuId, PredefinedMenuItem, Submenu};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::{runtime::Runtime, sync::Mutex};
use widget_types::{
    ApiAction, AppSettings, AppUiState, ConfigInformation, CreateCheckoutSessionResponse,
    CreateWidgetRequest, FileConfiguration, IpcEvent, Level, LicenceTier, Modifier,
    MonitorPosition, ScrapedData, UrlConfiguration, VersionInfo, WidgetBounds, WidgetConfiguration,
    WidgetModifier, WidgetType, API_PORT, DEFAULT_WIDGET_HEIGHT, DEFAULT_WIDGET_WIDTH,
    DEFAULT_WIDGET_X, DEFAULT_WIDGET_Y,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    platform::macos::{EventLoopBuilderExtMacOS, MonitorHandleExtMacOS, WindowAttributesExtMacOS},
    window::{self, Window, WindowId, WindowLevel},
};

// use cocoa::{
//     appkit::{NSApp, NSObject},
//     base::nil,
//     foundation::{NSAutoreleasePool, NSData, NSString, NSUserDefaults},
// };
// use cocoa::foundation::NSString;
// use cocoa::{appkit::*, foundation::NSData};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    #[error("Failed to get checkout session url: {0}")]
    CheckoutSessionUrl(String),
}

const DOCK_ICON: &[u8] = include_bytes!("../build_assets/icon.png");
const TRAY_ICON: &[u8] = include_bytes!("../build_assets/tray-icon.png");
// const TRAY_ICON_WHITE: &[u8] = include_bytes!("../build_assets/tray-icon-white.png");

use tray_icon::{TrayIcon, TrayIconAttributes, TrayIconBuilder};

use wry::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
    Rect, WebView, WebViewBuilder, WebViewExtMacOS,
};

// mod db;

use image::ImageBuffer;

pub const RESIZE_DEBOUNCE_TIME: u128 = 50;
pub const DEFAULT_SCRAPE_INTERVAL: u64 = 5;
pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

use widget_types::NanoId;

mod event_sender;
pub use event_sender::WinitEventSender;

// conditionally set the max widgets based on the environment variable
#[cfg(feature = "pro")]
const MAX_WIDGETS: usize = 20;

#[cfg(not(feature = "pro"))]
const MAX_WIDGETS: usize = 3;

mod updater;
use updater::{UpdateInfo, Updater};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ViewSize {
    width: u32,
    height: u32,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// struct AppConfig {
//     show_tray_icon: bool,
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// enum Theme {
//     Light,
//     Dark,
// }

// struct AppTheme {
//     mode: Mode,
//     light_icon: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
//     dark_icon: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
// }

// impl AppTheme {
//     fn new() -> Self {
//         // let mode = dark_light::detect().unwrap_or(Mode::Light);
//         // let light_icon = image::load_from_memory(TRAY_ICON)
//         //     .expect("Failed to load icon")
//         //     .into_rgba8();
//         // // let dark_icon = image::load_from_memory(TRAY_ICON_WHITE)
//         //     .expect("Failed to load icon")
//         //     .into_rgba8();
//         Self {
//             mode,
//             light_icon,
//             dark_icon,
//         }
//     }

//     fn get_icon(&self) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
//         let mode = dark_light::detect().unwrap_or(Mode::Light);
//         if mode == Mode::Dark {
//             self.dark_icon.clone()
//         } else {
//             self.light_icon.clone()
//         }
//     }
// }
struct App {
    updater: Updater,
    tray_icon: TrayIcon,
    // theme: AppTheme,
    menu_items: MenuItems,
    current_size: LogicalSize<u32>,
    menu: Menu,
    current_modifiers: Modifiers,
    proxy: EventLoopProxy<UserEvent>,
    last_resize: Option<Instant>,
    all_widgets: HashMap<WindowId, WidgetView>,
    widget_id_to_window_id: HashMap<NanoId, WindowId>,
    window_id_to_widget_id: HashMap<WindowId, NanoId>,
    db: widget_db::Database,
    settings: DesktopAppSettings,
    ui_state: AppUiState,
}

struct WidgetView {
    // webview: wry::WebView,
    app_webview: AppWebView,
    last_scrape: Option<Instant>,
    last_refresh: Instant,
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

        let window_id = self.widget_id_to_window_id.get(&id);
        if window_id.is_none() {
            error!("Webview not found or window is closed");
            return;
        }

        let Some(webview) = self.all_widgets.get_mut(&window_id.unwrap()) else {
            error!("Webview not found");
            return;
        };

        if webview.last_refresh.elapsed() > Duration::from_secs(refresh_interval_secs as u64) {
            webview.last_refresh = Instant::now();
        } else {
            info!("Skipping refresh for widget: {:?}", id);
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
            self.all_widgets.remove(window_id);
            self.window_id_to_widget_id.remove(window_id);
            self.widget_id_to_window_id.remove(&id);
        } else {
            info!("Webview not found");
        }
    }

    fn scrape_webview(&self, widget_id: NanoId, element_selector: String) {
        info!("Scraping webview: {:?}", widget_id);
        info!("Widget id to window id: {:?}", self.widget_id_to_window_id);
        info!("window to webview id: {:?}", self.window_id_to_widget_id);

        let Some(window_id) = self.widget_id_to_window_id.get(&widget_id) else {
            info!("Webview not found");
            return;
        };

        let Some(widget_view) = self.all_widgets.get(window_id) else {
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
            .replace("$widget_id", &widget_view.nano_id.0.clone())
            .replace("$PORT", &API_PORT.to_string());

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

    fn create_widget(&mut self, event_loop: &ActiveEventLoop, widget_config: WidgetConfiguration) {
        // Check the widget limit, excluding the 'controls' widget
        let current_widget_count = self
            .all_widgets
            .values()
            .filter(|w| w.nano_id != NanoId("controls".to_string()))
            .count();

        // if current_widget_count >= MAX_WIDGETS
        //     && widget_config.widget_id != NanoId("controls".to_string())
        // {
        //     warn!(
        //         "Widget limit ({}) reached. Cannot create new widget: {}",
        //         MAX_WIDGETS, widget_config.title
        //     );
        //     // Optionally, send a message back to the UI or notify the user
        //     return;
        // }

        // check if widget is visible or not
        if !widget_config.is_open {
            info!("Widget is not visible, skipping creation");
            return;
        }

        let log_position =
            LogicalPosition::new(widget_config.bounds.x as f64, widget_config.bounds.y as f64);

        let size = LogicalSize::new(
            widget_config.bounds.width as f64,
            widget_config.bounds.height as f64,
        );
        let window_attributes = Window::default_attributes()
            .with_accepts_first_mouse(true)
            .with_position(log_position)
            .with_inner_size(size)
            .with_transparent(widget_config.transparent)
            // .with_blur(true) // barely supported, not really working
            .with_movable_by_window_background(true)
            .with_title_hidden(false)
            // .with_titlebar_buttons_hidden(false)
            // .with_titlebar_hidden(false)
            .with_title(widget_config.title.clone())
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
                    .with_title(widget_config.title.clone())
                    .with_window_level(match widget_config.level {
                        Level::AlwaysOnTop => WindowLevel::AlwaysOnTop,
                        Level::Normal => WindowLevel::Normal,
                        Level::AlwaysOnBottom => WindowLevel::AlwaysOnBottom,
                    }),
            )
            .expect("Something failed");

        // let scale_factor = new_window.scale_factor();

        let common_webview_attributes = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: self.current_size.into(),
            })
            // .with_drag_drop_handler(false)
            .with_transparent(widget_config.transparent)
            .with_ipc_handler({
                info!("Recieved ipc message");
                let proxy_clone = self.proxy.clone();
                move |message| {
                    App::ipc_handler(message.body(), proxy_clone.clone());
                }
            })
            .with_focused(false)
            .with_hotkeys_zoom(true)
            // .with_traffic_light_inset(PhysicalPosition::new(10, 10))
            .with_initialization_script(
                r#"
                window.WINDOW_ID = "$window_id  ";
                window.WIDGET_ID = "$widget_id";
                window.PORT = "$PORT";

                // Add CSS to ensure 100% width and height
                const style = document.createElement('style');
                style.textContent = `
                    html, body {
                        margin: 0;
                        padding: 0;
                        width: 100%;
                        height: 100%;
                        overflow: hidden;
                    }
                `;
                document.head.appendChild(style);
                "#
                .replace("$window_id", &format!("{:?}", new_window.id()))
                .replace("$widget_id", &widget_config.widget_id.0)
                .replace("$PORT", &API_PORT.to_string())
                .as_str(),
            );

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

        if webview.is_none() {
            error!("Failed to create webview");
            return;
        }

        self.widget_id_to_window_id
            .insert(widget_config.widget_id.clone(), new_window.id());
        self.window_id_to_widget_id
            .insert(new_window.id(), widget_config.widget_id.clone());
        self.all_widgets.insert(
            new_window.id(),
            WidgetView {
                last_scrape: None,
                last_refresh: Instant::now(),
                app_webview: AppWebView {
                    webview: webview.unwrap(),
                },
                window: new_window,
                nano_id: widget_config.widget_id.clone(),
                visible: true,
                options: WidgetOptions {
                    title: widget_config.title.clone(),
                    widget_type: widget_config.widget_type.clone(),
                },
            },
        );

        // todo: update or create a new widget in the database here?

        info!("Widget created: {:?}", widget_config.title);
    }

    fn ipc_handler(body: &str, proxy: EventLoopProxy<UserEvent>) {
        info!("IPC handler received message: {:?}", body);
        let val = serde_json::from_str::<Value>(body).unwrap();
        info!("Value: {:?}", val);

        let user_event = match serde_json::from_value::<IpcEvent>(val) {
            Ok(event) => event,
            Err(e) => {
                // let mut app_ui_state = self.db.get_app_ui_state().unwrap().clone();
                // app_ui_state
                //     .messages
                //     .push(format!("Failed to deserialize ipc event: {:?}", e));
                // self.db.set_app_ui_state(&app_ui_state).unwrap();
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
            // let mut widget_config = get_controls_widget_config();
            // widget_config.is_open = true;
            info!("Controls widget not found");
            self.create_widget(event_loop, get_controls_widget_config());
            return;
        };
        let Some(window) = self.all_widgets.get(window_id) else {
            info!("Controls widget not found in widgets");
            return;
        };
        window.window.focus_window();
    }

    fn hide_titlebars(&mut self, event_loop: &ActiveEventLoop) {
        self.all_widgets.iter().for_each(|(_, widget)| {
            widget.window.set_decorations(false);
        });
    }

    fn reset_database(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resetting database");
        self.db.reset();
    }

    fn show_titlebars(&mut self, event_loop: &ActiveEventLoop) {
        self.all_widgets.iter().for_each(|(_, widget)| {
            widget.window.set_decorations(true);
        });
    }

    // fn check_theme(&mut self) {
    //     if self.theme.mode != dark_light::detect().unwrap_or(Mode::Light) {
    //         self.theme.mode = dark_light::detect().unwrap_or(Mode::Light);
    //         let icon = tray_icon::Icon::from_rgba(
    //             self.theme.get_icon().into_raw(),
    //             self.theme.get_icon().width(),
    //             self.theme.get_icon().height(),
    //         )
    //         .unwrap();
    //         self.tray_icon.set_icon(Some(icon));
    //     }
    // }

    fn update_app_settings(&mut self, settings: AppSettings) {
        info!("Updating app settings: {:?}", settings);
        // Save to database
        if let Err(e) = self.db.set_settings(&settings) {
            error!("Failed to save settings to db: {:?}", e);
        }

        // update the tray icon
        if !settings.show_tray_icon {
            self.tray_icon.set_visible(false);
        } else {
            self.tray_icon.set_visible(true);
        }
        self.settings.app_settings = settings;
    }

    fn toggle_visibility(
        &mut self,
        event_loop: &ActiveEventLoop,
        widget_id: String,
        visible: bool,
    ) {
        if let Some(window_id) = self.widget_id_to_window_id.get(&NanoId(widget_id.clone())) {
            info!("TOGGLE VISIBILITY: Widget {:?} found", widget_id);
            let widget = self.all_widgets.get_mut(window_id).unwrap();
            widget.visible = !widget.visible;
            widget.window.set_visible(widget.visible);
            self.db
                .update_widget_open_state(NanoId(widget_id), widget.visible);
        } else {
            // create the widget based on widget in the database
            let mut widget_config = self
                .db
                .get_widget_configuration_by_id(widget_id.clone().as_str())
                .unwrap();
            widget_config.is_open = true;
            self.db.update_widget_open_state(NanoId(widget_id), true);
            self.create_widget(event_loop, widget_config);
        }
    }

    fn update_widget_bounds(&mut self, widget_id: String, bounds: WidgetBounds) {
        if let Some(window_id) = self.widget_id_to_window_id.get(&NanoId(widget_id.clone())) {
            let widget = self.all_widgets.get_mut(window_id).unwrap();
            widget
                .window
                .request_inner_size(LogicalSize::new(bounds.width, bounds.height));
            widget
                .window
                .set_outer_position(LogicalPosition::new(bounds.x, bounds.y));
            self.db.update_widget_bounds(widget_id, bounds);
        } else {
            info!("Widget {:?} not found", widget_id);
        }
    }

    fn maximize_webview(&mut self, nano_id: NanoId) {
        if let Some(window_id) = self.widget_id_to_window_id.get(&nano_id) {
            let widget = self.all_widgets.get_mut(window_id).unwrap();
            widget.window.set_maximized(true);
        } else {
            info!("Widget {:?} not found", nano_id);
        }
    }

    fn minimize_webview(&mut self, nano_id: NanoId) {
        if let Some(window_id) = self.widget_id_to_window_id.get(&nano_id) {
            let widget = self.all_widgets.get_mut(window_id).unwrap();
            widget.window.set_maximized(false);
            widget.window.request_inner_size(LogicalSize::new(0, 0));
            widget
                .window
                .set_outer_position(LogicalPosition::new(10, 10));
        } else {
            info!("Widget {:?} not found", nano_id);
        }
    }

    fn handle_drag_event(&mut self, drag_event: widget_types::DragEvent) {
        info!("Drag event: {:?}", drag_event);
        let Some(window_id) = self
            .widget_id_to_window_id
            .get(&NanoId(drag_event.widget_id.clone()))
        else {
            info!("Widget {:?} not found", drag_event.widget_id);
            return;
        };

        let widget = self.all_widgets.get_mut(window_id).unwrap();
        if let Ok(current_position) = widget.window.outer_position() {
            info!("Current position: {:?}", current_position);
            info!("Drag event: {:?}", drag_event);

            // Get the scale factor for this window
            let scale_factor = widget.window.scale_factor();

            // Convert logical position to physical position and add the drag movement
            widget.window.set_outer_position(PhysicalPosition::new(
                (current_position.x as f64) + (drag_event.x as f64 * scale_factor),
                (current_position.y as f64) + (drag_event.y as f64 * scale_factor),
            ));
        } else {
            error!(
                "Failed to get window position for widget {:?}",
                drag_event.widget_id
            );
        }
    }

    fn open_external_browser(&self, checkout_url: String) {
        let url = format!("https://hoverpane.com/buy?email={}", checkout_url);
        info!("Opening external browser to: {}", url);
        let _ = open::that(url).unwrap();
    }

    fn get_checkout_session_url(&self, user_email: String) -> Result<String, AppError> {
        let checkout_url = format!(
            "{}/stripe/generate-stripe-checkout",
            self.settings.api_base_url
        );
        let request = json!({ "email": user_email });
        info!("Request: {:?}", request);

        let checkout_session_response = reqwest::blocking::Client::new()
            .post(checkout_url)
            .json(&request)
            .send()
            .map_err(|e| {
                error!("Failed to get checkout session url: {:?}", e);
                return AppError::CheckoutSessionUrl(e.to_string());
            })?
            .json::<CreateCheckoutSessionResponse>()
            .map_err(|e| {
                error!("Failed to deserialize response: {:?}", e);
                return AppError::CheckoutSessionUrl(e.to_string());
            })?;
        Ok(checkout_session_response.checkout_session_url)
    }

    // fn remove_widget_modifier(&self, widget_id: String, modifier_id: String) -> Result<(), ()> {
    //     self.db.delete_widget_modifier(modifier_id);

    //     let widget = self.all_widgets.get_mut(&NanoId(widget_id.clone()));
    //     if widget.is_none() {
    //         error!("Widget {:?} not found", widget_id);
    //         return Err(());
    //     }
    //     let widget = widget.unwrap();
    //     widget.modifiers.remove(&NanoId(modifier_id.clone()));
    //     Ok(())
    // }
}

pub struct MenuItems {
    pub quit_id: String,
    pub show_controls_id: String,
    pub hide_titlebar_id: String,
    pub show_titlebar_id: String,
    pub reset_database_id: String,
    pub check_updates_id: String,
}

fn setup_tray_menu(
    user_version: LicenceTier,
    // theme: &AppTheme,
    proxy_clone: EventLoopProxy<UserEvent>,
) -> (MenuItems, TrayIcon) {
    let tray_menu = tray_icon::menu::Menu::new();
    let quit_item = tray_icon::menu::MenuItem::new("Quit HoverPane", true, None);
    let show_controls_item = tray_icon::menu::MenuItem::new("Show controls", true, None);
    let hide_titlebar_item = tray_icon::menu::MenuItem::new("Hide titlebars", true, None);
    let show_titlebar_item = tray_icon::menu::MenuItem::new("Show titlebars", true, None);
    let reset_database_item = tray_icon::menu::MenuItem::new("Reset database", true, None);
    let check_updates_item = tray_icon::menu::MenuItem::new("Check for updates...", true, None);
    let version_item = tray_icon::menu::MenuItem::new(
        format!(
            "HoverPane ({:?}) v{}",
            user_version,
            env!("CARGO_PKG_VERSION")
        ),
        false,
        None,
    );

    // Append items and the separator correctly
    tray_menu.append(&show_controls_item).unwrap();
    tray_menu.append(&hide_titlebar_item).unwrap();
    tray_menu.append(&show_titlebar_item).unwrap();
    tray_menu.append(&reset_database_item).unwrap();
    tray_menu.append(&check_updates_item).unwrap();
    tray_menu.append(&version_item).unwrap();
    tray_menu
        .append(&tray_icon::menu::PredefinedMenuItem::separator())
        .unwrap();
    tray_menu.append(&quit_item).unwrap();

    let tray_quit_id = quit_item.id().0.clone();
    let tray_show_controls_id = show_controls_item.id().0.clone();
    let tray_hide_titlebar_id = hide_titlebar_item.id().0.clone();
    let tray_show_titlebar_id = show_titlebar_item.id().0.clone();
    let tray_reset_database_id = reset_database_item.id().0.clone();
    let tray_check_updates_id = check_updates_item.id().0.clone();

    // let icon = AppTheme::get_icon().clone();
    let icon = image::load_from_memory(TRAY_ICON)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = icon.dimensions();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("HoverPane")
        .with_icon(tray_icon::Icon::from_rgba(icon.into_raw(), width, height).unwrap())
        .with_menu(Box::new(tray_menu))
        .with_icon_as_template(true)
        .build()
        .unwrap();
    let tray_icon_proxy = proxy_clone.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        // info!("Tray icon event: {:?}", event);
        tray_icon_proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    (
        MenuItems {
            quit_id: tray_quit_id,
            show_controls_id: tray_show_controls_id,
            hide_titlebar_id: tray_hide_titlebar_id,
            show_titlebar_id: tray_show_titlebar_id,
            reset_database_id: tray_reset_database_id,
            check_updates_id: tray_check_updates_id,
        },
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
    RemoveWebView(NanoId),
    ExtractResult(ScrapedData),
    SaveSettings(AppSettings),
    CheckForUpdates,
    UpdateAvailable(UpdateInfo),
    UpdateProgress { downloaded: u64, total: u64 },
    UpdateError(String),
}

impl ApplicationHandler<UserEvent> for App {
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application suspended");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");

        let monitors = event_loop.available_monitors().collect::<Vec<_>>();
        info!("Monitors: {:?}", monitors);
        let mut config_information = vec![];
        for monitor in monitors {
            info!("Monitor: {:?}", monitor);
            let config_information = ConfigInformation {
                identifier: monitor.native_id().to_string(),
                physical_size: (
                    monitor.size().width as usize,
                    monitor.size().height as usize,
                ),
                scale_factor: monitor.scale_factor(),
            };
        }
        self.db.set_config_information(config_information);

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
                let widget_id = self.window_id_to_widget_id.remove(&window_id).unwrap();
                self.widget_id_to_window_id.remove(&widget_id);
                self.all_widgets.remove(&window_id);
                self.db.update_widget_open_state(widget_id, false);
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
                    size: size.into(),
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
                // info!("Cursor entered: {:?}", device_id);
            }
            WindowEvent::CursorLeft { device_id } => {
                // info!("Cursor left: {:?}", device_id);
            }
            WindowEvent::Moved(position) => {
                // info!("Window moved: {:?}", position);
                // TODO: Update widget position in the database
            }
            _ => {
                info!("Unhandledevent: {:?}", event);
            }
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        let size = self.current_size;
        match event {
            UserEvent::RemoveWebView(id) => {
                info!("Removing webview at index {}", id.0);
                self.remove_webview(id);
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
                    .with_title(widget_options.title.unwrap_or("".to_string()))
                    .with_bounds(if let Some(bounds) = widget_options.bounds {
                        bounds
                    } else {
                        WidgetBounds {
                            x: DEFAULT_WIDGET_X,
                            y: DEFAULT_WIDGET_Y,
                            width: DEFAULT_WIDGET_WIDTH,
                            height: DEFAULT_WIDGET_HEIGHT,
                        }
                    });
                let res: Result<(), ()> = {
                    info!("TODO: Inserting widget configuration: {:?}", widget_config);
                    Ok(())
                };
                info!("Inserted widget configuration: {:?}", res);
                if res.is_err() {
                    error!("Failed to insert widget configuration: {:?}", res);
                }
                self.create_widget(event_loop, widget_config.clone());
            }
            UserEvent::TrayIconEvent(trayevent) => {
                match trayevent {
                    tray_icon::TrayIconEvent::Move { id, position, rect } => {
                        // info!("Tray icon moved: {:?}", position);
                    }
                    tray_icon::TrayIconEvent::Click {
                        id,
                        position,
                        button,
                        button_state,
                        rect,
                    } => {
                        info!("Tray icon clicked: {:?}", position);
                    }
                    _ => {
                        // info!("Unhandled Tray icon event: {:?}", trayevent);
                    }
                }
            }
            UserEvent::MenuEvent(menu_event) => {
                info!("Menu event: {:?}", menu_event);
                let id = menu_event.id();
                info!("Menu id: {:?}", id);

                match id.0.as_str() {
                    val if val == self.menu_items.quit_id => {
                        info!("Quitting application");
                        event_loop.exit();
                    }
                    val if val == self.menu_items.show_controls_id => {
                        info!("Showing controls");
                        self.show_controls(event_loop);
                    }
                    val if val == self.menu_items.hide_titlebar_id => {
                        info!("Hiding titlebars");
                        self.hide_titlebars(event_loop);
                    }
                    val if val == self.menu_items.show_titlebar_id => {
                        info!("Showing titlebars");
                        self.show_titlebars(event_loop);
                    }
                    val if val == self.menu_items.reset_database_id => {
                        info!("Resetting database");
                        self.reset_database(event_loop);
                    }
                    val if val == self.menu_items.check_updates_id => {
                        info!("Checking for updates");
                        self.proxy.send_event(UserEvent::CheckForUpdates).unwrap();
                    }
                    _ => {
                        info!("No tray menu show controls id found");
                    }
                }
            }
            UserEvent::ModifierEvent(modifier) => {
                info!("Modifier event: {:?}", modifier);
                let widget_id = modifier.widget_id.clone();
                match modifier.modifier_type {
                    Modifier::Refresh {
                        modifier_id,
                        interval_sec,
                    } => {
                        info!("User event: Refreshing widget: {:?}", widget_id);
                        self.refresh_webview(widget_id, interval_sec);
                    }
                    Modifier::Scrape {
                        modifier_id,
                        selector,
                    } => {
                        info!("User event: Scraping widget: {:?}", widget_id);
                        self.scrape_webview(widget_id, selector);
                    }
                }
            }
            UserEvent::ApiAction(action) => {
                info!("Api action received");
                match action {
                    ApiAction::CreateWidget(widget_request) => {
                        info!("Creating widget: {:?}", widget_request.title);

                        self.create_widget(event_loop, widget_request);
                    }
                    ApiAction::DeleteWidget(widget_id) => {
                        info!("Deleting widget: {:?}", widget_id);
                        self.remove_webview(NanoId(widget_id));
                    }
                    ApiAction::ToggleWidgetVisibility { widget_id, visible } => {
                        info!("Toggling widget visibility: {:?}", widget_id);
                        self.toggle_visibility(event_loop, widget_id, visible);
                    }
                    ApiAction::UpdateWidgetBounds { widget_id, bounds } => {
                        info!("Updating widget bounds: {:?}", widget_id);
                        self.update_widget_bounds(widget_id, bounds);
                    }
                    ApiAction::MaximizeWidget { widget_id } => {
                        info!("Maximizing widget: {:?}", widget_id);
                        self.maximize_webview(NanoId(widget_id));
                    }
                    ApiAction::MinimizeWidget { widget_id } => {
                        info!("Minimizing widget: {:?}", widget_id);
                        self.minimize_webview(NanoId(widget_id));
                    }
                    ApiAction::DeleteWidgetModifier {
                        widget_id,
                        modifier_id,
                    } => {
                        info!("Deleting widget modifier: {:?}", modifier_id);
                        // self.remove_widget_modifier(widget_id, modifier_id);
                        self.db.delete_widget_modifier(modifier_id.as_str());
                    }
                    ApiAction::CheckLicence {
                        user_email,
                        licence_key,
                    } => {
                        info!("Checking licence: {:?}, {:?}", user_email, licence_key);
                        let user_version =
                            check_user_version(&mut self.settings, &user_email, &licence_key);
                        info!("User version: {:?}", user_version);
                        if let Some(user_version) = user_version {
                            self.settings.app_settings.licence_tier = user_version;
                            self.settings.app_settings.licence_key = licence_key;
                            self.settings.app_settings.email = user_email;
                            self.update_app_settings(self.settings.app_settings.clone());

                            let mut all_messages = self.ui_state.messages.clone();
                            all_messages.push("Licence check successful".to_string());
                            self.db.set_app_ui_state(&AppUiState {
                                app_settings: self.settings.app_settings.clone(),
                                messages: all_messages,
                            });
                        }
                    }
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
                    IpcEvent::DragEvent(drag_event) => self.handle_drag_event(drag_event),
                    IpcEvent::BuyLicence(email) => {
                        info!("Buying licence for: {:?}", email);
                        let checkout_url = self.get_checkout_session_url(email.email);
                        match checkout_url {
                            Ok(checkout_url) => {
                                open::that(checkout_url).unwrap();
                            }
                            Err(e) => {
                                let mut app_ui_state = self.db.get_app_ui_state().unwrap().clone();
                                app_ui_state
                                    .messages
                                    .push(format!("Failed to get checkout session url: {:?}", e));
                                self.db.set_app_ui_state(&app_ui_state).unwrap();
                                error!("Failed to get checkout session url: {:?}", e);
                            }
                        }
                    }
                    IpcEvent::CheckLicence(check_licence_request) => {
                        info!("Checking licence: {:?}", check_licence_request);
                        let user_version = check_user_version(
                            &mut self.settings,
                            &check_licence_request.email,
                            &check_licence_request.licence_key,
                        );
                        info!("User version: {:?}", user_version);
                        if let Some(user_version) = user_version {
                            self.settings.app_settings.licence_tier = user_version;
                            self.settings.app_settings.licence_key =
                                check_licence_request.licence_key;
                            self.settings.app_settings.email = check_licence_request.email;
                            self.update_app_settings(self.settings.app_settings.clone());

                            let mut all_messages = self.ui_state.messages.clone();
                            all_messages.push("Licence check successful".to_string());
                            self.db.set_app_ui_state(&AppUiState {
                                app_settings: self.settings.app_settings.clone(),
                                messages: all_messages,
                            });
                        } else {
                            let mut app_ui_state = self.db.get_app_ui_state().unwrap().clone();
                            app_ui_state
                                .messages
                                .push("Licence check failed".to_string());
                            self.db.set_app_ui_state(&app_ui_state).unwrap();
                        }
                    }
                }
            }
            UserEvent::ExtractResult(scraped_data) => {
                // should not happen?
            }
            UserEvent::SaveSettings(app_settings) => {
                // should not happen?
            }
            UserEvent::CheckForUpdates => {
                let proxy = self.proxy.clone();
                let updater = self.updater.clone();
                let app_settings = self.settings.clone();
                // thread::spawn(move || {
                //     let rt = Runtime::new().unwrap();
                //     rt.block_on(async {
                match updater.check_for_updates(
                    // &app_settings.app_settings.licence_key,
                    // &app_settings.app_settings.machine_id,
                    // &app_settings.app_settings.email,
                    true,
                ) {
                    Ok(_) => {
                        info!("Update completed");
                        // Show update notification to user
                        // proxy
                        //     .send_event(UserEvent::UpdateAvailable(update_info))
                        //     .unwrap();
                    }
                    Err(e) => {
                        error!("Failed to check for updates: {}", e);
                        // Show "up to date" notification
                        proxy
                            .send_event(UserEvent::UpdateError("Already up to date".to_string()))
                            .unwrap();
                    }
                }
                //     });
                // });
            }
            UserEvent::UpdateAvailable(update_info) => {
                let proxy = self.proxy.clone();
                let updater = self.updater.clone();
                let quit_id = self.menu_items.quit_id.clone();

                // Show update confirmation dialog
                let update_confirmed = true; // TODO: Implement proper confirmation dialog
                if !update_confirmed {
                    return;
                }
            }
            UserEvent::UpdateProgress { downloaded, total } => {
                // TODO: Show progress in UI
                info!("Update progress: {}/{} bytes", downloaded, total);
            }
            UserEvent::UpdateError(error_msg) => {
                // TODO: Show error in UI
                error!("Update error: {}", error_msg);
            }
        }
    }
}

// pub fn set_app_dock_icon(_window: &Window) {
//     unsafe {
//         let data = NSData::dataWithBytes_length_(
//             nil,
//             DOCK_ICON.as_ptr() as *const std::os::raw::c_void,
//             DOCK_ICON.len() as u64,
//         );

//         let ns_image = NSImage::initWithDataIgnoringOrientation_(NSImage::alloc(nil), data);
//         NSApplication::setApplicationIconImage_(NSApp(), ns_image);
//     }
// }

fn setup_menu() -> Menu {
    let menu = Menu::new();

    // Add application menu (required for macOS)
    #[cfg(target_os = "macos")]
    {
        let app_menu = Submenu::new("App", true);
        let _ = app_menu.append_items(&[
            &PredefinedMenuItem::about(Some("HoverPane"), None),
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
            html: include_str!("../../react-ui/dist/index.html")
                .to_string()
                .replace("$PORT", &API_PORT.to_string()),
        }))
        .with_bounds(WidgetBounds {
            x: 0,
            y: 0,
            width: 500,
            height: 400,
        })
        .with_open(true)
        .with_level(Level::Normal)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    pub os: String,
    pub arch: String,
    pub machine_id: String,
}

fn get_machine_info() -> MachineInfo {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let machine_id = machine_uid::get().unwrap();
    let hashed_id = blake3::hash(machine_id.as_bytes());

    MachineInfo {
        machine_id: hashed_id.to_string(),
        os: os.to_string(),
        arch: arch.to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenceCheckResponse {
    user_id: String,
    licence_tier: LicenceTier,
    valid: bool,
}

fn check_user_version(
    app_settings: &mut DesktopAppSettings,
    email: &str,
    licence_key: &str,
) -> Option<LicenceTier> {
    let client = reqwest::blocking::Client::new();
    // get some information about the machine
    // let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
    // let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let version = std::env::var("VERSION").unwrap_or_else(|_| "unknown".to_string());
    let machine_id = machine_uid::get().unwrap();
    let hashed_id = blake3::hash(machine_id.as_bytes());

    let request = json!({
        "email": email,
        "licence_key": licence_key,
        "machine_id": hashed_id.to_string(),
        "os": os,
        "arch": arch,
        "version": version,
    });

    info!(
        "Sending licence check request to {}: {:?}",
        app_settings.licence_check_url, request
    );

    let res = client
        .post(&app_settings.licence_check_url)
        .json(&request)
        .send();

    if let Err(e) = res {
        error!("Failed to send request: {}", e);
        return None;
    }

    let body = res.unwrap().text().unwrap();
    info!("Response body: {}", body);
    let version_info: Value = match serde_json::from_str(&body) {
        Ok(version_info) => version_info,
        Err(e) => {
            error!("Failed to parse response: {}", e);
            return None;
        }
    };

    if version_info.get("error").is_some() {
        error!("Error from licence check: {}", version_info);
        return None;
    }

    let licence_check_response = match serde_json::from_value::<LicenceCheckResponse>(version_info)
    {
        Ok(licence_check_response) => licence_check_response,
        Err(e) => {
            error!("Failed to parse licence check response: {}", e);
            return None;
        }
    };

    info!("Response from licence check: {:?}", licence_check_response);
    Some(licence_check_response.licence_tier)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopAppSettings {
    pub app_settings: AppSettings,
    pub licence_check_url: String,
    pub api_base_url: String,
}

impl DesktopAppSettings {
    fn new(app_settings: AppSettings, licence_check_url: &str, api_base_url: &str) -> Self {
        Self {
            app_settings,
            licence_check_url: licence_check_url.to_string(),
            api_base_url: api_base_url.to_string(),
        }
    }
}

fn load_app_settings(
    db: &widget_db::Database,
    licence_check_url: &str,
    api_base_url: &str,
) -> DesktopAppSettings {
    let app_settings = match db.get_settings() {
        Ok(settings) => settings,
        Err(_) => {
            // Insert default settings if not present
            let default = AppSettings {
                show_tray_icon: true,
                email: "".to_string(),
                licence_key: "".to_string(),
                machine_id: machine_uid::get().unwrap(),
                licence_tier: LicenceTier::None,
            };
            let _ = db.set_settings(&default);
            default
        }
    };
    DesktopAppSettings::new(app_settings, licence_check_url, api_base_url)
}

fn setup_logging(config_dir: &Path) {
    // Configure the logger
    let mut logger = Builder::new();
    logger
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{}] - {}",
                jiff::Timestamp::now().to_string(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info);

    if dotenvy::var("ENV").unwrap_or("prod".to_string()) == "prod" {
        let log_file = config_dir.join("hoverpane.log");
        // Create the log file if it doesn't exist
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&log_file)
            .expect("Failed to create log file");
        logger.target(env_logger::Target::Pipe(Box::new(file)));
        info!("Logging to file: {:?}", log_file);
    } else {
        logger.target(env_logger::Target::Stdout);
        info!("Logging to stdout");
    }

    if let Err(e) = logger.try_init() {
        warn!("Logger already initialized: {}", e);
    }
}

fn main() {
    // env_logger::init();
    info!("Starting application...");
    dotenvy::from_filename(".env.dev").ok();

    // loading app settings from user directory
    let directory = directories::ProjectDirs::from("com", "jarde", "hoverpane").unwrap();
    let config_dir = directory.config_dir();

    // Setup logging before anything else
    setup_logging(&config_dir);

    let licence_check_url = if dotenvy::var("ENV").unwrap_or("prod".to_string()) == "prod" {
        "https://api.hoverpane.com/licence/check"
    } else {
        "http://localhost:3000/licence/check"
    };
    let updater_api_url = if dotenvy::var("ENV").unwrap_or("prod".to_string()) == "prod" {
        "https://api.hoverpane.com/apps/hoverpane/updates"
    } else {
        "http://localhost:3000/apps/hoverpane/updates"
    };

    let api_base_url = if dotenvy::var("ENV").unwrap_or("prod".to_string()) == "prod" {
        "https://api.hoverpane.com"
    } else {
        "http://localhost:3000"
    };
    let app_db = widget_db::Database::from(false).unwrap();
    let desktop_settings = load_app_settings(&app_db, licence_check_url, api_base_url);
    info!("App settings: {:?}", desktop_settings);

    // load db, run migrations, etc
    let app_db = widget_db::Database::from(false).unwrap();
    let mut builder = EventLoop::<UserEvent>::with_user_event();
    #[cfg(target_os = "macos")]
    {
        info!("Initializing Macos App...");
        winit::platform::macos::EventLoopBuilderExtMacOS::with_activation_policy(
            &mut builder,
            winit::platform::macos::ActivationPolicy::Accessory,
            // winit::platform::macos::ActivationPolicy::Regular,
        );

        winit::platform::macos::EventLoopBuilderExtMacOS::with_default_menu(&mut builder, false);
    }

    let event_loop = builder
        // .with_default_menu(false)
        .build()
        .expect("Something failed");
    let event_loop_proxy = event_loop.create_proxy();
    let menu = setup_menu();
    // menu.init_for_nsapp();

    let config: Vec<WidgetConfiguration> = vec![get_controls_widget_config()];

    let modifiers: Vec<WidgetModifier> = vec![];

    info!("Debug Config: {:?}", config.len());

    let proxy_clone = event_loop_proxy.clone();
    let proxy_clone_muda = event_loop_proxy.clone();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        info!("Menu event: {:?}", event);
        proxy_clone_muda.send_event(UserEvent::MenuEvent(event));
    }));

    let (menu_items, tray_icon) = setup_tray_menu(
        desktop_settings.app_settings.licence_tier.clone(),
        // &theme,
        event_loop_proxy.clone(),
    );
    tray_icon.set_visible(desktop_settings.app_settings.show_tray_icon);

    let mut app = App {
        ui_state: AppUiState {
            app_settings: desktop_settings.app_settings.clone(),
            messages: vec![],
        },
        updater: Updater::new(env!("CARGO_PKG_VERSION"), &updater_api_url),
        tray_icon,
        // theme,
        db: app_db,
        settings: desktop_settings,
        menu_items,
        current_size: LogicalSize::new(DEFAULT_WIDGET_WIDTH, DEFAULT_WIDGET_HEIGHT),
        current_modifiers: Modifiers::default(),
        all_widgets: HashMap::new(),
        widget_id_to_window_id: HashMap::new(),
        window_id_to_widget_id: HashMap::new(),
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
            let mut api_db = widget_db::Database::from(false).unwrap();
            // put the new controls widget into the db
            let res = api_db.upsert_widget_configuration(config[0].clone());
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
        let modifier_db_access = widget_db::Database::from(false).unwrap();
        let mut last_refresh_dict = HashMap::new();
        let mut last_scrape_dict = HashMap::new();

        loop {
            let widget_modifiers = modifier_db_access.get_all_widget_modifiers();
            let modifiers = match widget_modifiers {
                Ok(modifiers) => modifiers,
                Err(e) => {
                    error!("Error getting widget modifiers: {:?}", e);
                    vec![]
                }
            };

            // let Ok(widget_modifiers) = widget_modifiers else {
            //     error!(
            //         "Error getting widget modifiers: {:?}",
            //         widget_modifiers.err().unwrap()
            //     );
            //     panic!("Error getting widget modifiers");
            // };
            info!("Found {} widget modifiers", modifiers.len());
            for modifier in modifiers {
                // info!("Modifier: {:?}", modifier);
                let modifier_clone = modifier.clone();
                let widget_id = modifier.widget_id.clone();
                match modifier.modifier_type {
                    Modifier::Refresh {
                        modifier_id,
                        interval_sec,
                    } => {
                        // info!("Refresh modifier: {:?}", modifier_id);
                        let last_update = last_refresh_dict
                            .entry(widget_id.clone())
                            .or_insert(Instant::now());

                        if last_update.elapsed().as_secs() >= interval_sec as u64 {
                            info!("Refreshing widget: {:?}", widget_id);
                            let _ = modifier_thread_proxy
                                .send_event(UserEvent::ModifierEvent(modifier_clone));
                            last_refresh_dict.insert(widget_id.clone(), Instant::now());
                        }
                    }
                    Modifier::Scrape {
                        modifier_id,
                        selector,
                    } => {
                        info!(
                            "Scrape modifier: {:?}, selector: {:?}",
                            modifier_id, selector
                        );
                        let last_update = last_scrape_dict
                            .entry(widget_id.clone())
                            .or_insert(Instant::now());
                        if last_update.elapsed().as_secs() >= DEFAULT_SCRAPE_INTERVAL {
                            info!("Scraping widget: {:?}", widget_id);
                            modifier_thread_proxy
                                .send_event(UserEvent::ModifierEvent(modifier_clone));
                            last_scrape_dict.insert(widget_id.clone(), Instant::now());
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(10));
        }
    });
    event_loop.run_app(&mut app).expect("Something failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_machine_info() {
        let machineid = machine_uid::get().unwrap();
        println!("Machine ID: {:?}", machineid);
        let hashed_id = blake3::hash(machineid.as_bytes());
        println!("Hashed ID: {:?}", hashed_id);

        let machine_info = get_machine_info();
        println!("Machine info: {:?}", machine_info);
    }
}
