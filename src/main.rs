use clap::{AppSettings, Clap};

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
    url: String,
}

#[derive(Clap, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
