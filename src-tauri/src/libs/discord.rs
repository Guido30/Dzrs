use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderValue};
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::GatewayIntents;
use serenity::model::id::{ChannelId, UserId};

use crate::models::discord::DiscordLogin;

static API_URL: &str = "https://discord.com/api/v9/";
static USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

pub struct DiscordClient {
    pub no_auth_client: reqwest::Client,
    pub auth_client: Option<serenity::Client>,
}

impl DiscordClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        let no_auth_client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(USER_AGENT)
            .build()
            .unwrap_or_default();
        Self {
            no_auth_client,
            auth_client: None,
        }
    }

    // Starts the serenity client using provided token
    pub async fn authenticate(&mut self, token: &str, event_handler: DiscordEventHandler) -> Result<(), String> {
        let auth_client = serenity::Client::builder(token, GatewayIntents::DIRECT_MESSAGES)
            .event_handler(event_handler)
            .await
            .map_err(|err| err.to_string())?;
        self.auth_client = Some(auth_client);
        Ok(())
    }

    // Retrieve a new user token from discord
    pub async fn token(&self, email: String, password: String) -> Result<String, String> {
        let body = format!(
            r#"{{"login":"{}", "password":"{}", "undelete": false, "login_source": null, "gift_code_sku_id": null }}"#,
            email, password
        );
        let res = self
            .no_auth_client
            .post(API_URL.to_owned() + "auth/login")
            .body(body)
            .send()
            .await;
        let payload = res
            .map_err(|err| err.to_string())?
            .json::<DiscordLogin>()
            .await
            .map_err(|err| err.to_string())?;

        Ok(payload.token)
    }

    // Sends the download command in the slavart discord channel
    pub fn send_download_cmd(&self) {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiscordEventHandler {
    channel_id: ChannelId,
    bot_id: UserId,
}

impl DiscordEventHandler {
    pub fn new(channel_id: &str, bot_id: &str) -> Self {
        Self {
            channel_id: ChannelId::from_str(channel_id).unwrap_or_default(),
            bot_id: UserId::from_str(bot_id).unwrap_or_default(),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for DiscordEventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == self.bot_id {
            println!("{:?}", msg.embeds);
        };
    }
}
