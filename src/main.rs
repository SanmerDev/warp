use crate::app::App;
use crate::input::Input;
use crate::key::Key;
use crate::output::Output;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

mod app;
mod input;
mod key;
mod output;

/// Unofficial CLI for Cloudflare Zero Trust
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Zero Trust JWT token
    #[arg(short, long, value_name = "TOKEN")]
    token: Option<String>,

    /// Write output to json
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,
}

fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[derive(Deserialize, Serialize, Debug)]
struct Out {
    key: Key,
    output: Output,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let token = args.token.unwrap_or_else(|| env::var("TOKEN").unwrap());
    let token = token.trim();

    init_logger();
    let key = Key::new();
    let input = Input::new(&key);
    let app = App::new(token);

    if let Some(output) = app.register(&input).await {
        let out = Out { key, output };
        let out = serde_json::to_string(&out).unwrap();
        tracing::info!(target: "main", out = %out);

        if let Some(file) = args.output {
            let out_str = serde_json::to_string_pretty(&out).unwrap();
            fs::write(file, out_str).unwrap();
        }
    }
}
