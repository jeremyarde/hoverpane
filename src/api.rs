use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use crate::db::db::{Database, ScrapedData};
use crate::{MonitoredElement, MonitoredSite, WidgetConfiguration, WidgetModifier};
use log::error;
use crate::UserEvent;
use crate::CreateWidgetRequest;
use crate::EventLoopProxy;

pub struct ApiState {
    pub db: Arc<Mutex<Database>>,
    pub proxy: Arc<EventLoopProxy<UserEvent>>,
}

pub fn create_api_router(db: Arc<Mutex<Database>>) -> Router {
    let state = ApiState { db, proxy: Arc::new(EventLoopProxy::new()) };

    Router::new()
        .route("/values", get(get_values))
        .route("/sites", get(get_sites))
        .route("/elements", get(get_elements))
        .route("/widgets/{widget_id}/latest", get(get_latest_values))
        .route("/widgets", get(get_widgets).post(create_widget))
        .route(
            "/widgets/{widget_id}/modifiers",
            post(add_widget_modifier).get(get_widget_modifiers),
        )
        .route(
            "/widgets/{widget_id}/modifiers/{modifier_id}",
            delete(delete_widget_modifier),
        )
        .with_state(state)
}

#[axum::debug_handler]
async fn get_values(State(state): State<ApiState>) -> Result<Json<Vec<ScrapedData>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_data().await {
        Ok(values) => Ok(Json(values)),
        Err(e) => {
            error!("Failed to get values: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn get_sites(State(state): State<ApiState>) -> Result<Json<Vec<MonitoredSite>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_sites().await {
        Ok(sites) => Ok(Json(sites)),
        Err(e) => {
            error!("Failed to get sites: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn get_elements(State(state): State<ApiState>) -> Result<Json<Vec<MonitoredElement>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_elements().await {
        Ok(elements) => Ok(Json(elements)),
        Err(e) => {
            error!("Failed to get elements: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn get_latest_values(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
) -> Result<Json<Vec<ScrapedData>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_latest_data().await {
        Ok(values) => Ok(Json(values)),
        Err(e) => {
            error!("Failed to get latest values: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn get_widgets(State(state): State<ApiState>) -> Result<Json<Vec<WidgetConfiguration>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_configuration().await {
        Ok(widgets) => Ok(Json(widgets)),
        Err(e) => {
            error!("Failed to get widgets: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn create_widget(
    State(state): State<ApiState>,
    Json(config): Json<WidgetConfiguration>,
) -> Result<StatusCode, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.create_widget(&config).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Failed to create widget: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn get_widget_modifiers(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
) -> Result<Json<Vec<WidgetModifier>>, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.get_widget_modifiers(&widget_id).await {
        Ok(modifiers) => Ok(Json(modifiers)),
        Err(e) => {
            error!("Failed to get modifiers: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn add_widget_modifier(
    State(state): State<ApiState>,
    Path(widget_id): Path<String>,
    Json(modifier): Json<WidgetModifier>,
) -> Result<StatusCode, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.add_widget_modifier(&widget_id, &modifier).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Failed to add modifier: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn delete_widget_modifier(
    State(state): State<ApiState>,
    Path((widget_id, modifier_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    let db = state.db.try_lock().unwrap();
    match db.delete_widget_modifier(&widget_id, &modifier_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Failed to delete modifier: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
} 