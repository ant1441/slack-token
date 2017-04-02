use token::User;

pub type TeamId = String;
pub type ChannelId = String;
pub type UserId = String;
pub type UserName = String;

#[derive(FromForm)]
pub struct SlashCommandData {
    pub token: String,
    pub team_id: TeamId,
    pub team_domain: String,
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub user_id: UserId,
    pub user_name: UserName,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

#[derive(Serialize)]
enum ResponseType {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "in_channel")]
    InChannel,
}
use self::ResponseType::*;

#[derive(Serialize)]
pub struct SlackResponse {
    response_type: ResponseType,
    text: Option<String>,
    attachments: Vec<SlackAttachment>,
}

#[derive(Serialize)]
pub struct SlackAttachment {
    text: String,
}

impl SlackResponse {
    pub fn ephemeral_text(text: &str) -> SlackResponse {
        SlackResponse {
            response_type: Ephemeral,
            text: Some(text.to_owned()),
            attachments: vec![]
        }
    }

    pub fn inchannel_text(text: &str) -> SlackResponse {
        SlackResponse {
            response_type: InChannel,
            text: Some(text.to_owned()),
            attachments: vec![]
        }
    }
}

pub fn send_help() -> SlackResponse {
    SlackResponse {
        response_type: Ephemeral,
        text: None,
        attachments: vec![SlackAttachment {
                              text: "
Token manager. Use `/token get` to take hold of the token.
\
                                     Other commands available:
• `/token get` adds yourself to \
                                     the queue
• `/token drop` removes yourself from the queue
"
                                  .to_string(),
                          }],
    }
}

/// Format a list into a simple Slack response, with each item numbered
pub fn format_list<'a, I>(text: Option<String>, items: I) -> SlackResponse
    where I: Iterator<Item=&'a User>
{
    let string = String::new();
    let string = items.fold(string, |acc, s| {
        acc + ":large_blue_circle: " + &s.as_slack_str() + "\n"
    });
    let attachment = SlackAttachment { text: string };
    SlackResponse {
        response_type: InChannel,
        text: text.map(|s| s.to_owned()),
        attachments: vec![attachment],
    }
}

pub fn validate_command(command: &SlashCommandData) -> Result<(), &'static str> {
    if !valid_team(&command.team_id) {
        return Err("invalid team");
    }
    if !valid_command(&command.command) {
        return Err("invalid command");
    }
    Ok(())
}

fn valid_team(_team: &str) -> bool {
    true
}

fn valid_command(_command: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_list() {
        let list = vec!["one", "two"];
        let ret = format_list(None, &list);
    }
}
