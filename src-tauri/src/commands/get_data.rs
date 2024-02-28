use sqlx::Row;
use std::collections::HashMap;

use crate::State;

#[tauri::command]
pub async fn get_data(
    database_type: &str,
    conn_name: &str,
    schema: &str,
    table: &str,
    state: tauri::State<'_, State>,
) -> Result<(Vec<String>, Vec<HashMap<String, serde_json::Value>>), String> {
    match database_type {
        "mysql" => {
            let mysql = state.mysql.lock().await;
            let pool = mysql.get(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            let pool = pool.unwrap();

            let pool_clone = pool.clone();
            let schma_clone = schema.to_string();
            let table_clone = table.to_string();
            let thread = tokio::spawn(async move {
                let rows = sqlx::query("SELECT column_name FROM information_schema.columns WHERE table_schema = ? AND table_name = ?")
                    .bind(schma_clone)
                    .bind(table_clone)
                    .fetch_all(&pool_clone)
                    .await
                    .unwrap();

                    rows.into_iter().map(|r| r.get_unchecked("column_name")).collect()
            });

            let rows = sqlx::query(&format!("SELECT * FROM `{schema}`.`{table}`"))
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                let outputs = sqlx_to_json::mysql::rows_to_json(rows)?;

            return Ok((thread.await.map_err(|e| e.to_string())?, outputs));
        }
        "pg" => {
            let pg = state.pg.lock().await;
            let pool = pg.get(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            let pool = pool.unwrap();

            let pool_clone = pool.clone();
            let schma_clone = schema.to_string();
            let table_clone = table.to_string();
            let thread = tokio::spawn(async move {
                let rows = sqlx::query("SELECT column_name FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2")
                    .bind(schma_clone)
                    .bind(table_clone)
                    .fetch_all(&pool_clone)
                    .await
                    .unwrap();

                rows.into_iter().map(|r| r.get_unchecked("column_name")).collect()
            });

            let rows = sqlx::query(&format!("SELECT * FROM {schema}.{table}"))
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            let outputs = sqlx_to_json::postgres::rows_to_json(rows)?;

            return Ok((thread.await.map_err(|e| e.to_string())?, outputs));
        }
        _ => return Err("Invalid database_type".to_string()),
    }
}
