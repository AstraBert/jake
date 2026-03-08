use anyhow::Result;

const JAKEFILE_PATH: &str = "jakefile.toml";

fn parse_tasks(tasks: &str) -> String {
    let split_off: Vec<&str> = tasks.split(",").collect();
    let mut content = String::new();
    for split in split_off {
        let content_piece = format!("{} = \"echo 'No task yet for {}'\"\n", split, split);
        content += &content_piece;
    }

    content
}

pub fn write_jakefile(tasks: &str, path: Option<String>) -> Result<()> {
    let owned_path;
    let jakefile_path = match path {
        None => JAKEFILE_PATH,
        Some(p) => {
            owned_path = p;
            &owned_path
        }
    };
    let file_content = parse_tasks(tasks);
    std::fs::write(jakefile_path, file_content)?;
    println!("\x1b[1;92m{} successfully written", jakefile_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    use serial_test::serial;

    fn cleanup_file(fl: &str) {
        let path = Path::new(fl);
        if path.exists() {
            fs::remove_file(path).expect("Should be able to remove file");
        }
    }

    #[test]
    fn test_parse_tasks() {
        let expected =
            "hello = \"echo 'No task yet for hello'\"\nbye = \"echo 'No task yet for bye'\"\n"
                .to_string();
        let actual = parse_tasks("hello,bye");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_tasks_no_commas() {
        let expected =
            "hello:there:ciao = \"echo 'No task yet for hello:there:ciao'\"\n".to_string();
        let actual = parse_tasks("hello:there:ciao");
        assert_eq!(expected, actual);
    }

    #[test]
    #[serial]
    fn test_write_jakefile() {
        let file_path = "testfiles/temp.toml".to_string();
        write_jakefile("hello,bye", Some(file_path.clone())).expect("Should be able to write file");
        let expected =
            "hello = \"echo 'No task yet for hello'\"\nbye = \"echo 'No task yet for bye'\"\n"
                .to_string();
        let actual = fs::read_to_string(&file_path).expect("Should be able to read file");
        assert_eq!(expected, actual);

        cleanup_file(&file_path);
    }
}
