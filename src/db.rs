use sqlx::{postgres::PgPoolOptions, FromRow};

#[derive(FromRow, Debug)]
pub struct UserInfo {
    pub name: String,
    pub email: String,
}

pub async fn get_data() -> Vec<UserInfo> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@localhost:5432/techfiesta24")
        .await
        .unwrap();

    let rows: Vec<UserInfo> = sqlx::query_as("SELECT name, email FROM workshop_cad")
        .fetch_all(&conn)
        .await
        .unwrap();

    conn.close().await;

    return rows;
}
