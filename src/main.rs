mod db;
mod repository;

use tokio::runtime::Runtime;
use crate::db::establish_connection;
use crate::repository::UserRepository;

fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let pool = establish_connection().await.expect("Failed to connect to database");
        let user_repo = UserRepository::new(pool);

        let user = user_repo.get_user_by_id(1).await.expect("Failed to get user");
        println!("{:?}", user);
    });
}
