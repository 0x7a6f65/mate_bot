diesel::table! {
    mates (id) {
        id -> BigInt,
        display_name -> Text,
        count -> Integer,
    }
}

diesel::table! {
    allowed_chats (id) {
        id -> BigInt,
    }
}
