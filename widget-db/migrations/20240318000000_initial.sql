CREATE TABLE IF NOT EXISTS widgets (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    widget_type TEXT NOT NULL,
    level TEXT NOT NULL,
    transparent INTEGER NOT NULL,
    decorations INTEGER NOT NULL,
    is_open INTEGER NOT NULL,
    bounds TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS modifiers (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL,
    modifier_type TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS scraped_data (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL,
    value TEXT NOT NULL,
    error TEXT NOT NULL,
    timestamp TEXT NOT NULL
);


CREATE TABLE IF NOT EXISTS config (
    id INTEGER PRIMARY KEY,
    json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_ui_state (
    json TEXT NOT NULL
);
