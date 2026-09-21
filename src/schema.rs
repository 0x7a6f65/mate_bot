diesel::table! {
    mates (id) {
        id -> BigInt,
        username -> Text,
        count -> Integer,
    }
}

diesel::table! {
    allowed_chats (id) {
        id -> BigInt,
    }
}
