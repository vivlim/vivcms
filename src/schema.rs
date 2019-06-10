table! {
    post_contents (post_id, revision) {
        post_id -> Integer,
        revision -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    posts (id) {
        id -> Integer,
        author -> Integer,
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
    post_contents,
    posts,
    users,
);
