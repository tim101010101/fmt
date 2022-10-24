use std::fs::{remove_file, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};

pub fn exist(path: &str) -> Result<bool, Error> {
    if let Err(err) = OpenOptions::new().read(true).open(path) {
        match err.kind() {
            ErrorKind::AlreadyExists => Ok(true),
            ErrorKind::NotFound => Ok(false),
            ErrorKind::InvalidInput => Ok(false),
            _ => Err(err),
        }
    } else {
        Ok(true)
    }
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
    use crate::utils::io::{delete_file, exist};
    use crate::utils::{read_file, write_file};

    fn get_path() -> String {
        path!("src/tests", "test_io.txt")
    }

    #[test]
    fn test_exsit() {
        let valid_path = path!("src/tests/hello.txt");
        let invalid_path = path!("src/tests/not_exsit_file.fff");
        assert_eq!(exist(&valid_path).unwrap(), true);
        assert_eq!(exist(&invalid_path).unwrap(), false);
    }
    //
    // #[test]
    // fn test_delete_file() {
    //     let p = get_path();
    //     if let Ok(false) = exist(&p) {
    //         let content = "Hello World";
    //         write_file(&p, content).unwrap()
    //     }
    //     delete_file(&p).unwrap();
    // }
    //
    // #[test]
    // fn test_write_and_read() {
    //     let p = get_path();
    //     let content = "Hello World";
    //     write_file(&p, content).unwrap();
    //     if let Ok(text) = read_file(&p) {
    //         assert_eq!(text, content);
    //     }
    //     delete_file(&p).unwrap();
    // }

    #[test]
    fn smoke() {
        let p = get_path();
        if let Ok(true) = exist(&p) {
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
