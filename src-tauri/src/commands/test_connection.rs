use sqlx::Connection;

use super::Info;
use crate::State;

#[tauri::command]
pub async fn test_connection(
    database_type: &str,
    conn_name: &str,
    info: Info,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    match database_type {
        "mysql" => {
            {
                let mysql = state.mysql.lock().await;
                if mysql.contains_key(conn_name) {
                    return Err("Already connected".to_string());
                }
            }

            let mut conn_info = sqlx::mysql::MySqlConnectOptions::new()
                .host(&info.host)
                .port(info.port)
                .database(&info.database);

            if let Some(username) = &info.user {
                conn_info = conn_info.username(username);
            }

            if let Some(password) = &info.password {
                conn_info = conn_info.password(password);
            }

            _ = sqlx::MySqlConnection::connect_with(&conn_info)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(());
        }
        "pg" => {
            {
                let pg = state.pg.lock().await;
                if pg.contains_key(conn_name) {
                    return Err("Already connected".to_string());
                }
            }

            let mut conn_info = sqlx::postgres::PgConnectOptions::new()
                .host(&info.host)
                .port(info.port)
                .database(&info.database);

            if let Some(username) = &info.user {
                conn_info = conn_info.username(username);
            }

            if let Some(password) = &info.password {
                conn_info = conn_info.password(password);
            }

            _ = sqlx::PgConnection::connect_with(&conn_info)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(());
        }
        _ => return Err("Invalid database_type".to_string()),
    }
}
