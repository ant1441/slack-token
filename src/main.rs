#![feature(conservative_impl_trait, custom_derive, plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::Path;
use std::sync::{Arc, RwLock};

use rocket::State;
use rocket::request::Form;
use rocket_contrib::JSON;

mod config;
#[macro_use]
mod macros;
mod slack;
mod token;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/slack", format = "application/x-www-form-urlencoded", data = "<slash_form>")]
fn slack<'a>(slash_form: Form<slack::SlashCommandData>,
             config: State<config::CommandConfig>,
             tokens: State<token::Tokens>)
             -> Result<JSON<slack::SlackResponse>, &'static str> {
    let slash = slash_form.get();
    if slash.token != config.token {
        return Err("token mismatch");
    }
    slack::validate_command(&slash)?;

    let mut tokens_map = tokens.0.lock().unwrap();
    let token_entry = tokens_map.entry((slash.team_id.to_owned(), slash.channel_id.to_owned()));
    let token = token_entry.or_insert(Arc::new(RwLock::new(token::Token::new())));
    let user = token::User::new(slash.user_id.to_owned(), slash.user_name.to_owned());

    match slash.text.to_lowercase().trim() {
        "list" => {
            printlist!(token)
        }
        "get" => {
            if let Err(e) = (*token.write().map_err(|_| "unable to lock token (w)")?).get(user.clone()) {
                return Ok(JSON(slack::SlackResponse::ephemeral_text(e)));
            }
            printlist!(token, "{} joined the queue", user.as_slack_str())
        }
        "drop" => {
            if let Err(e) = (*token.write().map_err(|_| "unable to lock token (w)")?).drop(&user) {
                return Ok(JSON(slack::SlackResponse::ephemeral_text(e)));
            }
            printlist!(token, "{} dropped the token", user.as_slack_str())
        }
        "afteryou" => {
            if let Err(e) = (*token.write().map_err(|_| "unable to lock token (w)")?).step_back(&user) {
                return Ok(JSON(slack::SlackResponse::ephemeral_text(e)));
            };
            printlist!(token)
        }
        "barge" => {
            if let Err(e) = (*token.write().map_err(|_| "unable to lock token (w)")?).to_front(&user) {
                return Ok(JSON(slack::SlackResponse::ephemeral_text(e)));
            };
            printlist!(token, "{} barged to the front!", user.as_slack_str())
        }
        "steal" => {
            if let Err(e) = (*token.write().map_err(|_| "unable to lock token (w)")?).steal(&user) {
                return Ok(JSON(slack::SlackResponse::ephemeral_text(e)));
            };
            printlist!(token, "{} stole the token!", user.as_slack_str())
        }
        _ => Ok(JSON(slack::send_help())),
    }
}

fn format_list<'a>(text: Option<String>, token: &token::Token) -> Result<JSON<slack::SlackResponse>, &'static str> {
    if token.len() == 0 {
        if let Some(text) = text {
            Ok(JSON(slack::SlackResponse::inchannel_text(&(text + &"\nNo one in the Token queue"))))
        } else {
            Ok(JSON(slack::SlackResponse::inchannel_text("No one in the Token queue")))
        }
    } else {
        let list = token.iter();
        Ok(JSON(slack::format_list(text, list)))
    }
}

fn main() {
    let config = config::CommandConfig::from_path(Path::new("./config.json")).unwrap();
    let tokens = token::Tokens::new();
    rocket::ignite()
        .mount("/", routes![index, slack])
        .manage(config)
        .manage(tokens)
        .launch();
}
