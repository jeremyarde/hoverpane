pub mod api {
    use nanoid::nanoid_gen;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower_http::trace::TraceLayer;
    use widget_types::{
        ApiAction, EventSender, MonitorPosition, WidgetBounds, API_PORT, DEFAULT_WIDGET_HEIGHT,
        DEFAULT_WIDGET_WIDTH, DEFAULT_WIDGET_X, DEFAULT_WIDGET_Y,
    };
    use widget_types::{
        CreateWidgetRequest, FileConfiguration, Modifier, UrlConfiguration, WidgetConfiguration,
        WidgetModifier, WidgetType,
    };

    // use crate::db::db::ScrapedData;
    // use crate::Modifier;
    // use crate::NanoId;
    // use crate::WidgetModifier;
    use axum::extract::rejection::JsonRejection;
    use axum::routing::delete;
    use axum::routing::get;
    use axum::routing::post;

    use axum::extract::Path;
    use axum::Router;
    use log::info;
    use log::{debug, error};
    use tower_http::cors::AllowOrigin;
    use tower_http::cors::CorsLayer;

    use axum::http::StatusCode;

    use axum::response::IntoResponse;

    use axum::extract::State;

    use axum::middleware::Next;
    use axum::response::Response;
    use http::Request;
    use std::time::Instant;

    pub async fn run_api(db: Arc<Mutex<crate::db::db::Database>>, event_sender: EventSender) {
        info!("Starting API");
        let state = ApiState {
            db: db.clone(),
            event_sender: event_sender.clone(),
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
            // .route("/sites", get(get_sites))
            // .route("/elements", get(get_elements))
            // .route("/latest", get(get_latest_values))
            .route(
                "/widgets/{widget_id}",
                delete(delete_widget).post(widget_rpc_handler),
            )
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
            .layer(TraceLayer::new_for_http())
            .layer(cors_layer)
            // .layer(axum::middleware::from_fn(logging_middleware))
            .with_state(state);

        let addr = format!("{}:{}", "127.0.0.1", API_PORT);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        info!("API listening on http://{}", addr);
        axum::serve(listener, router).await.unwrap();
    }

    use thiserror::Error;

    use crate::deserializer::deserializer::Json;

    #[derive(Debug, Error)]
    pub enum ApiError {
        #[error("Database error: {0}")]
        Database(#[from] rusqlite::Error),

        #[error("Event sender error: {0}")]
        EventSender(String),

        #[error("Widget not found: {0}")]
        WidgetNotFound(String),

        #[error("Modifier not found: {0}")]
        ModifierNotFound(String),

        #[error("Invalid request: {0}")]
        InvalidRequest(String),
    }

    // We implement `IntoResponse` so ApiError can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let (status, message) = match self {
                ApiError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                ApiError::EventSender(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
                ApiError::WidgetNotFound(e) => (StatusCode::NOT_FOUND, e),
                ApiError::ModifierNotFound(e) => (StatusCode::NOT_FOUND, e),
                ApiError::InvalidRequest(e) => (StatusCode::BAD_REQUEST, e),
            };

            let payload = json!({
                "error": message,
                "type": std::any::type_name::<Self>()
            });

            (status, Json(payload)).into_response()
        }
    }

    #[axum::debug_handler]
    pub(crate) async fn delete_widget(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
    ) -> Result<StatusCode, ApiError> {
        info!("Deleting widget {}", widget_id);

        if state
            .event_sender
            .send_message(ApiAction::DeleteWidget(widget_id.clone()))
            .is_err()
        {
            return Err(ApiError::EventSender(
                "Failed to send delete widget event".into(),
            ));
        }

        let mut db = state.db.lock().await;
        db.delete_widget(&widget_id)?;

        Ok(StatusCode::NO_CONTENT)
    }

    #[axum::debug_handler]
    pub(crate) async fn widget_rpc_handler(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
        Json(rpc_request): Json<ApiAction>,
    ) -> Result<StatusCode, ApiError> {
        info!("Widget RPC handler called for widget {}", widget_id);

        if state.event_sender.send_message(rpc_request).is_err() {
            return Err(ApiError::EventSender("Failed to send RPC event".into()));
        }

        Ok(StatusCode::OK)
    }

    #[axum::debug_handler]
    pub(crate) async fn create_widget(
        State(state): State<ApiState>,
        Json(widget_request): Json<CreateWidgetRequest>,
    ) -> Result<(StatusCode, Json<Value>), ApiError> {
        info!("Creating widget: {:?}", widget_request.title);
        debug!("Creating widget: {:?}", widget_request);

        let title = if widget_request.title.is_some() {
            widget_request.title.unwrap()
        } else {
            widget_request
                .url
                .clone()
                .unwrap()
                .split("/")
                .last()
                .unwrap_or("")
                .to_string()
        };

        let widget_config: WidgetConfiguration = WidgetConfiguration {
            id: 0,
            widget_id: widget_types::NanoId(nanoid_gen(8)),
            title,
            widget_type: if widget_request.html.is_some()
                && !widget_request.html.clone().unwrap().is_empty()
            {
                info!("Creating file widget");
                WidgetType::File(FileConfiguration {
                    html: widget_request.html.unwrap(),
                })
            } else if widget_request.url.is_some() {
                info!("Creating url widget");
                WidgetType::Url(UrlConfiguration {
                    url: widget_request.url.unwrap(),
                })
            } else {
                return Err(ApiError::InvalidRequest(
                    "Either html or url must be provided".into(),
                ));
            },
            level: widget_request.level,
            transparent: widget_request.transparent,
            decorations: widget_request.decorations,
            is_open: true,
            bounds: widget_request.bounds.unwrap_or(WidgetBounds {
                x: DEFAULT_WIDGET_X,
                y: DEFAULT_WIDGET_Y,
                width: DEFAULT_WIDGET_WIDTH,
                height: DEFAULT_WIDGET_HEIGHT,
            }),
        };

        if state
            .event_sender
            .send_message(ApiAction::CreateWidget(widget_config.clone()))
            .is_err()
        {
            return Err(ApiError::EventSender(
                "Failed to send create widget event".into(),
            ));
        }

        let mut db = state.db.lock().await;
        db.insert_widget_configuration(vec![widget_config.clone()])?;

        if !widget_request.modifiers.is_empty() {
            db.insert_widget_modifiers(
                widget_request
                    .modifiers
                    .iter()
                    .map(|m| WidgetModifier {
                        id: 0,
                        widget_id: widget_config.widget_id.clone(),
                        modifier_type: m.clone(),
                    })
                    .collect(),
            )?;
        }

        Ok((StatusCode::CREATED, Json(json!(widget_config))))
    }

    #[axum::debug_handler]
    pub(crate) async fn get_values(State(state): State<ApiState>) -> impl IntoResponse {
        let db = state.db.lock().await;
        match db.get_data() {
            Ok(values) => Json(values).into_response(),
            Err(e) => {
                error!("Failed to get values: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    #[axum::debug_handler]
    pub(crate) async fn get_latest_values(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
    ) -> impl IntoResponse {
        info!("Getting latest values for widget {}", widget_id);
        let db = state.db.lock().await;
        match db.get_latest_data() {
            Ok(values) => Json(values).into_response(),
            Err(e) => {
                error!("Failed to get latest values: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    // pub(crate) async fn get_sites(State(state): State<ApiState>) -> impl IntoResponse {
    //     // let state = state.db.try_lock().unwrap();
    //     match state.db.try_lock().unwrap().get_sites().await {
    //         Ok(sites) => Json(sites).into_response(),
    //         Err(e) => {
    //             error!("Failed to get sites: {:?}", e);
    //             StatusCode::INTERNAL_SERVER_ERROR.into_response()
    //         }
    //     }
    // }

    // pub(crate) async fn get_elements(State(state): State<ApiState>) -> impl IntoResponse {
    //     let state = state.db.try_lock().unwrap();
    //     match state.get_elements().await {
    //         Ok(elements) => Json(elements).into_response(),
    //         Err(e) => {
    //             error!("Failed to get elements: {:?}", e);
    //             StatusCode::INTERNAL_SERVER_ERROR.into_response()
    //         }
    //     }
    // }

    #[axum::debug_handler]
    pub(crate) async fn get_widgets(
        State(state): State<ApiState>,
    ) -> Result<Json<Vec<WidgetConfiguration>>, ApiError> {
        info!("get widgets called");
        let db = state.db.lock().await;
        let widgets = db.get_configuration()?;
        info!("# widgets: {:?}", widgets.len());
        Ok(Json(widgets))
    }

    #[axum::debug_handler]
    pub(crate) async fn get_widget_modifiers(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
    ) -> Result<Json<Vec<WidgetModifier>>, ApiError> {
        info!("Getting modifiers for widget {}", widget_id);

        let db = state.db.lock().await;
        let modifiers = db.get_widget_modifier(&widget_id)?;
        Ok(Json(modifiers))
    }

    #[axum::debug_handler]
    pub(crate) async fn delete_widget_modifier(
        State(state): State<ApiState>,
        Path((widget_id, modifier_id)): Path<(String, String)>,
    ) -> Result<StatusCode, ApiError> {
        info!(
            "Deleting modifier {} from widget {}",
            modifier_id, widget_id
        );

        if state
            .event_sender
            .send_message(ApiAction::DeleteWidgetModifier {
                widget_id,
                modifier_id: modifier_id.clone(),
            })
            .is_err()
        {
            return Err(ApiError::EventSender(
                "Failed to send delete widget event".into(),
            ));
        }

        let mut db = state.db.lock().await;
        db.delete_widget_modifier(&modifier_id)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[derive(Clone)]
    pub(crate) struct ApiState {
        pub db: Arc<Mutex<crate::db::db::Database>>,
        pub event_sender: EventSender,
    }

    #[axum::debug_handler]
    pub async fn add_widget_modifier(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
        Json(modifier): Json<WidgetModifier>,
    ) -> Result<(StatusCode, Json<WidgetModifier>), ApiError> {
        info!("Adding modifier to widget {}: {:?}", widget_id, modifier);

        let widget_modifier = WidgetModifier {
            id: 0,
            widget_id: widget_types::NanoId(widget_id),
            modifier_type: match modifier.modifier_type {
                Modifier::Scrape {
                    modifier_id,
                    selector,
                } => Modifier::Scrape {
                    modifier_id,
                    selector,
                },
                Modifier::Refresh {
                    modifier_id,
                    interval_sec,
                } => Modifier::Refresh {
                    modifier_id,
                    interval_sec,
                },
            },
        };

        let mut db = state.db.lock().await;
        db.insert_widget_modifier(widget_modifier.clone())?;
        Ok((StatusCode::CREATED, Json(widget_modifier)))
    }
}
