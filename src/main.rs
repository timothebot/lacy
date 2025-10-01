use clap::Parser;
use cmd::LacyCli;

use crate::cmd::Run;

mod cmd;
mod query;
mod ui;

fn main() {
    LacyCli::parse().run();
}
