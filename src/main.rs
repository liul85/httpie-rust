use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use reqwest::Url;
use std::str::FromStr;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Clap, Debug)]
struct Post {
    #[clap(parse(try_from_str=parse_url))]
    url: String,

    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
    key: String,
    value: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");

        let err = || anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            key: (split.next().ok_or_else(err)?).to_string(),
            value: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
