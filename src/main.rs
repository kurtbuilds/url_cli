use std::collections::HashMap;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    parts: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut parts = Vec::new();
    let mut it = cli.parts.into_iter();
    while let Some(p) = it.next() {
        if p.contains('=') {
            let mut kv = p.split('=');
            let k = kv.next().unwrap().to_string();
            let v = kv.next().unwrap().to_string();
            parts.push((k, v));
        } else {
            let v = it.next().unwrap();
            parts.push((p, v));
        }
    }
    let parts = parts.into_iter().collect::<HashMap<_, _>>();
    let qs = serde_qs::to_string(&parts).unwrap();
    println!("{}", qs);
}