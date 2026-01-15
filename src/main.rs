use clap::Parser;
use cmd::LacyCli;

use crate::cmd::Run;

mod cmd;
mod directory;
mod query;
mod query_part;
mod ui;

fn main() {
    LacyCli::parse().run();
}
