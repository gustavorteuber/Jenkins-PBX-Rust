use dotenv::dotenv;
use mysql::{Pool, PooledConn, Error};

pub async fn establish_connection() -> Result<Pool, Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = Pool::new(database_url)?;

    Ok(pool)
}
