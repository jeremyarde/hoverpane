-- Add migration script here
CREATE TABLE IF NOT EXISTS sites (
    id INTEGER PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS widgets (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL,
    title TEXT NOT NULL,
    widget_type TEXT NOT NULL,
    level TEXT NOT NULL,
    transparent INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS modifiers (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL,
    modifier_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS scraped_data (
    id INTEGER PRIMARY KEY,
    widget_id TEXT NOT NULL,
    value TEXT NOT NULL,
    error TEXT NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS elements (
    id INTEGER PRIMARY KEY,
    site_id INTEGER NOT NULL,
    selector TEXT NOT NULL,
    data_key TEXT NOT NULL
); 