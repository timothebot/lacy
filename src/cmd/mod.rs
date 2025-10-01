mod cmd;
mod complete;
mod init;
mod prompt;

pub use crate::cmd::cmd::*;

pub trait Run {
    fn run(&self);
}

impl Run for LacyCli {
    fn run(&self) {
        match &self.command {
            LacyCommand::Prompt(cmd) => cmd.run(),
            LacyCommand::Init(cmd) => cmd.run(),
            LacyCommand::Complete(cmd) => cmd.run(),
        }
    }
}
