use sqlx::Row;

use crate::State;
use super::Info;

#[tauri::command]
pub async fn connect_db(
    database_type: &str,
    conn_name: &str,
    info: Info,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>, String> {
    match database_type {
        "mysql" => {
            let mut mysql = state.mysql.lock().await;
            if mysql.contains_key(conn_name) {
                return Err("Already connected".to_string());
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

            let pool = sqlx::mysql::MySqlPoolOptions::new()
                .max_connections(5)
                .connect_with(conn_info)
                .await
                .map_err(|e| e.to_string())?;

            mysql.insert(conn_name.to_string(), pool);

            let pool = mysql.get(conn_name).unwrap();

            let rows = sqlx::query("SHOW schemas")
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(rows.into_iter().map(|r| r.get_unchecked("Database")).collect());
        }
        "pg" => {
            let mut pg = state.pg.lock().await;
            if pg.contains_key(conn_name) {
                return Err("Already connected".to_string());
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

            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect_with(conn_info)
                .await
                .map_err(|e| e.to_string())?;

            pg.insert(conn_name.to_string(), pool);

            let pool = pg.get(conn_name).unwrap();

            let rows = sqlx::query("SELECT schema_name FROM information_schema.schemata ORDER BY schema_name ASC")
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            // let rows = sqlx::query("SELECT table_schema FROM information_schema.tables")
            //     .fetch_all(pool)
            //     .await
            //     .map_err(|e| e.to_string())?;


            return Ok(rows.into_iter().map(|r| r.get_unchecked("schema_name")).collect());
        }
        _ => return Err("Invalid database_type".to_string()),
    }
}
