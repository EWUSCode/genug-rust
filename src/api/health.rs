use axum::{extract::State, http::StatusCode};

use crate::{db::ConnectionPool, internal_error};

pub(crate) async fn health(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let sql = "SELECT 1 FROM DUAL";
    let mut stmt = conn.statement(sql).build().map_err(internal_error)?;
    let mut rows = stmt.query(&[]).map_err(internal_error)?;
    let row = rows
        .nth(0)
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No row".to_string()))?
        .map_err(internal_error)?;

    let c: i32 = row.get(0).map_err(internal_error)?;

    Ok(c.to_string())
}
