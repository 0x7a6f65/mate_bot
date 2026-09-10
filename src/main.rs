use diesel::prelude::*;
use dotenvy::dotenv;
use teloxide::dispatching::dialogue::GetChatId;
use std::env;
use teloxide::{prelude::*, types::*};

use mate_bot::models::*;
use mate_bot::schema::mates::dsl::*;

pub fn connect() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&db_url).unwrap()
}

async fn foo(bot: Bot, msg: Message) -> ResponseResult<()> {
    let conn = &mut connect();

    match msg.kind {
        MessageKind::Common(common) => match common.media_kind {
            MediaKind::Photo(_) => {
                let sender = match &msg.from {
                    Some(u) => u.id.0,
                    _ => 0,
                };
                if sender != 0 {
                    let user = mates
                        .find(sender as i64)
                        .select(Mates::as_select())
                        .load(conn);

                    match user {
                        Ok(u) if u.len() >= 1 => {
                            let u = &u[0];

                            diesel::update(mates.find(u.id))
                                .set(mate_bot::schema::mates::count.eq(count + 1))
                                .execute(conn)
                                .unwrap();
                        }
                        _ => {
                            let user = msg.from.unwrap();
                            let new_data = Mates {
                                id: user.id.0 as i64,
                                username: user.full_name(),
                                count: 1,
                            };

                            diesel::insert_into(mate_bot::schema::mates::table)
                                .values(&new_data)
                                .execute(conn)
                                .unwrap();
                        }
                    };
                }
            },
            MediaKind::Text(text) => {
                let lwc = text.text.to_lowercase();
                let mut lb: String = String::new();
                let mut j = 1;

                for i in mates.order_by(mate_bot::schema::mates::count.desc()).limit(10).select(Mates::as_select()).load(conn).unwrap() {
                    let line = format!("{j}: {} -- {}\n", i.username, i.count);
                    lb.push_str(line.as_str());
                    j += 1;
                }

                if lwc.starts_with("/stats") {
                    bot.send_message(msg.chat.id, lb).await.unwrap();
                }
            },
            _ => (),
        },
        _ => (),
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::new(env::var("TELOXIDE_TOKEN").unwrap());

    teloxide::repl(bot, foo).await;
}
