use std::path::Path;

use crate::models::Executor;
use anyhow::{Result, anyhow};
use toml::Table;

const JAKEFILE: &str = "jakefile.toml";

pub fn parse_jakefile(file_path: Option<&str>) -> Result<Table> {
    let owned_path;
    let path = match file_path {
        None => Path::new(JAKEFILE),
        Some(p) => {
            owned_path = p;
            Path::new(&owned_path)
        }
    };
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        let table = content.parse::<Table>()?;
        return Ok(table);
    } else {
        return Err(anyhow!("jakefile.toml does not exist"));
    }
}

pub fn execute_command(
    jakefile_path: Option<&str>,
    task: &str,
    flags: &str,
    executor: &dyn Executor,
) -> Result<()> {
    if task.is_empty() {
        return Ok(());
    }
    let cmd_options: Vec<&str> = if flags.is_empty() {
        vec![]
    } else {
        flags.split_whitespace().collect()
    };
    let available_tasks = parse_jakefile(jakefile_path)?;
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
                        Some(c) => execute_command(jakefile_path, c, "", executor)?,
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
    if cmd_parts.len() == 1 && cmd_options.is_empty() {
        executor.execute(main_command, vec![])?;
    } else if cmd_parts.len() == 1 && !cmd_options.is_empty() {
        executor.execute(main_command, cmd_options)?;
    } else if cmd_parts.len() > 1 && cmd_options.is_empty() {
        let cmd_slice = &cmd_parts[1..];
        executor.execute(main_command, cmd_slice.to_vec())?;
    } else {
        let cmd_slice = [&cmd_parts[1..], &cmd_options[..]].concat();
        executor.execute(main_command, cmd_slice)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::CommandExecutor;

    use super::*;

    struct MockCommandExecutor;

    impl MockCommandExecutor {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Executor for MockCommandExecutor {
        fn execute(&self, main_command: &str, args: Vec<&str>) -> anyhow::Result<()> {
            let full_command = main_command.to_owned() + " " + &args.join(" ");
            std::fs::write("test.mock", full_command)?;
            Ok(())
        }
    }

    #[test]
    fn test_parse_jakefile() {
        let result = parse_jakefile(Some("testfiles/jakefile.toml"));
        match result {
            Err(e) => {
                println!("An error occurred: {}", e.to_string());
                assert!(false); // fail here
            }
            Ok(t) => {
                assert!(t.contains_key("say-hello"));
                assert!(t.contains_key("say-hello-back"));
                assert!(t.contains_key("say-bye"));
                assert!(t.contains_key("list"));
                match t["say-hello"].as_table() {
                    None => {
                        println!("say-hello is not a table");
                        assert!(false); // fail here
                    }
                    Some(d) => {
                        assert!(d.contains_key("command"));
                        assert!(!d.contains_key("depends_on"));
                    }
                }
                match t["say-bye"].as_table() {
                    None => {
                        println!("say-bye is not a table");
                        assert!(false); // fail here
                    }
                    Some(d) => {
                        assert!(d.contains_key("command"));
                        assert!(d.contains_key("depends_on"));
                    }
                }
            }
        }
    }

    #[test]
    fn test_mock_command_execution() {
        let mock_executor = MockCommandExecutor::new();
        let result = execute_command(
            Some("testfiles/jakefile.toml"),
            "list",
            "-la /hello/something",
            &mock_executor,
        );
        assert!(result.is_ok());
        let mock_content =
            std::fs::read_to_string("test.mock").expect("Should be able to read test.mock");
        assert_eq!(mock_content.trim(), "ls -la /hello/something");
        let result_1 = execute_command(Some("testfiles/jakefile.toml"), "list", "", &mock_executor);
        assert!(result_1.is_ok());
        let mock_content_1 =
            std::fs::read_to_string("test.mock").expect("Should be able to read test.mock");
        assert_eq!(mock_content_1.trim(), "ls");
    }

    #[test]
    fn test_command_execution() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "say-hello", "", &executor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_execution_with_deps() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "say-bye", "", &executor);
        assert!(result.is_ok());
    }
}
