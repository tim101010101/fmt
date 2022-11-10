pub(crate) struct Rules {
    semi: bool,
    width: usize,
    indent_kind: String,
    end_of_line: String,
    single_quote: bool,
}

impl Rules {
    pub(crate) fn new() -> Self {
        Rules {
            semi: true,
            width: 80,
            indent_kind: String::from("  "),
            end_of_line: String::from("\n"),
            single_quote: true,
        }
    }

    pub(crate) fn semi(&self) -> bool {
        self.semi
    }
    pub(crate) fn set_semi(&mut self, semi: bool) {
        self.semi = semi;
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }
    pub(crate) fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub(crate) fn indent_kind(&self) -> String {
        self.indent_kind.to_string()
    }
    pub(crate) fn set_indent_kind(&mut self, indent_kind: String) {
        self.indent_kind = indent_kind;
    }

    pub(crate) fn end_of_line(&self) -> String {
        self.end_of_line.to_string()
    }
    pub(crate) fn set_end_of_line(&mut self, end_of_line: String) {
        self.end_of_line = end_of_line;
    }

    pub(crate) fn single_quote(&self) -> bool {
        self.single_quote
    }
    pub(crate) fn set_single_quote(&mut self, single_quote: bool) {
        self.single_quote = single_quote;
    }
}
