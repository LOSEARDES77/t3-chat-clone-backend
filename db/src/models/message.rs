use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::DieselCrud;
use diesel::prelude::*;

use crate::impl_diesel_crud;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub role: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::messages)]
pub struct NewMessage {
    pub conversation_id: i32,
    pub role: String,
    pub content: String,
}

impl_diesel_crud!(Message, NewMessage, messages, id, i32);
