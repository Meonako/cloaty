use crate::State;

#[tauri::command]
pub async fn disconnect_db(
    database_type: &str,
    conn_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    match database_type {
        "mysql" => {
            let mut mysql = state.mysql.lock().await;
            let pool = mysql.remove(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            pool.unwrap().close().await;
        }
        "pg" => {
            let mut pg = state.pg.lock().await;
            let pool = pg.remove(conn_name);

            if pool.is_none() {
                return Err("Not connected".to_string());
            }

            pool.unwrap().close().await;
        }
        _ => return Err("Invalid database_type".to_string()),
    }

    Ok(())
}
