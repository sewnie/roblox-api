use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://clientsettings.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClientVersion {
    pub version: String,
    #[serde(rename = "clientVersionUpload")]
    pub upload: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserChannel {
    #[serde(rename = "channelName")]
    pub name: String,
    pub token: Option<String>,
}

pub async fn client_version(
    client: &mut Client,
    binary_type: &str,
    channel: Option<&str>,
) -> Result<ClientVersion, Error> {
    let mut url = format!("{URL}/client-version/{binary_type}");
    if let Some(chan) = channel {
        url.push_str(&format!("/channel/{chan}"));
    }

    let result = client
        .requestor
        .client
        .get(url)
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<ClientVersion>(response).await
}

pub async fn user_channel(
    client: &mut Client,
    binary_type: Option<&str>,
) -> Result<UserChannel, Error> {
    let mut url = format!("{URL}/client-version");
    if let Some(bt) = binary_type {
        url.push_str(&format!("?binaryType={}", bt));
    }

    let result = client
        .requestor
        .client
        .get(url)
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<UserChannel>(response).await
}
