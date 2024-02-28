use sqlx::Row;

use crate::State;

#[tauri::command]
pub async fn get_tables(
    database_type: &str,
    conn_name: &str,
    schema: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>, String> {
    match database_type {
        "mysql" => {
            let mysql = state.mysql.lock().await;
            let pool = mysql.get(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            let pool = pool.unwrap();

            let rows = sqlx::query("SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = ?")
                .bind(schema)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            let tables = rows
                .into_iter()
                .map(|r| r.get("TABLE_NAME"))
                .collect::<Vec<String>>();

            return Ok(tables);
        }
        "pg" => {
            let pg = state.pg.lock().await;
            let pool = pg.get(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            let pool = pool.unwrap();

            let rows = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = $1")
                .bind(schema)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

            let tables = rows
                .into_iter()
                .map(|r| r.get("table_name"))
                .collect::<Vec<String>>();

            return Ok(tables);
        }
        _ => return Err("Invalid database_type".to_string()),
    }
}
