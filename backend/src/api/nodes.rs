use axum::{response::IntoResponse, Json};

use crate::database::nodes;

struct LoadRequest {
    from_id: Id,
}
pub fn load(Json(req): Json<LoadRequest>) -> IntoResponse {
    Ok(())
}

