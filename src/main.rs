use crate::{
    load::{execute_command, execute_default_command},
    models::CommandExecutor,
};
use clap::Parser;

mod load;
mod models;

/// Make-like task executor for Unix-based operating systems
#[derive(Parser, Debug)]
#[command(version = "0.2.0")]
#[command(name = "jake")]
#[command(about, long_about = None)]
struct Args {
    /// Task to execute (has to be defined within jakefile.toml)
    task: Option<String>,

    /// Options for the command to be executed with
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    options: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let executor = CommandExecutor::new();
    match args.task {
        Some(t) => execute_command(None, &t, &args.options, &executor)?,
        None => execute_default_command(None, &args.options, &executor)?,
    }
    Ok(())
}
