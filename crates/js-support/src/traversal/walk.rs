use crate::parser::Node;

struct Walker {
    root: Node,
}

impl Walker {
    pub(crate) fn new(root: Node) -> Self {
        Walker { root }
    }
}
