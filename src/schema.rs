table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        role -> Varchar,
        logged_in -> Bool,
    }
}
