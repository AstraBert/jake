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
