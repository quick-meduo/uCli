use crate::*;

#[derive(Default)]
pub struct Command;

impl Command {
    pub fn handle(&self,mut app: App, cmd: ShellCompletion) {
        match &cmd.shell[..] {
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
}
