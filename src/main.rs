use crate::{
    load::{execute_command, execute_default_command, is_posix_os, list_jakefile_tasks},
    models::CommandExecutor,
};
use anyhow::anyhow;
use clap::Parser;

mod load;
mod models;

/// Make-like task executor for Unix-based operating systems
#[derive(Parser, Debug)]
#[command(version = "0.4.1")]
#[command(name = "jake")]
#[command(about, long_about = None)]
struct Args {
    /// Task to execute (has to be defined within jakefile.toml)
    task: Option<String>,

    /// Options for the command to be executed with
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    options: String,

    /// List the tasks available within jakefile.toml
    #[arg(long, default_value_t = false)]
    list: bool,
}

fn main() -> anyhow::Result<()> {
    if !is_posix_os() {
        return Err(anyhow!(
            "jake` is not supported on operating systems outside of the Unix family"
        ));
    }
    let args = Args::parse();
    if args.list {
        let tasks = list_jakefile_tasks(None)?;
        let task_list = tasks.join("\n- ");
        println!("Available tasks:\n- {}\n", task_list);
        return Ok(());
    }
    let executor = CommandExecutor::new();
    match args.task {
        Some(t) => execute_command(None, &t, &args.options, &executor)?,
        None => execute_default_command(None, &args.options, &executor)?,
    }
    Ok(())
}
