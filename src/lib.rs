use std::io;

use clap::{App, Clap, IntoApp};

use clap_generate::{
    generate,
    generators::{Bash, Fish, Zsh},
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
enum SubCommand {
    #[clap(version = "1.0", author = "Meduo G. <meduo@foxmail.com>")]
    gencompletion(ShellCompletion),
}

/// A subcommand to generate shell completion
#[derive(Clap)]
struct ShellCompletion {
    /// create shell completion
    #[clap(short, long, default_value = "bash")]
    shell: String,
}

fn generate_completion(mut app: App,shell: String) {
    match &shell[..] {
        "bash" => {
            generate::<Bash, _>(&mut app, "uCli", &mut io::stdout());
        }
        "zsh" => {
            generate::<Zsh, _>(&mut app, "uCli", &mut io::stdout());
        }
        "fish" => {
            generate::<Fish, _>(&mut app, "uCli", &mut io::stdout());
        }
        _ => {}
    };
}

pub fn handleCmd() {
    let opts = Opts::parse();
    let mut app: App = Opts::into_app();

    match opts.subcmd {
        SubCommand::gencompletion(completation) => {
            generate_completion(app,completation.shell);
        }
    }
}
