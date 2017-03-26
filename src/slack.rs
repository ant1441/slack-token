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
pub struct SlackResponse<'a> {
    response_type: ResponseType,
    text: Option<&'a str>,
    attachments: Vec<SlackAttachment>,
}

#[derive(Serialize)]
pub struct SlackAttachment {
    text: String,
}

impl<'r> SlackResponse<'r> {
    pub fn ephemeral_text(text: &str) -> SlackResponse {
        SlackResponse {
            response_type: Ephemeral,
            text: Some(text),
            attachments: vec![]
        }
    }

    pub fn inchannel_text(text: &str) -> SlackResponse {
        SlackResponse {
            response_type: InChannel,
            text: Some(text),
            attachments: vec![]
        }
    }
}

pub fn send_help<'a>() -> SlackResponse<'a> {
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
pub fn format_list<'a>(items: &[&str]) -> SlackResponse<'a> {
    let text = String::new();
    let text = items.iter().enumerate().fold(text, |acc, (i, &s)| {
        acc + &format!("{}: {}\n", &i.to_string(), s)
    });
    let attachment = SlackAttachment { text: text };
    SlackResponse {
        response_type: InChannel,
        text: None,
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
        let ret = format_list(&list);
    }
}
