use self::db::Db;

mod db;

pub struct Services {
    pub db: Db
}

pub async fn make_services() -> Services {
    let db = Db::connect().await;

    Services { db }
}
