table! {
    posts (id) {
        id -> Integer,
        author -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
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
    posts,
    users,
);
