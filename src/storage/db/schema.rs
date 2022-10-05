// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Integer,
        title -> Text,
        details -> Text,
    }
}

diesel::table! {
    post_contents (id) {
        id -> Integer,
        post_id -> Integer,
        author_id -> Integer,
        title -> Text,
        body -> Text,
        created -> Integer,
        is_published -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        author_id -> Integer,
        thread_id -> Integer,
        created -> Integer,
    }
}

diesel::table! {
    threads (id) {
        id -> Integer,
        board_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        pass_sha -> Text,
        salt -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    post_contents,
    posts,
    threads,
    users,
);
