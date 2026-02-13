use crate::load::execute_command;
use clap::Parser;

mod load;

/// Make-like task executor for Unix-based operating systems
#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(name = "jake")]
#[command(about, long_about = None)]
struct Args {
    /// Task to execute (has to be defined within jakefile.toml)
    task: String,

    /// Options for the command to be executed with
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    options: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    execute_command(&args.task, &args.options)?;
    Ok(())
}
