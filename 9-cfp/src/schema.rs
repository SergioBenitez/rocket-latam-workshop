table! {
    talks (id) {
        id -> Integer,
        presenter -> Integer,
        title -> Text,
        status -> Integer,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        github_id -> Text,
        email -> Text,
        name -> Text,
        is_admin -> Bool,
    }
}

joinable!(talks -> users (presenter));

allow_tables_to_appear_in_same_query!(talks, users,);
