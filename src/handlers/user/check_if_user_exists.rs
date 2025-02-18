use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use diesel::{dsl::exists, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

use crate::{
    dto::{
        requests::user::check_if_user_exists_request::CheckIfUserExistsRequest,
        responses::response_data::http_resp,
    },
    errors::code_error::{code_err, CodeError, HandlerResponse},
    init::state::ServerState,
    schema::users,
    util::time::now::tokio_now,
};

// TODO: Move
#[derive(serde_derive::Serialize)]
struct CheckIfUserExistsRespose {
    email_exists: bool,
}

pub async fn check_if_user_exists_handler(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<CheckIfUserExistsRequest>,
) -> HandlerResponse<impl IntoResponse> {
    let start = tokio_now();

    if !email_address::EmailAddress::is_valid(&request.user_email) {
        return Err(CodeError::EMAIL_INVALID.into());
    }

    let mut conn = state
        .get_conn()
        .await
        .map_err(|e| code_err(CodeError::POOL_ERROR, e))?;

    #[rustfmt::skip]
    let email_exists: bool = diesel::select(
        exists(
            users::table.filter(users::user_email.eq(&request.user_email)),
        ))
        .get_result(&mut conn)
        .await
        .map_err(|e| code_err(CodeError::DB_QUERY_ERROR, e))?;

    drop(conn);

    Ok(http_resp(
        CheckIfUserExistsRespose { email_exists },
        (),
        start,
    ))
}
