// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "parentmode"))]
    pub struct Parentmode;
}

diesel::table! {
    postrelations (actor, target) {
        actor -> Uuid,
        target -> Uuid,
        relation -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Parentmode;

    posts (id) {
        id -> Uuid,
        body -> Text,
        media -> Jsonb,
        author -> Uuid,
        published -> Timestamp,
        likes -> Int8,
        reblogs -> Int8,
        comments -> Int8,
        mentions -> Array<Nullable<Uuid>>,
        tags -> Array<Nullable<Text>>,
        parent -> Nullable<Uuid>,
        parent_mode -> Nullable<Parentmode>,
        flags -> Jsonb,
    }
}

diesel::table! {
    userdata (id) {
        id -> Uuid,
        user_display -> Varchar,
        user_url -> Varchar,
        email -> Varchar,
        dob -> Date,
        location -> Varchar,
        socials -> Jsonb,
        profile_image -> Jsonb,
        cover_image -> Jsonb,
        bio -> Text,
        join_date -> Timestamp,
        last_login -> Timestamp,
        is_verified -> Bool,
        is_private -> Bool,
        followers_count -> Int4,
        following_count -> Int4,
        post_count -> Int4,
        prefs -> Jsonb,
    }
}

diesel::table! {
    userrelations (actor, target) {
        actor -> Uuid,
        target -> Uuid,
        relation -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        pw -> Varchar,
        salt -> Varchar,
    }
}

diesel::joinable!(postrelations -> posts (target));
diesel::joinable!(postrelations -> users (actor));
diesel::joinable!(posts -> users (author));
diesel::joinable!(userdata -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    postrelations,
    posts,
    userdata,
    userrelations,
    users,
);
