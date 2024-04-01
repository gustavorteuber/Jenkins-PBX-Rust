use mysql::{Pool, Row, Error};
use crate::models::User;

pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        UserRepository { pool }
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        let mut conn = self.pool.get_conn()?;
        let row: Option<Row> = conn.first_exec("SELECT id, username, email FROM users WHERE id = :id", params!{"id" => id})?.next()?;
        
        if let Some(row) = row {
            let (id, username, email) = mysql::from_row(row);
            let user = User { id, username, email };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}
