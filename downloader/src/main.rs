use std::path::PathBuf;

use clap::{command, Parser};
use discord::client::DiscordClientBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Discord authentication token
    #[arg(short, long, value_name = "TOKEN")]
    auth: String,
    
    /// ID of the user whose messages are prioritized in downloading
    #[arg(short, long, value_name = "ID")]
    user: String,

    /// ID of the channel to download from
    #[arg(short, long, value_name = "ID")]
    channel: String,
    
    /// Custom output filename
    #[arg(short, long, value_name = "NAME")]
    out: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let auth = cli.auth;
    let uid = cli.user;
    let cid = cli.channel;

    let client = match DiscordClientBuilder::new(&auth)
        .set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/37.0.2062.94 Chrome/37.0.2062.94 Safari/537.36")
        .build().await {
        Err(e) => {
            eprintln!("{e}");
            return;
        },
        Ok(client) => {
            client
        }
    };

    println!("Hello, world! {}", auth);
}
