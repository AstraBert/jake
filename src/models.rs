use std::collections::HashSet;

use crate::env_vars::dotenv_to_hashmap;

pub trait Executor {
    fn execute(&self, main_command: &str, args: Vec<&str>, load_env: bool) -> anyhow::Result<()>;
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Executor that prints the command and does not run it (for --dry-run).
pub struct DryRunExecutor;

impl DryRunExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executor for DryRunExecutor {
    fn execute(&self, main_command: &str, args: Vec<&str>, _load_env: bool) -> anyhow::Result<()> {
        let full_command = std::iter::once(main_command)
            .chain(args.into_iter())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", full_command);
        Ok(())
    }
}

impl Executor for CommandExecutor {
    fn execute(&self, main_command: &str, args: Vec<&str>, load_env: bool) -> anyhow::Result<()> {
        let mut command_args = args;
        command_args.insert(0, main_command);
        let full_command = command_args.join(" ");
        let mut cmd = if !load_env {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(full_command)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?
        } else {
            let env_vars = dotenv_to_hashmap()?;
            std::process::Command::new("sh")
                .arg("-c")
                .arg(full_command)
                .envs(env_vars)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?
        };
        cmd.wait()?;
        Ok(())
    }
}

pub struct TaskNode {
    pub command: String,
    pub dependencies: HashSet<String>,
}

impl TaskNode {
    pub fn new(command: String, dependencies: Vec<String>) -> Self {
        let hash_set = HashSet::from_iter(dependencies);
        Self {
            command,
            dependencies: hash_set,
        }
    }
}

pub enum NodeState {
    Univisited,
    Visiting, // currently in the stack
    Visited,
}
