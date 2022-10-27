use regex::Regex;

pub fn resolve(base: &str, path_list: Vec<&str>) -> String {
    struct PathCache<'a> {
        stack: Vec<&'a str>,
    }
    impl<'a> PathCache<'a> {
        fn new() -> Self {
            PathCache { stack: Vec::new() }
        }
        fn join(&mut self, path: &'a str) {
            let reg = Regex::new(r#"^(\.)?[a-zA-Z0-9_]+(\.[a-zA-Z0-9_]+)*$"#).unwrap();
            path.split("/").for_each(|s| match s {
                ".." => {
                    self.stack.pop();
                }
                _ if reg.is_match(s) => {
                    self.stack.push(s);
                }
                _ => {}
            })
        }
        fn canonicalize(&self) -> String {
            format!("/{}", self.stack.join("/"))
        }
    }

    let mut pr = PathCache::new();
    pr.join(base);
    path_list.iter().for_each(|p| pr.join(p));
    pr.canonicalize()
}

#[macro_export]
macro_rules! path {
( $( $x:expr),*) => {
  {
      let _dir = std::env::current_dir().unwrap();
      let str = _dir.to_str().unwrap();
      let mut path_list = Vec::new();
      $(
        path_list.push($x);
      )*
      crate::utils::path_resolve::resolve(str, path_list)
  }
};
}

#[cfg(test)]
mod tests {
    use crate::utils::resolve;
    use std::env;
    use std::path::PathBuf;

    fn init() -> String {
        let __dir = env::current_dir().unwrap();
        let str = __dir.to_str().unwrap();
        str.to_string()
    }

    fn get_benchmark() -> PathBuf {
        let mut p = env::current_dir().unwrap();
        p.push("src");
        p.push("tests");
        p.push("hello.txt");
        p.canonicalize().unwrap()
    }

    #[test]
    fn basic_test() {
        let base = init();
        let p = resolve(&base, vec![]);
        assert_eq!(p, base);
    }

    #[test]
    fn can_join() {
        let base = init();
        let p = resolve(&base, vec!["src", "tests", "hello.txt"]);
        let benchmark = get_benchmark();
        assert_eq!(p, benchmark.to_str().unwrap());
    }

    #[test]
    fn can_resolve() {
        let base = init();
        let p = resolve(
            &base,
            vec!["./src", "./tests", "..", "tests", "./hello.txt"],
        );
        let benchmark = get_benchmark();
        assert_eq!(p, benchmark.to_str().unwrap());
    }

    #[test]
    fn test_macro() {
        let benchmark = get_benchmark();
        assert_eq!(
            path!("./src", "./tests", "..", "tests", "./hello.txt"),
            benchmark.to_str().unwrap()
        )
    }
}
