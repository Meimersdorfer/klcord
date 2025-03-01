use keynergy::layout::Layout;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;
use std::fs;

use crate::utility::*;

pub struct Bot {
    layouts: HashMap<String, Layout>,
    names: Vec<String>
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
        layouts: HashMap::new(),
        names: Vec::new()
        }
    }

    pub fn with_layouts_in_dir(dir: &str) -> Bot {
        let mut bot = Bot::new();
        let dir = fs::read_dir(format!("./{}", dir)).unwrap();
        print!("Reading layouts... ");

        for file in dir.into_iter() {
            if let Ok(dir_entry) = file {
                if let Some(path) = dir_entry.path().to_str() {
                    if let Ok(mut l) = Layout::load(path) {
                        if l.link == Some(String::from("")) {
                            l.link = None;
                        }
                        let name = l.name.to_ascii_lowercase();
                        bot.layouts.insert(name.clone(), l);    //we do a little cloning :tf:
                        bot.names.push(name);
                    }
                }
            }
        }
        println!("done!");
        bot
    }
}

#[async_trait]
impl EventHandler for Bot {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content.starts_with("!layout") {
            let split: Vec<&str> = msg.content.split_whitespace().collect();
            if split.len() == 1 {
                send_message(&ctx, &msg, "Usage: `!layout <layout name>`").await;
            } else {
                let mut name = split[1..].join(" ").to_ascii_lowercase();

                if name == String::from("mtgap") {
                    name = String::from("mtgap30");
                }

                match self.layouts.get(&name) {
                    None => {
                        send_message(&ctx, &msg, format!(
                            "This layout does not exist.\n\
                             Did you mean {}?", closest_match(name, &self.names))).await;
                    }
                    Some(l) => {
                        send_message(&ctx, &msg, print_layout(l)).await;
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}