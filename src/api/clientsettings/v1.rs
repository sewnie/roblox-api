use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ApiError, Error, client::Client};

pub const URL: &str = "https://clientsettings.roblox.com/v1";

pub async fn installer_cdn(client: &mut Client) -> Result<String, Error> {
    #[derive(Serialize, Deserialize)]
    struct Response {
        #[serde(flatten)]
        pub cdns: HashMap<String, u32>,
    }

    let result = client
        .requestor
        .client
        .get(format!("{URL}/installer-cdns"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    let result = client.requestor.parse_json::<Response>(response).await?;

    result
        .cdns
        .iter()
        .find(|&(_, &success)| success == 100)
        .map(|(key, _)| key.clone())
        .ok_or(Error::ApiError(ApiError::Internal))
}
