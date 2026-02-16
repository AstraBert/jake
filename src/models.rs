use std::collections::HashSet;

pub trait Executor {
    fn execute(&self, main_command: &str, args: Vec<&str>) -> anyhow::Result<()>;
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executor for CommandExecutor {
    fn execute(&self, main_command: &str, args: Vec<&str>) -> anyhow::Result<()> {
        let mut cmd = std::process::Command::new(main_command)
            .args(args)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;
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
    Univisted,
    Visiting, // currently in the stack
    Visited,
}
