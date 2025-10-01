use clap::Parser;
use cmd::LacyCli;

use crate::cmd::Run;

mod cmd;
mod fuzzy;
mod query;
mod ui;

fn main() {
    LacyCli::parse().run();
}
