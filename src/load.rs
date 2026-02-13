use std::path::Path;

use anyhow::{Result, anyhow};
use toml::Table;

const JAKEFILE: &str = "jakefile.toml";

pub fn parse_jakefile() -> Result<Table> {
    let path = Path::new(JAKEFILE);
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        let table = content.parse::<Table>()?;
        return Ok(table);
    } else {
        return Err(anyhow!("jakefile.toml does not exist"));
    }
}

pub fn execute_command(task: &str, flags: &str) -> Result<()> {
    if task.is_empty() {
        return Ok(());
    }
    let cmd_options: Vec<&str> = if flags.is_empty() {
        vec![]
    } else {
        flags.split_whitespace().collect()
    };
    let available_tasks = parse_jakefile()?;
    if !available_tasks.contains_key(task) {
        return Err(anyhow!(
            "Task not available. Please define it within jakefile.toml"
        ));
    }
    let task_table = match available_tasks[task].as_table() {
        Some(t) => t,
        None => {
            return Err(anyhow!("No associated value to request task"));
        }
    };
    if !task_table.contains_key("command") {
        return Err(anyhow!(
            "`command` key not available for the requested task: ensure that there are no typos and the TOML syntax is correct before running again"
        ));
    }
    if task_table.contains_key("depends_on") {
        match task_table["depends_on"].as_array() {
            Some(depends) => {
                for value in depends {
                    match value.as_str() {
                        Some(c) => execute_command(c, "")?,
                        None => continue,
                    }
                }
            }
            None => {}
        };
    }
    let cmd = match task_table["command"].as_str() {
        Some(c) => c,
        None => return Err(anyhow!("Unsupported empty or null command for task")),
    };
    let cmd_parts: Vec<&str> = cmd.split_whitespace().collect();
    let main_command = cmd_parts[0];
    let mut cmd = if cmd_parts.len() == 1 && cmd_options.is_empty() {
        std::process::Command::new(main_command)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    } else if cmd_parts.len() == 1 && !cmd_options.is_empty() {
        std::process::Command::new(main_command)
            .args(cmd_options)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    } else if cmd_parts.len() > 1 && cmd_options.is_empty() {
        let cmd_slice = &cmd_parts[1..];
        std::process::Command::new(main_command)
            .args(cmd_slice)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    } else {
        let cmd_slice = [&cmd_parts[1..], &cmd_options[..]].concat();
        std::process::Command::new(main_command)
            .args(cmd_slice)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    };
    cmd.wait()?;
    Ok(())
}
