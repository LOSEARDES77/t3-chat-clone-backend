use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::{impl_diesel_crud, models::DieselCrud};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    pub id: i32,
    pub title: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::conversations)]
pub struct NewConversation {
    pub title: Option<String>,
}

impl_diesel_crud!(Conversation, NewConversation, conversations, id, i32);
