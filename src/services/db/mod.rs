use std::time::Duration;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::Utc;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::model::User;

pub struct Db {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Db {
    pub async fn connect() -> Self {
        let manager = PostgresConnectionManager::new_from_stringlike(
            "host=localhost user=ganlinchen dbname=toolbox",
            NoTls,
        )
        .unwrap();
        let pool = Pool::builder().build(manager).await.unwrap();

        Self { pool }
    }

    pub async fn insert_user(&self, user: User) {
        let connection_1 = self.pool.get().await.unwrap();

        connection_1
            .execute(
                "INSERT INTO users VALUES ($1, $2, $3)",
                &[&user.id, &user.email, &user.password],
            )
            .await
            .unwrap();
    }

    pub async fn create_session(&self, userid: Uuid, ttl: Duration) -> Uuid {
        let connection_3 = self.pool.get().await.unwrap();
        let token = Uuid::new_v4();
        let valid_until = Utc::now() + ttl;

        connection_3
            .execute(
                "INSERT INTO sessions VALUES ($1, $2, $3)",
                &[&userid, &token, &valid_until],
            )
            .await
            .unwrap();

        token
    }

    pub async fn find_user_by_email(&self, email: String) -> Option<User> {
        let connection_2 = self.pool.get().await.unwrap();

        let option_row = connection_2
            .query_opt("SELECT * FROM users WHERE email = $1", &[&email])
            .await
            .unwrap();

        match option_row {
            None => None,
            Some(row) => {
                let id = row.get("id");
                let email = row.get("email");
                let password = row.get("password");

                Some(User {
                    id,
                    email,
                    password,
                })
            }
        }
    }
}
