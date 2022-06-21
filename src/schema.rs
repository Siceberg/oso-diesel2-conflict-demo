// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    accounts (id) {
        id -> Uuid,
        email -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    projects (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(accounts, projects,);
