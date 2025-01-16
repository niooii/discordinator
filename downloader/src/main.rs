mod conversation;
mod context;

use std::path::PathBuf;
use anyhow::Result;
use clap::{command, Parser};
use context::Context;
use discord::{client::DiscordClientBuilder, model::Snowflake, MessageFetchRate};
use discord::StreamExt;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Discord authentication token
    #[arg(short, long, value_name = "TOKEN")]
    auth: String,
    
    /// ID of the user whose messages are prioritized in downloading
    #[arg(short, long, value_name = "ID")]
    user: Snowflake,

    /// ID of the channel to download from
    #[arg(short, long, value_name = "ID")]
    channel: Snowflake,
    
    /// Custom output filename
    #[arg(short, long, value_name = "NAME")]
    out: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let auth = cli.auth;
    let uid = cli.user;
    let cid = cli.channel;

    let client = DiscordClientBuilder::new(&auth)
        .set_random_agent(1)
        .build().await?;
    let mut ctx = Context::new();

    println!("Hello, {}!", client.me_cached().username);

    let mut msg_stream = client.messages(&cid, MessageFetchRate::Default);
    const FETCH_COUNT: usize = 100;
    let mut i = 0;
    while let Some(chunk) = msg_stream.next().await {
        i += 1;
        if i > FETCH_COUNT {
            break;
        }
        match chunk {
            Ok(msgs) => {
                ctx.add_chunk(&msgs);
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }

    let out = cli.out.unwrap_or(format!("{uid}_{cid}"));
    println!("Saving context as {out}");
    
    ctx.save_to(PathBuf::from(out)).await;

    Ok(())
}
