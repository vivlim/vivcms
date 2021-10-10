
use diesel::prelude::*;
use crate::storage::db::models::NewBoard;

use crate::storage::db::models;
use crate::storage::db::schema;


pub fn create_board(conn: &SqliteConnection, title: String, details: String) -> QueryResult<models::Board> {
    let new_board = NewBoard {
        title,
        details
    };
    {
        use schema::boards::dsl::*;
        diesel::insert_into(schema::boards::table)
            .values(&new_board)
            .execute(conn)?;
        boards.order(id.desc())
            .first::<models::Board>(conn)
    }
}