use std::{collections::HashSet, env, path::PathBuf};

use crate::env_vars::dotenv_to_hashmap;

pub trait Executor {
    fn execute(
        &self,
        full_command: &str,
        load_env: bool,
        cwd: Option<String>,
    ) -> anyhow::Result<()>;
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
    fn execute(
        &self,
        full_command: &str,
        _load_env: bool,
        _cwd: Option<String>,
    ) -> anyhow::Result<()> {
        println!("{}", full_command);
        Ok(())
    }
}

impl Executor for CommandExecutor {
    fn execute(
        &self,
        full_command: &str,
        load_env: bool,
        cwd: Option<String>,
    ) -> anyhow::Result<()> {
        let current_dir = match cwd {
            Some(c) => PathBuf::from(c.replace("\\", "/")).canonicalize()?,
            None => env::current_dir()?,
        };
        let mut cmd = if !load_env {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(full_command)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .current_dir(current_dir)
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
                .current_dir(current_dir)
                .spawn()?
        };
        let status = cmd.wait()?;
        if !status.success() {
            return Err(anyhow::anyhow!(
                "Command exited with a non-zero exit code: {:?}",
                status.code().unwrap_or(1)
            ));
        }

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
