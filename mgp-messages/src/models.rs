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
    pub id: Uuid,
    pub body: String,
    pub media: serde_json::Value,
    pub author: Uuid,
    pub published: NaiveDateTime,
    pub likes: i64,
    pub reblogs: i64,
    pub comments: i64,
    pub mentions: Vec<Option<Uuid>>,
    pub tags: Vec<Option<String>>,
    pub parent: Option<Uuid>,
    pub parent_mode: Option<ParentMode>,

    pub flags: serde_json::Value,
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