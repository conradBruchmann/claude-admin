use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde_json::json;

/// Custom JSON extractor that returns generic error messages without exposing internals.
pub struct AppJson<T>(pub T);

#[axum::async_trait]
impl<S, T> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppJsonRejection;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(value)) => Ok(AppJson(value)),
            Err(rejection) => Err(AppJsonRejection::from(rejection)),
        }
    }
}

pub struct AppJsonRejection {
    status: StatusCode,
    message: String,
}

impl From<JsonRejection> for AppJsonRejection {
    fn from(rejection: JsonRejection) -> Self {
        let (status, message) = match rejection {
            JsonRejection::MissingJsonContentType(_) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Content-Type must be application/json".to_string(),
            ),
            JsonRejection::JsonSyntaxError(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid JSON syntax in request body".to_string(),
            ),
            JsonRejection::JsonDataError(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid or missing fields in request body".to_string(),
            ),
            JsonRejection::BytesRejection(_) => (
                StatusCode::BAD_REQUEST,
                "Failed to read request body".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, "Invalid request body".to_string()),
        };

        AppJsonRejection { status, message }
    }
}

impl IntoResponse for AppJsonRejection {
    fn into_response(self) -> Response {
        let body = json!({ "error": self.message });
        (self.status, axum::Json(body)).into_response()
    }
}
