pub mod api {
    use nanoid::nanoid_gen;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use widget_types::EventSender;
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
    use axum::Json;

    use axum::Router;
    use log::error;
    use log::info;
    use tower_http::cors::AllowOrigin;
    use tower_http::cors::CorsLayer;
    // use typeshare::typeshare;
    // use winit::event_loop::EventLoopProxy;

    use axum::extract::Path;

    use axum::http::StatusCode;

    // use crate::UserEvent;

    use axum::response::IntoResponse;

    // use crate::CreateWidgetRequest;

    use axum::extract::State;

    pub struct ApiClient {}

    pub async fn run_api(db: Arc<Mutex<crate::db::db::Database>>, event_sender: EventSender) {
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
    }

    use axum_extra::extract::WithRejection;
    use thiserror::Error;
    // We derive `thiserror::Error`
    #[derive(Debug, Error)]
    pub enum ApiError {
        // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
        // implementation. See `thiserror` docs for more information
        #[error("Database error: {0}")]
        Database(#[from] rusqlite::Error),
    }

    // We implement `IntoResponse` so ApiError can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let (status, message) = match self {
                ApiError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            };

            let payload = json!({
                "message": message,
                "origin": "with_rejection"
            });

            (status, Json(payload)).into_response()
        }
    }

    #[axum::debug_handler]
    pub(crate) async fn create_widget(
        State(state): State<ApiState>,
        Json(widget_request): Json<CreateWidgetRequest>,
    ) -> impl IntoResponse {
        info!("Creating widget: {:?}", widget_request);

        let widget_config = WidgetConfiguration {
            id: 0,
            widget_id: widget_types::NanoId(nanoid_gen(8)),
            title: widget_request.title,
            widget_type: if widget_request.url.is_empty() {
                WidgetType::File(FileConfiguration {
                    html: widget_request.html,
                })
            } else {
                WidgetType::Url(UrlConfiguration {
                    url: widget_request.url,
                })
            },
            level: widget_request.level,
            transparent: widget_request.transparent,
        };

        let mut db = state.db.lock().await;
        match db.insert_widget_configuration(vec![widget_config]) {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => {
                error!("Failed to create widget: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
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
    pub(crate) async fn get_widgets(State(state): State<ApiState>) -> impl IntoResponse {
        info!("get widgets called");
        let db = state.db.lock().await;
        match db.get_configuration() {
            Ok(widgets) => {
                info!("# widgets: {:?}", widgets.len());
                Json(widgets).into_response()
            }
            Err(e) => {
                error!("Failed to get widgets: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    #[axum::debug_handler]
    pub(crate) async fn get_widget_modifiers(
        State(state): State<ApiState>,
        Path(widget_id): Path<String>,
    ) -> impl IntoResponse {
        info!("Getting modifiers for widget {}", widget_id);

        let db = state.db.lock().await;
        match db.get_widget_modifiers(&widget_id) {
            Ok(modifiers) => Json(modifiers).into_response(),
            Err(e) => {
                error!("Failed to get modifiers: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    #[axum::debug_handler]
    pub(crate) async fn delete_widget_modifier(
        State(state): State<ApiState>,
        Path((widget_id, modifier_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "Deleting modifier {} from widget {}",
            modifier_id, widget_id
        );

        let db = state.db.lock().await;
        match db.delete_widget_modifier(&modifier_id) {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => {
                error!("Failed to delete modifier: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
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
    ) -> impl IntoResponse {
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
        match db.insert_widget_modifier(widget_modifier) {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => {
                error!("Failed to add modifier: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
