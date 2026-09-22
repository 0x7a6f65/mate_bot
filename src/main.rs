use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use teloxide::utils::command::{self, BotCommands};
use teloxide::{prelude::*};

use mate_bot::models::*;
use mate_bot::schema::allowed_chats::dsl::*;
use mate_bot::schema::mates::dsl::*;

#[derive(BotCommands, Clone)]
#[command(
    description = "These commands are supported:",
    rename_rule = "lowercase"
)]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Top statistics")]
    Stats(i32),
    #[command(description = "My statistics")]
    MyStats,
    #[command(description = "Enable chat to count matés")]
    Enable,
    #[command(description = "Fact checked TRUE maté by real argentinian patriots")]
    Add,
    #[command(description = "Remove a FAKE maté")]
    Remove,
    #[command(description = "This bot is licensed AGPL, check out the source code")]
    Source,
}

pub fn connect() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&db_url).unwrap()
}

async fn photo_handler(_: Bot, msg: Message) -> ResponseResult<()> {
    let conn = &mut connect();

    let sender = match &msg.from {
        Some(u) => u.id.0 as i64,
        _ => 0,
    };
    let chat = allowed_chats
        .find(msg.chat.id.0 as i64)
        .select(AllowedChats::as_select())
        .load(conn);

    if sender != 0 && chat.is_ok() && chat.unwrap_or_else(|_| Vec::new()).len() > 0 {
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
                    display_name: user.full_name().to_string(),
                    count: 1,
                };

                diesel::insert_into(mate_bot::schema::mates::table)
                    .values(&new_data)
                    .execute(conn)
                    .unwrap();
            }
        };
    }

    Ok(())
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let conn = &mut connect();

    match cmd {
        Command::Help => bot
            .send_message(msg.chat.id, Command::descriptions().to_string())
            .await
            .unwrap(),
        Command::Stats(c) => {
            let mut lb: String = String::new();
            let mut j = 1;

            for i in mates
                .order_by(mate_bot::schema::mates::count.desc())
                .limit(c as i64)
                .select(Mates::as_select())
                .load(conn)
                .unwrap()
            {
                let line = format!("{j}: {} -- {}\n", i.display_name, i.count);
                lb.push_str(line.as_str());
                j += 1;
            }

            bot.send_message(msg.chat.id, lb).await?
        }
        Command::Enable => {
            if msg.from.clone().unwrap().id.0 == 8322506629 {
                let chat = allowed_chats
                    .find(msg.chat.id.0)
                    .select(AllowedChats::as_select())
                    .load(conn);

                if chat.is_ok() {
                    let new_data = AllowedChats { id: msg.chat.id.0 };

                    diesel::insert_into(mate_bot::schema::allowed_chats::table)
                        .values(&new_data)
                        .execute(conn)
                        .unwrap();

                    bot.send_message(msg.chat.id, format!("Enabled maté count in this channel"))
                        .await?
                } else {
                    bot.send_message(msg.chat.id, format!("Some error i don't understand"))
                        .await?
                }
            } else {
                bot.send_message(msg.chat.id, format!("Not an admin!!"))
                    .await?
            }
        }
        Command::MyStats => {
            let from = &msg.from;

            let user = mates
                .find(from.clone().unwrap().id.0 as i64)
                .select(Mates::as_select())
                .load(conn)
                .unwrap();

            if user.len() > 0 {
                let user = &user[0];
                
                bot.send_message(msg.chat.id, format!("You have drunk {} matés", user.count))
                .await?
            } else {
                bot.send_message(msg.chat.id, format!("You have yet to drink an maté"))
                .await?
            }
        }
        Command::Source => {
            bot.send_message(msg.chat.id, format!("See the license for this cool bot over at https://github.com/PadjokeJ/mate_bot"))
                .await?
        }
        _ => {
            bot.send_message(msg.chat.id, format!("Not implemented yet :<"))
                .await?
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::new(env::var("TELOXIDE_TOKEN").unwrap());

    let cmd = teloxide::filter_command::<Command, _>().endpoint(command_handler);

    let schema = Update::filter_message()
        .branch(cmd)
        .branch(Message::filter_photo().endpoint(photo_handler));

    Dispatcher::builder(bot, schema)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
