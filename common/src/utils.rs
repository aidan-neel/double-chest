use axum::http::StatusCode;
use tonic::{Code, Status};

pub fn tonic_to_http(status: &Status) -> StatusCode {
    match status.code() {
        Code::Ok => StatusCode::OK,
        Code::Cancelled => StatusCode::REQUEST_TIMEOUT,
        Code::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        Code::InvalidArgument => StatusCode::BAD_REQUEST,
        Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::AlreadyExists => StatusCode::CONFLICT,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        Code::ResourceExhausted => StatusCode::TOO_MANY_REQUESTS,
        Code::FailedPrecondition => StatusCode::PRECONDITION_FAILED,
        Code::Aborted => StatusCode::CONFLICT,
        Code::OutOfRange => StatusCode::BAD_REQUEST,
        Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        Code::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        Code::DataLoss => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,
    }
}

pub fn tonic_to_http_response(status: tonic::Status) -> (StatusCode, String) {
    let code = tonic_to_http(&status);
    let msg = status.message().to_string();
    (code, msg)
}