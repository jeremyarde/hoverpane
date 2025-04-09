use nanoid::nanoid_gen;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod event;
pub use event::ApiAction;
pub use event::EventSender;
pub use event::EventSenderImpl;

#[derive(Debug, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
pub enum IpcEvent {
    SaveSettings(AppSettings),
    ExtractResult(ScrapedData),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct AppSettings {
    pub show_tray_icon: bool,
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
pub struct WidgetConfiguration {
    #[serde(skip)]
    pub id: i64,
    pub widget_id: NanoId,
    pub title: String,
    pub widget_type: WidgetType,
    pub level: Level,
    pub transparent: bool,
    pub decorations: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[typeshare]
pub struct CreateWidgetRequest {
    pub url: String,
    pub html: String,
    pub title: String,
    pub level: Level,
    pub transparent: bool,
    pub decorations: bool,
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
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[typeshare]
pub enum Level {
    AlwaysOnTop,
    Normal,
    AlwaysOnBottom,
}
