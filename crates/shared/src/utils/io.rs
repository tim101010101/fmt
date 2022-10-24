use std::fs::{remove_file, OpenOptions};
use std::io::{Error, Read, Write};
use std::path::Path;

pub fn file_exists(path: &str) -> bool {
    let p = Path::new(path);
    p.is_file() && p.exists()
}

pub fn dir_exists(path: &str) -> bool {
    let p = Path::new(path);
    let dir_name = p.file_name().unwrap();
    let expect_dir_name = path.split("/").last().unwrap();
    p.is_dir() && p.exists() && dir_name == expect_dir_name
}

pub fn write_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;
    file.write(content.as_ref())?;
    Ok(())
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn delete_file(path: &str) -> Result<(), Error> {
    remove_file(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::path;
    use crate::utils::io::{delete_file, dir_exists, file_exists};
    use crate::utils::{read_file, write_file};

    fn get_path() -> String {
        path!("src/tests", "test_io.txt")
    }

    #[test]
    fn test_exsit() {
        let valid_file_path = path!("src/tests/hello.txt");
        let invalid_file_path = path!("src/tests/not_exsit_file.fff");
        assert!(file_exists(&valid_file_path));
        assert!(!file_exists(&invalid_file_path));

        let valid_dir_path = path!("src/tests");
        let invalid_dir_path = path!("src/tests/hello");
        assert!(dir_exists(&valid_dir_path));
        assert!(!dir_exists(&invalid_dir_path));
    }

    #[test]
    fn smoke_test() {
        let p = get_path();
        if file_exists(&p) {
            delete_file(&p).unwrap();
        }

        let content = "Hello World";
        write_file(&p, content).unwrap();
        if let Ok(text) = read_file(&p) {
            assert_eq!(text, content);
        }
        delete_file(&p).unwrap();
    }
}
