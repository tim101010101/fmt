use crate::parser::Node;
use crate::rules::Rules;
use crate::traversal::Visitor;

#[derive(Clone)]
pub(crate) enum WhitespaceSurround {
    Left,
    Right,
    Both,
    None,
}

pub(crate) struct Generator {
    output: String,
    indent_level: usize,
    width: usize,
    rules: Rules,
}
impl Generator {
    pub(crate) fn new() -> Self {
        Generator {
            output: String::new(),
            indent_level: 0,
            width: 0,
            rules: Rules::new(),
        }
    }
    pub(crate) fn gen(&mut self, root: &Node) -> &str {
        self.visit(root);
        self.output.as_str()
    }
}

impl Generator {
    pub(crate) fn append(&mut self, str: &str) {
        self.width += str.len();
        self.output.push_str(str);
    }
    pub(crate) fn append_token(&mut self, token: &str, ws_surround: WhitespaceSurround) {
        match ws_surround {
            WhitespaceSurround::Left => {
                self.ws();
                self.append(token);
            }
            WhitespaceSurround::Right => {
                self.append(token);
                self.ws()
            }
            WhitespaceSurround::Both => {
                self.ws();
                self.append(token);
                self.ws();
            }
            WhitespaceSurround::None => {
                self.append(token);
            }
        }
    }
    pub(crate) fn push_list<'input, I>(
        &mut self,
        mut iter: I,
        sep: &str,
        ws_surround: WhitespaceSurround,
    ) where
        I: Iterator<Item = &'input Box<Node>>,
    {
        if let Some(item) = iter.next() {
            self.visit(item);

            while let Some(item) = iter.next() {
                self.append_token(sep, ws_surround.clone());
                self.visit(item);
            }
        }
    }
    pub(crate) fn push_block<'input, I>(&mut self, mut iter: I)
    where
        I: Iterator<Item = &'input Box<Node>>,
    {
        self.append("{");
        self.tab();
        self.newline();
        if let Some(item) = iter.next() {
            self.visit(item);

            while let Some(item) = iter.next() {
                self.newline();
                self.visit(item);
            }
        }
        self.backspace();
        self.newline();
        self.append("}");
    }
    pub(crate) fn ws(&mut self) {
        self.append(" ");
    }
    pub(crate) fn newline(&mut self) {
        self.width = 0;
        self.append(&self.rules().end_of_line());
        self.indent();
    }
    pub(crate) fn tab(&mut self) {
        self.indent_level += 1;
    }
    pub(crate) fn backspace(&mut self) {
        self.indent_level -= 1;
    }
    fn indent(&mut self) {
        let indent = self.rules().indent_kind().repeat(self.indent_level);
        self.append(&indent);
    }
}

impl Generator {
    pub(crate) fn rules(&self) -> &Rules {
        &self.rules
    }
}
