// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        #[max_length = 128]
        sender_username -> Nullable<Varchar>,
        chat -> Nullable<Text>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 264]
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(messages -> rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
);
