#![allow(warnings)]

use nanoid::nanoid_gen;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod event;
pub use event::EventSender;
pub use event::EventSenderImpl;

pub const API_PORT: u16 = 3111;
pub const DEFAULT_WIDGET_WIDTH: u32 = 200;
pub const DEFAULT_WIDGET_HEIGHT: u32 = 200;
pub const DEFAULT_WIDGET_X: u32 = 0;
pub const DEFAULT_WIDGET_Y: u32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub licence_key: String,
    pub user_version: LicenceTier,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct ApiError {
    pub message: String,
    pub origin: String,
}

#[derive(Debug, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum IpcEvent {
    SaveSettings(AppSettings),
    ExtractResult(ScrapedData),
    DragEvent(DragEvent),
    BuyLicence(UserEmail),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct UserEmail {
    pub user_email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct CreateCheckoutSessionResponse {
    pub checkout_session_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct DragEvent {
    pub widget_id: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub enum LicenceTier {
    Pro,
    Free,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct AppUiState {
    pub app_settings: AppSettings,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct AppSettings {
    pub show_tray_icon: bool,
    pub user_email: String,
    pub licence_key: String,
    pub machine_id: String,
    pub licence_tier: LicenceTier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigInformation {
    pub identifier: String,
    pub physical_size: (usize, usize),
    pub scale_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[typeshare]
pub struct NanoId(pub String);

#[typeshare]
pub struct Widget {
    pub id: String,
    pub name: String,
    pub description: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ScrapedValue {
//     // pub id: i32,
//     pub widget_id: NanoId,
//     pub value: Option<String>,
//     pub error: Option<String>,
//     pub timestamp: i64,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "content")]
#[typeshare]

pub enum ApiAction {
    DeleteWidget(String),
    CreateWidget(WidgetConfiguration),
    ToggleWidgetVisibility {
        widget_id: String,
        visible: bool,
    },
    UpdateWidgetBounds {
        widget_id: String,
        bounds: WidgetBounds,
    },
    MaximizeWidget {
        widget_id: String,
    },
    MinimizeWidget {
        widget_id: String,
    },
    DeleteWidgetModifier {
        widget_id: String,
        modifier_id: String,
    },
    CheckLicence {
        user_email: String,
        licence_key: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct WidgetBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

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
    Scrape {
        modifier_id: NanoId,
        selector: String,
    },
    Refresh {
        modifier_id: NanoId,
        interval_sec: i32,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct ScrapedData {
    #[serde(skip)]
    pub id: i64,
    pub widget_id: String,
    pub value: String,
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[typeshare]
pub struct WidgetModifier {
    // #[serde(skip)]
    pub id: i32,
    pub widget_id: NanoId,
    pub modifier_type: Modifier,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct UrlConfiguration {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct FileConfiguration {
    pub html: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct MonitorPosition {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct WidgetConfiguration {
    #[serde(skip)]
    pub id: i64,
    pub widget_id: NanoId,
    pub title: String,
    pub widget_type: WidgetType,
    pub level: Level,
    pub transparent: bool,
    pub decorations: bool,
    pub is_open: bool,
    pub bounds: WidgetBounds,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct CreateWidgetRequest {
    pub url: Option<String>,
    pub html: Option<String>,
    pub title: Option<String>,
    pub level: Level,
    pub transparent: bool,
    pub decorations: bool,
    pub modifiers: Vec<Modifier>,
    pub bounds: Option<WidgetBounds>,
}

impl WidgetConfiguration {
    pub fn new() -> Self {
        Self {
            id: 0,
            widget_id: NanoId(nanoid_gen(8)),
            title: "".to_string(),
            widget_type: WidgetType::Url(UrlConfiguration {
                url: "".to_string(),
            }),
            level: Level::Normal,
            transparent: false,
            decorations: false,
            is_open: false,
            bounds: WidgetBounds {
                x: DEFAULT_WIDGET_X,
                y: DEFAULT_WIDGET_Y,
                width: DEFAULT_WIDGET_WIDTH,
                height: DEFAULT_WIDGET_HEIGHT,
            },
        }
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
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

    pub fn with_widget_id(mut self, id: NanoId) -> Self {
        self.widget_id = id;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn with_bounds(mut self, bounds: WidgetBounds) -> Self {
        self.bounds = bounds;
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[typeshare]
pub enum Level {
    AlwaysOnTop,
    Normal,
    AlwaysOnBottom,
}
