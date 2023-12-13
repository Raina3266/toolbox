use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::model::User;

pub struct Services {
    db_pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub async fn make_services() -> Services {
    let db = make_db().await;

    Services { db_pool: db }
}

async fn make_db() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=ganlinchen dbname=toolbox",
        NoTls,
    )
    .unwrap();
    Pool::builder().build(manager).await.unwrap()
}

//There will be a table in the database called 'user' with the following columns:
// - 'id INT PRIMARY KEY'
// -  'email TEXT NOT NULL'
// - 'prefers_dark_theme BOOL NOT NULL'
// - 'is_admin BOOL NOT NULL'
impl Services {
    pub async fn insert_user(&self, user: User) {
        let connection = self.db_pool.get().await.unwrap();

        connection
            .execute(
                "INSERT INTO users VALUES ($1, $2, $3, $4)",
                &[
                    &user.id,
                    &user.email,
                    &user.prefers_dark_theme,
                    &user.is_admin,
                ],
            )
            .await
            .unwrap();
    }
}
