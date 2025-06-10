use clap::Parser;
use rand::{Rng, distr::Alphanumeric, rng};
use std::io::Write;
#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Whether to append the generated keys to the .env file
    #[arg(short, long, default_value_t = false)]
    append_to_env_file: bool,
    /// Length of the generated bearer key
    #[arg(short = 'l', long, default_value_t = 64)]
    key_length: usize,
    /// Number of keys to generate (default is 1)
    num: Option<usize>,
}

pub fn generate_bearer_key(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
fn main() {
    let cli = Cli::parse();

    let mut api_keys = Vec::new();
    let num_keys = cli.num.unwrap_or(1);
    for _ in 0..num_keys {
        let key = generate_bearer_key(64);
        api_keys.push(format!("chai_{}", key));
    }
    if cli.append_to_env_file {
        let mut env_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(".env")
            .expect("Failed to open .env file");

        writeln!(env_file, "API_KEYS={}", api_keys.join(","))
            .expect("Failed to write to .env file");
    } else {
        for key in api_keys {
            println!("{}", key);
        }
    }
}
