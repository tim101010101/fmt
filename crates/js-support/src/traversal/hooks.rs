pub(crate) fn before_visit() {
    todo!()
}

pub(crate) fn after_visit() {
    todo!()
}

struct IdLogger;

impl IdLogger {
    pub(crate) fn new() -> Self {
        IdLogger
    }

    pub(crate) fn visit_id(&self, name: &str) {
        println!("{name}");
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{lex, syntax};
    use crate::traversal::hooks::IdLogger;

    #[test]
    fn smoke() {
        let ast = syntax(lex("const a = b")).unwrap();
        let id_logger = IdLogger::new();

        id_logger.visit_id("");

        // TODO need a struct Walker to terversal the AST and register the visitor
    }
}
