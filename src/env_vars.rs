use std::{collections::HashMap, env};

use anyhow::{Result, anyhow};
use dotenv::{from_path, vars};

const DOTENV_PATH: &str = ".env";

fn resolve_dotenv_path() -> Result<String> {
    let current_dir = env::current_dir()?;
    let dotenv_path = current_dir.join(DOTENV_PATH);
    if dotenv_path.exists() {
        return Ok(dotenv_path.to_string_lossy().to_string());
    }
    for ancestor in current_dir.ancestors() {
        let dotenv = ancestor.join(DOTENV_PATH);
        if dotenv.exists() {
            return Ok(dotenv.to_string_lossy().to_string());
        }
    }

    Err(anyhow!(
        "Could not find a .env file in the current directory or in any of its ancestors"
    ))
}

pub fn dotenv_to_hashmap() -> Result<HashMap<String, String>> {
    let dotenv_path = resolve_dotenv_path()?;
    from_path(dotenv_path)?;
    let result: Vec<(String, String)> = vars().collect();
    let mut dotenv_hash: HashMap<String, String> = HashMap::new();
    for r in result {
        dotenv_hash.insert(r.0, r.1);
    }
    Ok(dotenv_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_resolve_dotenv_path_current_dir() {
        let current_dir =
            env::current_dir().expect("Should be able to resolve the current directory");
        let resolved_path = resolve_dotenv_path().expect("Should be able to resolve jakefile path");
        assert_eq!(
            resolved_path,
            current_dir.join(DOTENV_PATH).to_string_lossy().to_string()
        );
    }

    #[test]
    #[serial]
    fn test_resolve_dotenv_path_subdir() {
        let parent_dir =
            env::current_dir().expect("Should be able to resolve the current directory");
        let new_dir = parent_dir.join(".github/workflows");
        env::set_current_dir(new_dir).expect("Should be able to set a new current directory");
        let cur_dir = env::current_dir().expect("Should be able to get current directory");
        assert_eq!(cur_dir, parent_dir.join(".github/workflows"));
        let resolved_path = resolve_dotenv_path().expect("Should be able to resolve jakefile path");
        assert_eq!(
            resolved_path,
            parent_dir.join(DOTENV_PATH).to_string_lossy().to_string()
        );
        env::set_current_dir(parent_dir)
            .expect("Should be able to set the current directory back to the original one");
    }

    #[test]
    #[serial]
    fn test_dotenv_to_hashmap() {
        let dotenv_hashmap =
            dotenv_to_hashmap().expect("Should be able to load dotenv to a hashmap");
        assert!(dotenv_hashmap.len() >= 3);
        assert!(dotenv_hashmap.contains_key("TEST_VAR"));
        assert!(dotenv_hashmap.contains_key("ANOTHER_TEST_VAR"));
        assert!(dotenv_hashmap.contains_key("HELLO"));
        if let Some(val) = dotenv_hashmap.get("TEST_VAR") {
            assert_eq!(*val, "this is a test".to_string())
        }
        if let Some(val1) = dotenv_hashmap.get("ANOTHER_TEST_VAR") {
            assert_eq!(*val1, "another test".to_string())
        }
        if let Some(val2) = dotenv_hashmap.get("HELLO") {
            assert_eq!(*val2, "hello".to_string())
        }
    }
}
