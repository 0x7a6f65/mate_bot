diesel::table! {
    mates (id) {
        id -> BigInt,
        username -> Text,
        count -> Integer,
    }
}
