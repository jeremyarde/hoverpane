pub mod api {
    use nanoid::NanoId;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    // use tokio::sync::Mutex;
    use widget_types::CreateWidgetRequest;
    use widget_types::Modifier;
    use widget_types::WidgetModifier;

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

    use std::sync::Arc;
    use std::sync::Mutex;

    // use crate::WidgetConfiguration;

    use axum::extract::Path;

    use axum::http::StatusCode;

    // use crate::UserEvent;

    use axum::response::IntoResponse;

    // use crate::CreateWidgetRequest;

    use axum::extract::State;

    pub struct ApiClient {}

    pub async fn run_api(
        db: Arc<Mutex<crate::db::db::Database>>,
        // api_proxy: Arc<EventLoopProxy<UserEvent>>,
    ) {
        // let (event_sender, event_receiver) = mpsc::channel(100);
        let state = ApiState {
            db: db.clone(),
            // proxy: api_proxy.clone(),
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
        #[error(transparent)]
        JsonExtractorRejection(#[from] JsonRejection),
    }

    // We implement `IntoResponse` so ApiError can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let (status, message) = match self {
                ApiError::JsonExtractorRejection(json_rejection) => {
                    (json_rejection.status(), json_rejection.body_text())
                }
            };

            let payload = json!({
                "message": message,
                "origin": "with_rejection"
            });

            (status, Json(payload)).into_response()
        }
    }

    pub(crate) async fn create_widget(
        State(state): State<ApiState>,
        WithRejection(Json(widget_options), _): WithRejection<Json<CreateWidgetRequest>, ApiError>,
    ) -> impl IntoResponse {
        info!("Creating widget: {:?}", widget_options);

        // todo!("Need to send possibly send an event to the app..., because we may be getting a message from somewhere else instead of through the app");
        // let res = state
        //     .proxy
        //     .send_event(UserEvent::CreateWidget(widget_options.clone()));

        // info!("[DEBUG] Result of event: {:?}", res);

        // if res.is_err() {
        //     error!("Failed to send event to event loop");
        //     return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // }

        StatusCode::CREATED.into_response()
    }

    #[axum::debug_handler]
    pub(crate) async fn get_values(State(state): State<ApiState>) -> impl IntoResponse {
        let state = state.db.try_lock().unwrap();
        match state.get_data().await {
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
        let state = state.db.try_lock().unwrap();
        match state.get_latest_data().await {
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

    pub(crate) async fn get_widgets(State(state): State<ApiState>) -> impl IntoResponse {
        info!("get widgets called");
        let state = state.db.try_lock().unwrap();
        match state.get_configuration().await {
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

        let db = state.db.try_lock().unwrap();
        match db.get_widget_modifiers(widget_id.as_str()).await {
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

        let mut db = state.db.try_lock().unwrap();
        match db.delete_widget_modifier(&modifier_id).await {
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
        // pub proxy: Arc<EventLoopProxy<UserEvent>>,
        // pub event_sender: Sender<UserEvent>,
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
            widget_id: NanoId(widget_id),
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

        let mut db = state.db.try_lock().unwrap();
        match db.insert_widget_modifier(widget_modifier).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => {
                error!("Failed to add modifier: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
