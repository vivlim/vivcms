
use diesel::prelude::*;
use crate::storage::db::models::Board;
use crate::storage::db::models::NewThread;
use crate::storage::db::models::User;

use crate::storage::db::models;
use crate::storage::db::schema;

use super::posts::create_post;


pub fn create_thread(conn: &SqliteConnection, board: &Board, author: &User, title: String, body: String) -> QueryResult<models::Thread> {
    let thread = NewThread {
        board_id: board.id
    };
    let thread = {
        use schema::threads::dsl::*;
        diesel::insert_into(schema::threads::table)
            .values(&thread)
            .execute(conn)?;
        threads.order(id.desc())
            .first::<models::Thread>(conn)?
    };
    let post = create_post(conn, &thread, author, title, body)?;
    Ok(thread)
}