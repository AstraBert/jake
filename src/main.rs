use crate::{
    initialize::write_jakefile,
    load::{execute_command, execute_default_command, is_posix_os, list_jakefile_tasks},
    models::CommandExecutor,
};
use anyhow::anyhow;
use clap::Parser;

mod env_vars;
mod initialize;
mod load;
mod models;

/// Make-like task executor for Unix-based operating systems
#[derive(Parser, Debug)]
#[command(version = "0.5.0")]
#[command(name = "jake")]
#[command(about, long_about = None)]
struct Args {
    /// Task to execute (has to be defined within jakefile.toml)
    task: Option<String>,

    /// Options for the command to be executed with
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    options: String,

    /// Resolve and load a .env file from the current path of its ancestors
    #[arg(long, default_value_t = false)]
    env: bool,

    /// List the tasks available within jakefile.toml
    #[arg(long, default_value_t = false)]
    list: bool,

    /// Initialize a Jakefile by providing a list of comma-separated tasks
    #[arg(long, default_value = None)]
    init: Option<String>,
}

fn main() -> anyhow::Result<()> {
    if !is_posix_os() {
        return Err(anyhow!(
            "jake` is not supported on operating systems outside of the Unix family"
        ));
    }
    let args = Args::parse();
    if let Some(tasks) = args.init {
        write_jakefile(&tasks, None)?;
        return Ok(());
    }
    if args.list {
        let tasks = list_jakefile_tasks(None)?;
        let task_list = tasks.join("\n- ");
        println!("Available tasks:\n- {}\n", task_list);
        return Ok(());
    }
    let executor = CommandExecutor::new();
    match args.task {
        Some(t) => execute_command(None, &t, &args.options, &executor, args.env)?,
        None => execute_default_command(None, &args.options, &executor, args.env)?,
    }
    Ok(())
}
