
use diesel::prelude::*;
use crate::storage::db::models::NewBoard;

use crate::storage::db::models;
use crate::storage::db::schema;

// TODO for boards
// add board ordering
// get last post in board
// get # threads and messages in board?

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

pub fn get_boards(conn: &SqliteConnection) -> QueryResult<Vec<models::Board>> {
    use schema::boards::dsl::*;
    boards.order(id.asc())
        .load::<models::Board>(conn)
}

pub fn get_board_by_id<'a>(conn: &SqliteConnection, board_id: i32) -> QueryResult<models::Board> {
    use schema::boards::dsl::*;
    boards.filter(id.eq(board_id)).first::<models::Board>(conn)
}