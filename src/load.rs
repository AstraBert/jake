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
        Ok(table)
    } else {
        Err(anyhow!("jakefile.toml does not exist"))
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
    let cmd;
    if let Some(task_table) = available_tasks[task].as_table() {
        if !task_table.contains_key("command") {
            return Err(anyhow!(
                "`command` key not available for the requested task: ensure that there are no typos and the TOML syntax is correct before running again"
            ));
        }
        if task_table.contains_key("depends_on")
            && let Some(depends) = task_table["depends_on"].as_array()
        {
            for value in depends {
                match value.as_str() {
                    Some(c) => execute_command(jakefile_path, c, "", executor)?,
                    None => continue,
                }
            }
        }
        match task_table["command"].as_str() {
            Some(c) => cmd = c,
            None => return Err(anyhow!("Unsupported value for the task's command")),
        }
    } else {
        match available_tasks[task].as_str() {
            Some(t) => cmd = t,
            None => return Err(anyhow!("Unsupported value for the task's command")),
        }
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

pub fn execute_default_command(
    jakefile_path: Option<&str>,
    flags: &str,
    executor: &dyn Executor,
) -> Result<()> {
    let available_tasks = parse_jakefile(jakefile_path)?;
    if available_tasks.contains_key("default") {
        execute_command(jakefile_path, "default", flags, executor)?;
    } else {
        let first_key = available_tasks.keys().next();
        match first_key {
            None => return Err(anyhow!("could not find any task within jakefile")),
            Some(task) => {
                execute_command(jakefile_path, task, flags, executor)?;
            }
        }
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
                assert!(t.contains_key("strcmd"));
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
                match t["strcmd"].as_str() {
                    None => {
                        println!("strcmd is not a string");
                        assert!(false); // fail here
                    }
                    Some(s) => {
                        assert_eq!(s, "echo ciao");
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
        let result_2 = execute_default_command(Some("testfiles/jakefile.toml"), "", &mock_executor);
        assert!(result_2.is_ok());
        let mock_content_2 =
            std::fs::read_to_string("test.mock").expect("Should be able to read test.mock");
        assert_eq!(mock_content_2.trim(), "echo 'hello'");
        let result_3 =
            execute_default_command(Some("testfiles/withdefault.toml"), "", &mock_executor);
        assert!(result_3.is_ok());
        let mock_content_3 =
            std::fs::read_to_string("test.mock").expect("Should be able to read test.mock");
        assert_eq!(mock_content_3.trim(), "true");
        let result_4 = execute_command(
            Some("testfiles/jakefile.toml"),
            "strcmd",
            "",
            &mock_executor,
        );
        assert!(result_4.is_ok());
        let mock_content_4 =
            std::fs::read_to_string("test.mock").expect("Should be able to read test.mock");
        assert_eq!(mock_content_4.trim(), "echo ciao");
    }

    #[test]
    fn test_command_execution() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "say-hello", "", &executor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_execution_task_not_found() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "say-ciao", "", &executor);
        assert!(result.is_err_and(|e| e.to_string()
            == "Task not available. Please define it within jakefile.toml".to_string()));
    }

    #[test]
    fn test_command_execution_unexpected_format() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/withdefault.toml"), "error", "", &executor);
        assert!(result.is_err_and(
            |e| e.to_string() == "Unsupported value for the task's command".to_string()
        ));
    }

    #[test]
    fn test_command_execution_no_command() {
        let executor = CommandExecutor::new();
        let result = execute_command(
            Some("testfiles/withdefault.toml"),
            "nocommand",
            "",
            &executor,
        );
        assert!(result.is_err_and(
            |e| e.to_string() == "`command` key not available for the requested task: ensure that there are no typos and the TOML syntax is correct before running again".to_string()
        ));
    }

    #[test]
    fn test_command_execution_wrong_command() {
        let executor = CommandExecutor::new();
        let result = execute_command(
            Some("testfiles/jakefile.toml"),
            "wrongcommand",
            "",
            &executor,
        );
        assert!(result.is_err_and(
            |e| e.to_string() == "Unsupported value for the task's command".to_string()
        ));
    }

    #[test]
    fn test_command_execution_with_deps() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "say-bye", "", &executor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_execution_from_str() {
        let executor = CommandExecutor::new();
        let result = execute_command(Some("testfiles/jakefile.toml"), "strcmd", "", &executor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_command_with_default() {
        let executor = CommandExecutor::new();
        let result = execute_default_command(Some("testfiles/withdefault.toml"), "", &executor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_command_first_key() {
        let executor = CommandExecutor::new();
        let result = execute_default_command(Some("testfiles/jakefile.toml"), "", &executor);
        assert!(result.is_ok());
    }
}
