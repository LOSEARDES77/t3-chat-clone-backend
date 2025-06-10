// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        conversation_id -> Int4,
        #[max_length = 20]
        role -> Varchar,
        content -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(messages -> conversations (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(conversations, messages,);
