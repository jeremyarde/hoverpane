pub mod deserializer {
    use axum::{
        extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode,
        response::IntoResponse,
    };
    use log::error;
    use serde::Serialize;
    use serde_json::{json, Value};

    // We create our own rejection type
    #[derive(Debug)]
    pub struct ApiError {
        status: StatusCode,
        message: String,
        // path: String,
    }
    // create an extractor that internally uses `axum::Json` but has a custom rejection
    #[derive(FromRequest)]
    #[from_request(via(axum::Json), rejection(ApiError))]
    pub struct Json<T>(pub T);

    // We implement `IntoResponse` for our extractor so it can be used as a response
    impl<T: Serialize> IntoResponse for Json<T> {
        fn into_response(self) -> axum::response::Response {
            let Self(value) = self;
            axum::Json(value).into_response()
        }
    }

    // We implement `From<JsonRejection> for ApiError`
    impl From<JsonRejection> for ApiError {
        fn from(rejection: JsonRejection) -> Self {
            error!("Failed to deserialize JSON: {}", rejection.body_text());
            Self {
                status: rejection.status(),
                message: rejection.body_text(),
                // path: String::new(),
            }
        }
    }

    // We implement `IntoResponse` so `ApiError` can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let payload = json!({
                "message": self.message,
                "origin": "derive_from_request",
                // "path": self.path
            });

            (self.status, axum::Json(payload)).into_response()
        }
    }
}
