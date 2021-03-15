use std::io;

use clap::{App, Clap, IntoApp};

use clap_generate::{
    generate,
    generators::{Bash, Fish, Zsh},
};

pub mod Commands;
use Commands::{
    TokenCommandHandler,
    ShellCompleteHandler,
};

#[derive(Clap)]
#[clap(version = "1.0", author = "Meduo G. <meduo@foxmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: Option<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "1.0", author = "Meduo G. <meduo@foxmail.com>")]
    gencompletion(ShellCompletion),
    #[clap(version = "1.0", author = "Meduo G. <meduo@foxmail.com>")]
    token(TokenCommand),
}

/// A subcommand to generate shell completion
#[derive(Clap)]
pub struct ShellCompletion {
    /// create shell completion
    #[clap(short, long, default_value = "bash")]
    shell: String,
}

/// A subcommand to manage security token
#[derive(Clap)]
pub struct TokenCommand {
    /// command(create,query,remove,revoke) to manage sucurity token
    #[clap(short, long, default_value = "create")]
    cmd: String,

    /// name of the user granted
    #[clap(short, long)]
    name: String,

    /// email of the user granted
    #[clap(short, long)]
    email: String,

    /// domain of the user located
    #[clap(short, long)]
    domain: String,
}

pub fn handleCmd() {
    let opts = Opts::parse();
    let mut app: App = Opts::into_app();

    match opts.subcmd {
        SubCommand::gencompletion(completation) => {
            let handler = ShellCompleteHandler::Command::default();
            handler.handle(app,completation);
        },
        SubCommand::token(token) => {
            let handler = TokenCommandHandler::Command::default();
            handler.handle(token);
        }
        _ => {}
    }
}
