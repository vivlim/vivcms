table! {
    boards (id) {
        id -> Integer,
        title -> Text,
        details -> Text,
    }
}

table! {
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

table! {
    posts (id) {
        id -> Integer,
        author_id -> Integer,
        thread_id -> Integer,
        created -> Integer,
    }
}

table! {
    threads (id) {
        id -> Integer,
        board_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        pass_sha -> Text,
        salt -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    boards,
    post_contents,
    posts,
    threads,
    users,
);
