use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
    pw: String,
    salt: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::userdata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserData {
    id: Uuid,
    user_display: String,
    user_url: String,
    email: String,
    dob: NaiveDate,
    location: String,
    socials: serde_json::Value,
    profile_image: serde_json::Value,
    cover_image: serde_json::Value,
    bio: String,

    join_date: NaiveDateTime,
    last_login: NaiveDateTime,

    is_verified: bool,
    is_private: bool,
    followers_count: i32,
    following_count: i32,
    post_count: i32,

    prefs: serde_json::Value,
}

#[derive(Debug)]
#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Parentmode"]
pub enum ParentMode {
    Reblog,
    Reply,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    id: Uuid,
    body: String,
    media: serde_json::Value,
    author: Uuid,
    published: NaiveDateTime,
    likes: i64,
    reblogs: i64,
    comments: i64,
    mentions: Vec<Option<Uuid>>,
    tags: Vec<Option<String>>,
    parent: Option<Uuid>,
    parent_mode: Option<ParentMode>,
    
    flags: serde_json::Value,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::userrelations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRelations {
    actor: Uuid,
    target: Uuid,
    relation: serde_json::Value,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::userrelations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostRelations {
    actor: Uuid,
    target: Uuid,
    relation: serde_json::Value,
}