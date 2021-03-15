use crate::*;
use reqwest;

#[derive(Default)]
pub struct Command;

impl Command {
    pub fn handle(&self, cmd: TokenCommand) {
        match &cmd.cmd[..] {
            "create" => {
                println!("{} {}",cmd.domain,cmd.name)
            },
            _ => {
                println!("invalid command")
            }
        }
    }
}
