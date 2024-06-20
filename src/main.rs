use std::path::PathBuf;
use std::{env, fs};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::request::RegisterRequest;
use crate::response::RegisterResponse;
use crate::util::Json;
use crate::wireguard::WireguardKey;

mod app;
mod request;
mod response;
mod util;
mod wireguard;

#[derive(Parser)]
#[command(disable_colored_help = true, disable_help_subcommand = true)]
struct Args {
    /// JWT token
    #[arg(short, long, value_name = "TOKEN", global = true)]
    token: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Register new device
    Register {
        /// Write output to json
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
}

fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[derive(Deserialize, Serialize, Debug)]
struct Output {
    key: WireguardKey,
    response: RegisterResponse,
}

async fn register(app: &App, output: Option<PathBuf>) {
    let key = WireguardKey::new();
    let request = RegisterRequest::new(&key);

    if let Some(response) = app.register(&request).await {
        let key_str = key.to_string();
        tracing::info!(target: "main", key = %key_str);

        if let Some(file) = output {
            let output = Output { key, response };
            let output_str = output.to_string_pretty();
            fs::write(file, output_str).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    init_logger();

    let token = args.token.unwrap_or_else(|| env::var("TOKEN").unwrap());
    let token = token.trim();
    let app = App::new(token);

    match args.command {
        Commands::Register { output } => register(&app, output).await,
    }
}
