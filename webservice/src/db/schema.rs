table! {
    canals (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    clients (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        pass -> Text,
        salt -> Text,
    }
}

table! {
    messages (id) {
        id -> Integer,
        body -> Text,
        client -> Varchar,
        date -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    canals,
    clients,
    messages,
);
