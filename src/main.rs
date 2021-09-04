use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::collections::HashMap;
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

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(args) => get(client, &args).await?,
        SubCommand::Post(args) => post(client, &args).await?,
    };
    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let response = client.get(&args.url).send().await?;
    print_resp(response).await?;
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for kv in args.body.iter() {
        body.insert(&kv.key, &kv.value);
    }

    let response = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", response.text().await?);
    Ok(())
}

async fn print_resp(response: Response) -> Result<()> {
    print_status(&response);
    print_headers(&response);

    let mime = get_content_type(&response);
    let body = response.text().await?;
    print_body(mime, body);
    Ok(())
}

fn print_status(response: &Response) {
    let status = format!("{:?} {}", response.version(), response.status()).blue();
    println!("{}\n", status);
}

fn print_headers(response: &Response) {
    for (name, value) in response.headers() {
        println!("{} => {:?}", name.to_string().green(), value);
    }

    println!();
}

fn get_content_type(response: &Response) -> Option<Mime> {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(mime: Option<Mime>, body: String) {
    match mime {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(&body).unwrap().cyan())
        }
        _ => println!("{}", body),
    };
}
