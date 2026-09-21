use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::mates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Mates {
    pub id: i64,
    pub display_name: String,
    pub count: i32,
}

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::allowed_chats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AllowedChats {
    pub id: i64,
}
