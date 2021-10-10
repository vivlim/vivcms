
use diesel::prelude::*;
use crate::storage::db::models::Board;
use crate::storage::db::models::NewThread;
use crate::storage::db::models::Thread;
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

pub fn get_threads_in_board(board: &models::Board, conn: &SqliteConnection) -> QueryResult<Vec<models::Thread>> {
    models::Thread::belonging_to(board).load::<models::Thread>(conn)
}

pub fn get_thread_posts(conn: &SqliteConnection, thread: &models::Thread) -> QueryResult<Vec<models::Post>> {
    use schema::posts::dsl::*;
    models::Post::belonging_to(thread).order(id.asc()).load::<models::Post>(conn)
}

pub fn get_thread_first_post(conn: &SqliteConnection, thread: &models::Thread) -> QueryResult<models::Post> {
    use schema::posts::dsl::*;
    models::Post::belonging_to(thread).order(id.asc()).first::<models::Post>(conn)
}

pub fn get_thread_by_id<'a>(conn: &SqliteConnection, thread_id: i32) -> QueryResult<models::Thread> {
    use schema::threads::dsl::*;
    threads.filter(id.eq(thread_id)).first::<models::Thread>(conn)
}