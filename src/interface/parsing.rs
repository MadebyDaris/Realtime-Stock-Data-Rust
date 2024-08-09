use clap::Parser;
// use crate::app::App;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum command {
    getTask()
}