use std::{collections::HashMap, env, fs, path::Path};

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::models::Executor;

const PACKAGE_JSON: &str = "package.json";

fn resolve_package_json_path() -> Result<String> {
    let current_dir = env::current_dir()?;
    let dotenv_path = current_dir.join(PACKAGE_JSON);
    if dotenv_path.exists() {
        return Ok(dotenv_path.to_string_lossy().to_string());
    }
    for ancestor in current_dir.ancestors() {
        let dotenv = ancestor.join(PACKAGE_JSON);
        if dotenv.exists() {
            return Ok(dotenv.to_string_lossy().to_string());
        }
    }

    Err(anyhow!(
        "Could not find package.json in the current directory or in any of its ancestors"
    ))
}

fn load_package_json(path: &Path) -> Result<HashMap<String, Value>> {
    let content = fs::read_to_string(path)?;
    let map: HashMap<String, Value> = serde_json::from_str(&content)?;
    Ok(map)
}

fn load_scripts(map: HashMap<String, Value>) -> Result<HashMap<String, String>> {
    if let Some(scripts) = map.get("scripts") {
        let mut val_scripts: HashMap<String, String> = HashMap::new();
        if let Some(mp) = scripts.as_object() {
            for (k, v) in mp.iter() {
                if let Some(val) = v.as_str() {
                    val_scripts.insert(k.as_str().to_string(), val.to_string());
                } else {
                    return Err(anyhow!("Encountered a non-string value."));
                }
            }
        } else {
            return Err(anyhow!("`scripts` is not a JSON map."));
        }
        return Ok(val_scripts);
    }
    Err(anyhow!("Cannot find the `scripts` key in package.json"))
}

fn get_script_command(scripts: HashMap<String, String>, script_name: String) -> Result<String> {
    if let Some(command) = scripts.get(&script_name) {
        return Ok(command.as_str().to_string());
    }
    Err(anyhow!("Could not find script {}", script_name))
}

pub fn execute_script(
    package_json_path: Option<String>,
    script_name: String,
    load_env: bool,
    executor: &dyn Executor,
) -> Result<()> {
    let owned_path;
    let path = match package_json_path {
        None => {
            let p = resolve_package_json_path()?;
            owned_path = p;
            Path::new(&owned_path)
        }
        Some(p) => {
            owned_path = p;
            Path::new(&owned_path)
        }
    };
    let map = load_package_json(path)?;
    let scripts = load_scripts(map)?;
    let command = get_script_command(scripts, script_name)?;
    executor.execute(&command, vec![], load_env)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::{CommandExecutor, DryRunExecutor};

    use super::*;

    use serial_test::serial;

    struct MockCommandExecutor;

    impl MockCommandExecutor {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Executor for MockCommandExecutor {
        fn execute(
            &self,
            main_command: &str,
            args: Vec<&str>,
            _load_env: bool,
        ) -> anyhow::Result<()> {
            let full_command = main_command.to_owned() + " " + &args.join(" ");
            std::fs::write("package.mock", full_command)?;
            Ok(())
        }
    }

    #[test]
    #[serial]
    fn test_resolve_package_json_current_dir() {
        let current_dir =
            env::current_dir().expect("Should be able to resolve the current directory");
        let resolved_path =
            resolve_package_json_path().expect("Should be able to resolve jakefile path");
        assert_eq!(
            resolved_path,
            current_dir.join(PACKAGE_JSON).to_string_lossy().to_string()
        );
    }

    #[test]
    #[serial]
    fn test_resolve_package_json_subdir() {
        let parent_dir =
            env::current_dir().expect("Should be able to resolve the current directory");
        let new_dir = parent_dir.join(".github/workflows");
        env::set_current_dir(new_dir).expect("Should be able to set a new current directory");
        let cur_dir = env::current_dir().expect("Should be able to get current directory");
        assert_eq!(cur_dir, parent_dir.join(".github/workflows"));
        let resolved_path =
            resolve_package_json_path().expect("Should be able to resolve jakefile path");
        assert_eq!(
            resolved_path,
            parent_dir.join(PACKAGE_JSON).to_string_lossy().to_string()
        );
        env::set_current_dir(parent_dir)
            .expect("Should be able to set the current directory back to the original one");
    }

    #[test]
    #[serial]
    fn test_load_package_json() {
        let path = Path::new("testfiles/test-package.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("type"));
        assert!(map.contains_key("scripts"));
    }

    #[test]
    #[serial]
    fn test_load_package_json_array() {
        let path = Path::new("testfiles/json-array.json");
        let result = load_package_json(&path);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_load_scripts_success() {
        let path = Path::new("testfiles/test-package.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let scripts = load_scripts(map).expect("Should be able to load scripts");
        assert_eq!(scripts.len(), 2);
        assert!(scripts.contains_key("hello"));
        assert!(scripts.contains_key("test"));
    }

    #[test]
    #[serial]
    fn test_load_scripts_no_scripts() {
        let path = Path::new("testfiles/not-scripts.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let result = load_scripts(map);
        assert_eq!(
            result
                .is_err_and(|e| e.to_string()
                    == "Cannot find the `scripts` key in package.json".to_string()),
            true
        );
    }

    #[test]
    #[serial]
    fn test_load_scripts_not_json_map() {
        let path = Path::new("testfiles/not-json-map.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let result = load_scripts(map);
        assert_eq!(
            result.is_err_and(|e| e.to_string() == "`scripts` is not a JSON map.".to_string()),
            true
        );
    }

    #[test]
    #[serial]
    fn test_load_scripts_invalid_script() {
        let path = Path::new("testfiles/invalid-script.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let result = load_scripts(map);
        assert_eq!(
            result.is_err_and(|e| e.to_string() == "Encountered a non-string value.".to_string()),
            true
        );
    }

    #[test]
    #[serial]
    fn test_get_script_command_success() {
        let path = Path::new("testfiles/test-package.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let scripts = load_scripts(map).expect("Should be able to load scripts");
        assert_eq!(scripts.len(), 2);
        let command = get_script_command(scripts.clone(), "test".to_string())
            .expect("Should be able to load command");
        assert_eq!(command, "true".to_string());
        let command_1 = get_script_command(scripts, "hello".to_string())
            .expect("Should be able to load command");
        assert_eq!(command_1, "echo 'hello'".to_string());
    }

    #[test]
    #[serial]
    fn test_get_script_command_failure() {
        let path = Path::new("testfiles/test-package.json");
        let map = load_package_json(&path).expect("Should be able to load the file");
        let scripts = load_scripts(map).expect("Should be able to load scripts");
        let result = get_script_command(scripts, "bye".to_string());
        assert_eq!(
            result.is_err_and(|e| e.to_string() == "Could not find script bye".to_string()),
            true
        );
    }

    #[test]
    #[serial]
    fn test_mock_command_execution() {
        let executor = MockCommandExecutor::new();
        let path = Some("testfiles/test-package.json".to_string());
        let result = execute_script(path, "test".to_string(), false, &executor);
        assert!(result.is_ok());
        let content = fs::read_to_string("package.mock").expect("Should be able to read file");
        assert_eq!(content.trim(), "true");
    }

    #[test]
    #[serial]
    fn test_command_execution() {
        let executor = CommandExecutor::new();
        let path = Some("testfiles/test-package.json".to_string());
        let result = execute_script(path, "test".to_string(), false, &executor);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_dry_run_command_execution() {
        let executor = DryRunExecutor::new();
        let path = Some("testfiles/test-package.json".to_string());
        let result = execute_script(path, "test".to_string(), false, &executor);
        assert!(result.is_ok());
    }
}
