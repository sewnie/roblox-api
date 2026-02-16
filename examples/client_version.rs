use dotenvy_macro::dotenv;
use roblox_api::{api::clientsettings, client::Client};

#[tokio::main]
async fn main() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    let version = clientsettings::v2::client_version(&mut client, "WindowsStudio64", None)
        .await
        .unwrap();
    println!("Version: {}", version.version);
    println!("Version Upload: {}", version.upload);

    let cdn = clientsettings::v1::installer_cdn(&mut client)
        .await
        .unwrap();
    println!("Known CDN: {}", cdn);
}
