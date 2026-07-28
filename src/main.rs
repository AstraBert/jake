use std::path::PathBuf;

use crate::{
    initialize::write_jakefile,
    load::{execute_command, execute_default_command, is_posix_os, list_jakefile_tasks},
    models::{CommandExecutor, DryRunExecutor},
    package_json::execute_script,
};
use anyhow::anyhow;
use clap::Parser;

mod code_blocks;
mod env_vars;
mod initialize;
mod load;
mod models;
mod package_json;

/// Make-like task executor for Unix-based operating systems
#[derive(Parser, Debug)]
#[command(version = "0.10.0")]
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

    /// Load and execute scripts from a package.json file instead of a jakefile.toml
    #[arg(long, default_value_t = false)]
    js: bool,

    /// Initialize a Jakefile by providing a list of comma-separated tasks
    #[arg(long, default_value = None)]
    init: Option<String>,

    /// Print commands that would be run without executing them
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Directory under which jakefile.toml can be found
    /// and that should be used as current working directory
    /// for the command execution
    #[arg(long, default_value = None)]
    cwd: Option<String>,

    /// Do not print the command
    /// that you are about to execute
    /// (only applicable for non-dry runs)
    #[arg(long, default_value_t = false)]
    no_print_command: bool,
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
    let (executor, print_command): (Box<dyn models::Executor>, bool) = if args.dry_run {
        (Box::new(DryRunExecutor::new()), false)
    } else {
        (Box::new(CommandExecutor::new()), !args.no_print_command)
    };
    if args.js {
        if let Some(script_name) = args.task {
            let package_json_path = args.cwd.map(|p| {
                PathBuf::from(p)
                    .join("package.json")
                    .to_string_lossy()
                    .to_string()
            });
            execute_script(
                package_json_path,
                script_name,
                args.env,
                executor.as_ref(),
                print_command,
            )?;
        } else {
            return Err(anyhow!(
                "No script name provided, please provide one or, if you wish to execute the default command from jakefile.toml, do not pass the `--js` flag."
            ));
        }
        return Ok(());
    }
    match args.task {
        Some(t) => {
            let jakefile_path = args.cwd.map(|p| {
                PathBuf::from(p)
                    .join("jakefile.toml")
                    .to_string_lossy()
                    .to_string()
            });
            execute_command(
                jakefile_path.as_deref(),
                &t,
                &args.options,
                executor.as_ref(),
                args.env,
                print_command,
            )?
        }
        None => {
            let jakefile_path = args.cwd.map(|p| {
                PathBuf::from(p)
                    .join("jakefile.toml")
                    .to_string_lossy()
                    .to_string()
            });
            execute_default_command(
                jakefile_path.as_deref(),
                &args.options,
                executor.as_ref(),
                args.env,
                print_command,
            )?
        }
    }
    Ok(())
}
